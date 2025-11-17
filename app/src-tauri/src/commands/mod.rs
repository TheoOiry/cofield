use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use cofield_receiver::{FlexSensorGloveNotification, MeanAggregator, MovingFingers, Opt, Process, TextPattern, flex_sensor_glove::FlexSensorGlove};
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};
use tokio::{sync::Mutex, task::JoinHandle};

struct GloveProcess {
    process: JoinHandle<()>,
    text_patterns: Arc<Mutex<Option<TextPattern>>>,
    aggregator: Arc<Mutex<Option<MeanAggregator>>>,
    raw_output_writer: Arc<Mutex<Option<csv::Writer<std::fs::File>>>>,
}

pub struct ProcessHandle {
    process: Mutex<Option<GloveProcess>>,
}

pub struct ProcessConfig {
    aggregation_size: Mutex<usize>,
    use_keyboard_emulation: Mutex<bool>,
}

impl ProcessHandle {
    pub fn new() -> Self {
        Self {
            process: Mutex::new(None),
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct NotificationPayload {
    notification: FlexSensorGloveNotification,
    moved_fingers: MovingFingers,
}

impl ProcessConfig {
    pub fn new() -> Self {
        Self {
            aggregation_size: Opt::default().aggregation_size.into(),
            use_keyboard_emulation: true.into(),
        }
    }
}

#[tauri::command]
pub async fn start_listening_glove(
    app: AppHandle,
    process_handle: State<'_, ProcessHandle>,
    process_config: State<'_, ProcessConfig>,
) -> Result<(), String> {
    if (*process_handle.process.lock().await).is_some() {
        return Ok(());
    }

    let mut opt = Opt::default();
    opt.verbose = true;
    opt.aggregation_size = *process_config.aggregation_size.lock().await;

    let app_text = app.clone();
    let mut text_patterns = TextPattern::new(Box::new(move |str| {
        app_text.emit("new_character", str).ok();
    }));

    let use_keyboard_emulation = *process_config.use_keyboard_emulation.lock().await;
    text_patterns.use_keyboard_emulation(use_keyboard_emulation);

    let aggregator = Arc::new(Mutex::new(Some(MeanAggregator::new(opt.aggregation_size))));
    let text_patterns = Arc::new(Mutex::new(Some(text_patterns)));
    let raw_output_writer = Arc::new(Mutex::new(None::<csv::Writer<std::fs::File>>));

    let process_aggregator = aggregator.clone();
    let process_text_patterns = text_patterns.clone();
    let process_raw_output_writer = raw_output_writer.clone();

    let handle = tokio::spawn(async move {
        let flex_sensor_glove = FlexSensorGlove::new(&opt)
            .await
            .map_err(|err| err.to_string())
            .unwrap();

        let notification_stream = Box::pin(
            flex_sensor_glove
                .get_notifications_stream()
                .await
                .map_err(|e| e.to_string())
                .unwrap(),
        );

        app.emit("glove_connected", ()).unwrap();

        let mut process = Process::new(notification_stream, opt.fingers_sensibility).await;

        process.set_aggregator(process_aggregator);
        process.set_text_pattern_detection(process_text_patterns);
        process.set_raw_output_writer(process_raw_output_writer);
        process.on_notification(move |notification, moved_fingers| { 
            app.emit("glove_notification", NotificationPayload {
            notification: notification.clone(), 
            moved_fingers
        }).ok();});

        process.run().await.expect("An error occured while running process");
    });

    *process_handle.process.lock().await = Some(GloveProcess {
        process: handle,
        text_patterns,
        aggregator,
        raw_output_writer,
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_listening_glove(
    app: AppHandle,
    process_handle: State<'_, ProcessHandle>,
) -> Result<(), String> {
    let mut process = process_handle.process.lock().await;
    let Some(glove_process) = process.take() else {
        return Ok(());
    };

    glove_process.process.abort();

    app.emit("glove_disconnected", ()).ok();

    Ok(())
}

#[tauri::command]
pub async fn set_aggregation_size(
    process_handle: State<'_, ProcessHandle>,
    process_config: State<'_, ProcessConfig>,
    aggregation_size: usize,
) -> Result<(), String> {
    if aggregation_size == 0 {
        return Err("Aggregation size must be greater than 0".to_string());
    }

    if aggregation_size == *process_config.aggregation_size.lock().await {
        return Ok(());
    }
    *process_config.aggregation_size.lock().await = aggregation_size;

    let mut process = process_handle.process.lock().await;
    let Some(glove_process) = process.as_mut() else {
        return Ok(());
    };

    glove_process
        .aggregator
        .lock()
        .await
        .as_mut()
        .map(|aggregator| aggregator.set_aggregation_size(aggregation_size));

    Ok(())
}

#[tauri::command]
pub async fn set_keyboard_emulation_config(
    process_handle: State<'_, ProcessHandle>,
    process_config: State<'_, ProcessConfig>,
    is_enabled: bool,
) -> Result<(), String> {
    if is_enabled == *process_config.use_keyboard_emulation.lock().await {
        return Ok(());
    }

    *process_config.use_keyboard_emulation.lock().await = is_enabled;

    let mut process = process_handle.process.lock().await;
    let Some(glove_process) = process.as_mut() else {
        return Ok(());
    };

    glove_process
        .text_patterns
        .lock()
        .await
        .as_mut()
        .map(|text_patterns| text_patterns.use_keyboard_emulation(is_enabled));

    Ok(())
}

#[tauri::command]
pub async fn set_output_raw_data(
    process_handle: State<'_, ProcessHandle>,
    folder_path: Option<String>,
) -> Result<Option<PathBuf>, String> {
    let mut process = process_handle.process.lock().await;
    let Some(glove_process) = process.as_mut() else {
        return Ok(None);
    };

    let file_path = folder_path.map(|path| {
        let timestamp = chrono::Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();
        let file_name = format!("raw_{timestamp}.csv");

        Path::new(&path).join(file_name)
    });

    let writer = match &file_path {
        Some(file_path) => {
            let writer = csv::WriterBuilder::new()
                .has_headers(false)
                .from_path(file_path)
                .map_err(|e| e.to_string())?;

            Some(writer)
        }
        None => None,
    };

    *glove_process.raw_output_writer.lock().await = writer;

    Ok(file_path)
}

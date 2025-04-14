use std::sync::Arc;

use cofield_receiver::{flex_sensor_glove::FlexSensorGlove, MeanAggregator, Opt, TextPattern};
use futures::StreamExt;
use tauri::{AppHandle, Emitter, State};
use tokio::{sync::Mutex, task::JoinHandle};

struct GloveProcess {
    process: JoinHandle<()>,
    text_patterns: Arc<Mutex<TextPattern>>,
    aggregator: Arc<Mutex<MeanAggregator>>,
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

    let aggregator = Arc::new(Mutex::new(MeanAggregator::new(opt.aggregation_size)));
    let text_patterns = Arc::new(Mutex::new(text_patterns));

    let process_aggregator = aggregator.clone();
    let process_text_patterns = text_patterns.clone();

    let handle = tokio::spawn(async move {
        let flex_sensor_glove = FlexSensorGlove::new(&opt)
            .await
            .map_err(|err| err.to_string())
            .unwrap();

        let mut notification_stream = Box::pin(
            flex_sensor_glove
                .get_notifications_stream()
                .await
                .map_err(|e| e.to_string())
                .unwrap(),
        );

        app.emit("glove_connected", ()).unwrap();

        while let Some(notification) = notification_stream.next().await {
            let notification = process_aggregator
                .lock()
                .await
                .push_and_aggregate(notification);

            app.emit("glove_notification", notification.clone()).ok();

            let moved_fingers = notification
                .flex_values
                .detect_moved_fingers(&opt.fingers_sensibility);

            app.emit("moved_fingers", moved_fingers).ok();

            process_text_patterns
                .lock()
                .await
                .process_moved_fingers(&moved_fingers, notification.dt);
        }
    });

    *process_handle.process.lock().await = Some(GloveProcess {
        process: handle,
        text_patterns,
        aggregator,
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
        .set_aggregation_size(aggregation_size);

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
        .use_keyboard_emulation(is_enabled);

    Ok(())
}

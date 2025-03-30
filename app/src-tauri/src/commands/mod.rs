use std::sync::Arc;

use cofield_receiver::{
    flex_sensor_glove::FlexSensorGlove, MeanAggregator, Opt, TextPattern,
};
use futures::StreamExt;
use tauri::{AppHandle, Emitter, State};
use tokio::{sync::Mutex, task::JoinHandle};


struct GloveProcess {
    process: JoinHandle<()>,
    text_patterns: Arc<Mutex<TextPattern>>,
    aggregator: Arc<Mutex<MeanAggregator>>,
}

pub struct ProcessHandle {
    process: Mutex<Option<GloveProcess>>
}

impl ProcessHandle {
    pub fn new() -> Self {
        Self {
            process: Mutex::new(None)
        }
    }
}

#[tauri::command]
pub async fn start_listening_glove(
    app: AppHandle,
    process_handle: State<'_, ProcessHandle>,
) -> Result<(), String> {
    if let Some(_) = &*process_handle.process.lock().await {
        return Ok(());
    }

    let opt = Opt::default();

    let app_text = app.clone();
    let text_patterns = TextPattern::new(Box::new(move |str| {
        app_text
            .emit("new_character", str)
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .ok();
    }));

    let aggregator = Arc::new(Mutex::new(MeanAggregator::new(opt.aggregation_size)));
    let text_patterns = Arc::new(Mutex::new(text_patterns));

    let process_aggregator = aggregator.clone();
    let process_text_patterns = text_patterns.clone();

    let handle = tokio::spawn(async move {
        let flex_sensor_glove = FlexSensorGlove::new(&opt)
            .await
            .map_err(|err| err.to_string()).unwrap();

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

            app.emit("glove_notification", notification.clone())
                .map_err(|e| anyhow::anyhow!(e.to_string()))
                .ok();

            let moved_fingers = notification
                .flex_values
                .detect_moved_fingers(&opt.fingers_sensibility);

            app.emit("moved_fingers", moved_fingers)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
                .ok();

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

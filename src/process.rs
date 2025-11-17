use std::sync::Arc;

use futures::StreamExt;

#[cfg(feature = "lsl")]
use lsl::Pushable;
use tokio::sync::Mutex;

use crate::{
    FlexSensorGloveNotification, MovingFingers, OutputWriterDyn, TextPattern, aggregator::MeanAggregator, opt::FingersSensibility, output::OutputRow
};

pub struct Process<'a> {
    fingers_sensibility: FingersSensibility,

    notification_stream: futures::stream::BoxStream<'a, crate::parser::FlexSensorGloveNotification>,

    aggregator: Arc<Mutex<Option<MeanAggregator>>>,
    output_writer: Arc<Mutex<Option<OutputWriterDyn>>>,
    raw_output_writer: Arc<Mutex<Option<csv::Writer<std::fs::File>>>>,
    text_pattern_detection: Arc<Mutex<Option<TextPattern>>>,

    on_notification: Option<Box<dyn FnMut(&FlexSensorGloveNotification, MovingFingers) + Send + Sync>>,

    #[cfg(feature = "lsl")]
    lsl_stream_outlet: Option<lsl::StreamOutlet>,

}

impl<'a> Process<'a> {
    pub async fn new(
        notification_stream: futures::stream::BoxStream<
            'a,
            crate::parser::FlexSensorGloveNotification,
        >,
        fingers_sensibility: FingersSensibility,
    ) -> Self {
        Self {
            fingers_sensibility,
            notification_stream: notification_stream.boxed(),

            aggregator: Arc::new(Mutex::new(None)),
            output_writer: Arc::new(Mutex::new(None)),
            raw_output_writer: Arc::new(Mutex::new(None)),
            text_pattern_detection: Arc::new(Mutex::new(None)),

            on_notification: None,

            #[cfg(feature = "lsl")]
            lsl_stream_outlet: None,
        }
    }

    pub fn set_output_writer(&mut self, output_writer: Arc<Mutex<Option<OutputWriterDyn>>>) {
        self.output_writer = output_writer;
    }

    pub fn set_raw_output_writer(&mut self, raw_output_writer: Arc<Mutex<Option<csv::Writer<std::fs::File>>>>) {
        self.raw_output_writer = raw_output_writer;
    }

    pub fn set_aggregator(&mut self, aggregator: Arc<Mutex<Option<MeanAggregator>>>) {
        self.aggregator = aggregator;
    }

    pub fn set_text_pattern_detection(&mut self, text_pattern_detection: Arc<Mutex<Option<TextPattern>>>) {
        self.text_pattern_detection = text_pattern_detection;
    }

    pub fn on_notification(&mut self, closure: impl FnMut(&FlexSensorGloveNotification, MovingFingers) + Send + Sync + 'static) {
        self.on_notification = Some(Box::new(closure))
    }

    #[cfg(feature = "lsl")]
    pub fn set_lsl_stream_outlet(&mut self, lsl_stream_outlet: lsl::StreamOutlet) {
        self.lsl_stream_outlet = Some(lsl_stream_outlet);
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        while let Some(notification) = self.notification_stream.next().await {
            if let Some(raw_data_writer) = self.raw_output_writer.lock().await.as_mut() {
                raw_data_writer.serialize(&notification)?;
                raw_data_writer.flush()?;
            }

            let aggregated_notification = if let Some(aggregator) = self.aggregator.lock().await.as_mut() {
                aggregator.push_and_aggregate(notification)
            } else {
                notification
            };

            let moved_fingers = aggregated_notification
                .flex_values
                .detect_moved_fingers(&self.fingers_sensibility);

            if let Some(on_notification) = self.on_notification.as_mut() {
                on_notification(&aggregated_notification, moved_fingers)
            }

            let output_row = OutputRow {
                notification: &aggregated_notification,
                moving_fingers: moved_fingers.map(|f| f as u32 * 500),
            };

            if let Some(output_writer) = self.output_writer.lock().await.as_mut() {
                output_writer.write_row(&output_row)?;
            }

            if let Some(text_pattern) = self.text_pattern_detection.lock().await.as_mut() {
                text_pattern.process_moved_fingers(&moved_fingers, aggregated_notification.dt);
            }

            #[cfg(feature = "lsl")]
            if let Some(lsl_stream_outlet) = &self.lsl_stream_outlet {
                lsl_stream_outlet.push_sample(&output_row)?;
            }
        }

        Ok(())
    }
}

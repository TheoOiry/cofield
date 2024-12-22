use futures::StreamExt;
use lsl::Pushable;

use crate::{aggregator::MeanAggregator, devices::{FlexSensorGlove, VibrationGlove}};

pub struct Process<'a, 'b> {
    vibration_glove: Option<&'b mut VibrationGlove>,
    
    notification_stream: futures::stream::BoxStream<'a, crate::parser::FlexSensorGloveNotification>,

    aggregator: MeanAggregator,
    output_writer: Option<Box<dyn crate::output::OutputWriter>>,

    lsl_stream_outlet: Option<lsl::StreamOutlet>,
}

impl<'a, 'b> Process<'a, 'b> {
    pub async fn new(flex_sensor_glove: &'a FlexSensorGlove, aggregation_size: usize) -> anyhow::Result<Self> {

        let mut notification_stream = flex_sensor_glove.get_notifications_stream().await?;

        let init_data = notification_stream.by_ref().take(aggregation_size).collect().await;
        let aggregator = MeanAggregator::new(init_data);

        Ok(Self {
            notification_stream: notification_stream.boxed(),
            vibration_glove: None,
            aggregator,
            output_writer: None,
            lsl_stream_outlet: None,
        })
    }

    pub fn set_vibration_glove(&mut self, vibration_glove: &'b mut VibrationGlove) {
        self.vibration_glove = Some(vibration_glove);
    }

    pub fn set_output_writer(&mut self, output_writer: Box<dyn crate::output::OutputWriter>) {
        self.output_writer = Some(output_writer);
    }

    pub fn set_lsl_stream_outlet(&mut self, lsl_stream_outlet: lsl::StreamOutlet) {
        self.lsl_stream_outlet = Some(lsl_stream_outlet);
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        while let Some(notification) = self.notification_stream.next().await {
            let aggregated_notification = self.aggregator.push_and_aggregate(notification.clone());
            
            if let Some(vibration_glove) = &mut self.vibration_glove {
                vibration_glove.process_flex_values(&aggregated_notification.flex_values).await?;
            }

            if let Some(output_writer) = &mut self.output_writer {
                output_writer.write_row(&aggregated_notification)?;
            }

            if let Some(lsl_stream_outlet) = &self.lsl_stream_outlet {
                lsl_stream_outlet.push_sample(&aggregated_notification)?;
            }
        }

        Ok(())
    }
}

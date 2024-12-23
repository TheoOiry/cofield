use futures::StreamExt;
use lsl::Pushable;

use crate::{
    aggregator::MeanAggregator,
    devices::{FlexSensorGlove, RandomVibrationModeConfig, VibrationGlove, VibrationMode},
    opt::FingersSensibility,
    patterns::{
        Pattern, ReapeatingPattern, DEFAULT_PATTERN_MAX_DELAY, DEFAULT_REPEATING_PATTERN_DELAY,
        FINGERS_ORDER,
    },
};

pub struct Process<'a, 'b> {
    vibration_glove: Option<&'b mut VibrationGlove>,
    fingers_sensibility: FingersSensibility,

    notification_stream: futures::stream::BoxStream<'a, crate::parser::FlexSensorGloveNotification>,
    aggregator: MeanAggregator,

    output_writer: Option<Box<dyn crate::output::OutputWriter>>,
    lsl_stream_outlet: Option<lsl::StreamOutlet>,

    lucid_dream_detection_pattern: ReapeatingPattern,
    is_lucid_dreaming: bool,
}

impl<'a, 'b> Process<'a, 'b> {
    pub async fn new(
        flex_sensor_glove: &'a FlexSensorGlove,
        aggregation_size: usize,
        fingers_sensibility: FingersSensibility,
    ) -> anyhow::Result<Self> {
        let mut notification_stream = flex_sensor_glove.get_notifications_stream().await?;

        let init_data = notification_stream
            .by_ref()
            .take(aggregation_size)
            .collect()
            .await;
        let aggregator = MeanAggregator::new(init_data);

        let lucid_dream_detection_pattern =
            Pattern::new(FINGERS_ORDER.to_vec(), DEFAULT_PATTERN_MAX_DELAY);
        let lucid_dream_detection_pattern = ReapeatingPattern::new(
            lucid_dream_detection_pattern,
            DEFAULT_REPEATING_PATTERN_DELAY,
        );

        Ok(Self {
            fingers_sensibility,
            notification_stream: notification_stream.boxed(),
            aggregator,
            lucid_dream_detection_pattern,

            is_lucid_dreaming: false,
            vibration_glove: None,
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
            let moved_fingers = aggregated_notification
                .flex_values
                .detect_moved_fingers(&self.fingers_sensibility);

            if !self.is_lucid_dreaming {
                self.lucid_dream_detection_pattern
                    .process_moved_fingers(&moved_fingers, notification.dt);
                self.is_lucid_dreaming = self.lucid_dream_detection_pattern.nb_done >= 3;
            }

            if let Some(vibration_glove) = &mut self.vibration_glove {
                vibration_glove
                    .process_moved_fingers(&moved_fingers, notification.dt)
                    .await?;

                if self.is_lucid_dreaming
                    && !matches!(vibration_glove.vibration_mode, VibrationMode::Random(_))
                {
                    vibration_glove.set_vibration_mode(VibrationMode::Random(
                        RandomVibrationModeConfig::new(),
                    ));
                }
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

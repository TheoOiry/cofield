use futures::StreamExt;

#[cfg(feature = "lsl")]
use lsl::Pushable;

use crate::{
    aggregator::MeanAggregator,
    opt::FingersSensibility,
    output::OutputRow,
    patterns::{
        Pattern, ReapeatingPattern, DEFAULT_PATTERN_MAX_DELAY, DEFAULT_REPEATING_PATTERN_DELAY,
        FINGERS_ORDER,
    },
};

pub struct Process<'a> {
    fingers_sensibility: FingersSensibility,

    notification_stream: futures::stream::BoxStream<'a, crate::parser::FlexSensorGloveNotification>,
    aggregator: Option<MeanAggregator>,

    output_writer: Option<Box<dyn crate::output::OutputWriter + Send>>,

    #[cfg(feature = "lsl")]
    lsl_stream_outlet: Option<lsl::StreamOutlet>,

    raw_data_writer: Option<csv::Writer<std::fs::File>>,

    lucid_dream_detection_pattern: ReapeatingPattern,
    is_lucid_dreaming: bool,
}

impl<'a> Process<'a> {
    pub async fn new(
        notification_stream: futures::stream::BoxStream<
            'a,
            crate::parser::FlexSensorGloveNotification,
        >,
        aggregation_size: usize,
        fingers_sensibility: FingersSensibility,
    ) -> anyhow::Result<Self> {
        let aggregator = if aggregation_size > 0 {
            Some(MeanAggregator::new(aggregation_size))
        } else {
            None
        };

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
            output_writer: None,
            raw_data_writer: None,

            #[cfg(feature = "lsl")]
            lsl_stream_outlet: None,
        })
    }

    pub fn set_output_writer(
        &mut self,
        output_writer: Box<dyn crate::output::OutputWriter + Send>,
    ) {
        self.output_writer = Some(output_writer);
    }

    pub fn set_output_raw_data(
        &mut self,
        output_raw_data: Option<std::path::PathBuf>,
    ) -> anyhow::Result<()> {
        self.raw_data_writer = match output_raw_data {
            Some(output_raw_data) => Some(
                csv::WriterBuilder::new()
                    .has_headers(false)
                    .from_path(output_raw_data)?,
            ),
            None => None,
        };

        Ok(())
    }

    #[cfg(feature = "lsl")]
    pub fn set_lsl_stream_outlet(&mut self, lsl_stream_outlet: lsl::StreamOutlet) {
        self.lsl_stream_outlet = Some(lsl_stream_outlet);
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        while let Some(notification) = self.notification_stream.next().await {
            if let Some(raw_data_writer) = &mut self.raw_data_writer {
                raw_data_writer.serialize(&notification)?;
                raw_data_writer.flush()?;
            }

            let aggregated_notification = if let Some(aggregator) = &mut self.aggregator {
                aggregator.push_and_aggregate(notification)
            } else {
                notification
            };

            let moved_fingers = aggregated_notification
                .flex_values
                .detect_moved_fingers(&self.fingers_sensibility);

            if !self.is_lucid_dreaming {
                self.lucid_dream_detection_pattern
                    .process_moved_fingers(&moved_fingers, aggregated_notification.dt);
                self.is_lucid_dreaming = self.lucid_dream_detection_pattern.nb_done >= 3;
            }

            let output_row = OutputRow {
                notification: &aggregated_notification,
                moving_fingers: moved_fingers.map(|f| f as u32 * 500),
            };

            if let Some(output_writer) = &mut self.output_writer {
                output_writer.write_row(&output_row)?;
            }

            #[cfg(feature = "lsl")]
            if let Some(lsl_stream_outlet) = &self.lsl_stream_outlet {
                lsl_stream_outlet.push_sample(&output_row)?;
            }
        }

        Ok(())
    }
}

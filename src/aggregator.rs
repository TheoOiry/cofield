use crate::parser::{FingersFlexValues, FlexSensorGloveNotification};
use futures::stream::StreamExt;
pub struct MeanAggregator {
    rows: Vec<FlexSensorGloveNotification>,
}

impl MeanAggregator {
    pub fn new(rows: Vec<FlexSensorGloveNotification>) -> Self {
        Self { rows }
    }

    pub fn push_and_aggregate(
        &mut self,
        new_row: FlexSensorGloveNotification,
    ) -> FlexSensorGloveNotification {
        self.rows.remove(0);
        self.rows.push(new_row);

        self.aggregate_rows()
    }

    fn aggregate_rows(&self) -> FlexSensorGloveNotification {
        let len = self.rows.len() as u32;
        let last_row = &self.rows[self.rows.len() - 1];

        let mut flex_values: FingersFlexValues = self.rows.iter().map(|row| row.flex_values).sum();
        flex_values = last_row.flex_values - (flex_values / len);

        FlexSensorGloveNotification {
            dt: last_row.dt,
            flex_values,
        }
    }
}

pub async fn _mean_flex_values_by_size<S>(
    mut stream: S,
    aggregation_size: usize,
) -> impl futures::Stream<Item = FlexSensorGloveNotification>
where
    S: futures::Stream<Item = FlexSensorGloveNotification>,
    for<'a> &'a mut S: futures::Stream<Item = FlexSensorGloveNotification>,
{
    let init_data = stream.by_ref().take(aggregation_size).collect().await;

    let mut aggregator = MeanAggregator::new(init_data);

    stream.map(move |row| aggregator.push_and_aggregate(row))
}

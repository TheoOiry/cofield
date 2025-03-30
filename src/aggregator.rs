use crate::parser::{FingersFlexValues, FlexSensorGloveNotification};

pub struct MeanAggregator {
    rows: Vec<FlexSensorGloveNotification>,
    target_aggregation_size: usize,
}

impl MeanAggregator {
    pub fn new(target_aggregation_size: usize) -> Self {
        assert!(target_aggregation_size > 0);

        Self { rows: vec![], target_aggregation_size }
    }

    pub fn push_and_aggregate(
        &mut self,
        new_row: FlexSensorGloveNotification,
    ) -> FlexSensorGloveNotification {
        let len = self.rows.len() as usize;

        if len >= self.target_aggregation_size {
            self.rows.remove(0);
        }

        if len <= self.target_aggregation_size {
            self.rows.push(new_row);            
        }

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

use std::fmt::{self, Display, Formatter};

use chrono::{DateTime, Local, TimeDelta};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlexSensorGloveNotification {
    dt: DateTime<Local>,
    flex_values: [u16; 5],
}

impl FlexSensorGloveNotification {
    pub fn from_buffer(buffer: &[u8], dt_start: DateTime<Local>) -> Self {
        let mut flex_values = [0; 5];
        for i in 0..5 {
            flex_values[i] = u16::from_le_bytes([buffer[i * 2], buffer[i * 2 + 1]]);
        }

        let millis = u32::from_le_bytes([buffer[10], buffer[11], buffer[12], buffer[13]]);
        let millis = TimeDelta::milliseconds(millis as i64);

        FlexSensorGloveNotification {
            dt: dt_start + millis,
            flex_values,
        }
    }
}

impl Display for FlexSensorGloveNotification {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.dt, self.flex_values)
    }
}

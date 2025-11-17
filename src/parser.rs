use std::{
    fmt::{self, Display, Formatter},
    iter::Sum,
    ops::{Add, Div, Sub},
};

use chrono::{DateTime, Local, TimeDelta};
use serde::{Deserialize, Serialize};

use crate::opt::FingersSensibility;

pub type MovingFingers = [bool; 5];

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct FingersFlexValues(pub [u32; 5]);

impl FingersFlexValues {
    pub fn detect_moved_fingers(&self, sensibility: &FingersSensibility) -> MovingFingers {
        let mut moved_fingers = [false; 5];

        self.0.iter().enumerate().for_each(|(i, &value)| {
            moved_fingers[i] = value > sensibility.0[i];
        });

        moved_fingers
    }
}

impl Div<u32> for FingersFlexValues {
    type Output = Self;

    fn div(self, rhs: u32) -> Self {
        let mut result = FingersFlexValues([0; 5]);
        for i in 0..5 {
            result.0[i] = self.0[i] / rhs;
        }
        result
    }
}

impl Sub for FingersFlexValues {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = FingersFlexValues([0; 5]);
        for i in 0..5 {
            result.0[i] = self.0[i].saturating_sub(other.0[i]);
        }
        result
    }
}

impl Add for FingersFlexValues {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = FingersFlexValues([0; 5]);
        for i in 0..5 {
            result.0[i] = self.0[i] + other.0[i];
        }
        result
    }
}

impl Sum for FingersFlexValues {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = FingersFlexValues([0; 5]);
        for item in iter {
            sum = sum + item;
        }
        sum
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexSensorGloveNotification {
    pub dt: DateTime<Local>,
    pub flex_values: FingersFlexValues,
}

impl FlexSensorGloveNotification {
    pub fn from_buffer(buffer: &[u8], dt_start: DateTime<Local>) -> Self {
        let mut flex_values = [0; 5];
        for i in 0..5 {
            flex_values[i] = u16::from_le_bytes([buffer[i * 2], buffer[i * 2 + 1]]) as u32;
        }

        let millis = u32::from_le_bytes([buffer[10], buffer[11], buffer[12], buffer[13]]);
        let millis = TimeDelta::milliseconds(millis as i64);

        FlexSensorGloveNotification {
            dt: dt_start + millis,
            flex_values: FingersFlexValues(flex_values),
        }
    }
}

impl Display for FlexSensorGloveNotification {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.dt, self.flex_values)
    }
}

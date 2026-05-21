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
pub struct FingersFlexValues(pub [i32; 5]);

impl FingersFlexValues {
    pub fn detect_moved_fingers(&self, sensibility: &FingersSensibility) -> MovingFingers {
        let mut moved_fingers = [false; 5];

        self.0.iter().enumerate().for_each(|(i, &value)| {
            moved_fingers[i] = value.abs() > sensibility.0[i] as i32;
        });

        moved_fingers
    }
}

impl Div<i32> for FingersFlexValues {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImuReading {
    pub acc: [i32; 3],
    pub gyro: [i32; 3],
}

impl ImuReading {
    pub const fn zero() -> Self {
        Self {
            acc: [0; 3],
            gyro: [0; 3],
        }
    }
}

impl Add for ImuReading {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = ImuReading::zero();
        for i in 0..3 {
            result.acc[i] = self.acc[i] + other.acc[i];
            result.gyro[i] = self.gyro[i] + other.gyro[i];
        }
        result
    }
}

impl Sub for ImuReading {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = ImuReading::zero();
        for i in 0..3 {
            result.acc[i] = self.acc[i].saturating_sub(other.acc[i]);
            result.gyro[i] = self.gyro[i].saturating_sub(other.gyro[i]);
        }
        result
    }
}

impl Div<i32> for ImuReading {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        let mut result = ImuReading::zero();
        for i in 0..3 {
            result.acc[i] = self.acc[i] / rhs;
            result.gyro[i] = self.gyro[i] / rhs;
        }
        result
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ImuValues(pub [ImuReading; 5]);

impl ImuValues {
    pub const fn zero() -> Self {
        Self([ImuReading::zero(); 5])
    }
}

impl Add for ImuValues {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = ImuValues::zero();
        for i in 0..5 {
            result.0[i] = self.0[i] + other.0[i];
        }
        result
    }
}

impl Sub for ImuValues {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = ImuValues::zero();
        for i in 0..5 {
            result.0[i] = self.0[i] - other.0[i];
        }
        result
    }
}

impl Div<i32> for ImuValues {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        let mut result = ImuValues::zero();
        for i in 0..5 {
            result.0[i] = self.0[i] / rhs;
        }
        result
    }
}

impl Sum for ImuValues {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = ImuValues::zero();
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
    pub imu_values: ImuValues,
}

impl FlexSensorGloveNotification {
    pub fn from_buffer(buffer: &[u8], dt_start: DateTime<Local>) -> Self {
        let mut flex_values = [0; 5];
        for i in 0..5 {
            flex_values[i] = i16::from_le_bytes([buffer[i * 2], buffer[i * 2 + 1]]) as i32;
        }

        let mut imu_values = ImuValues::zero();
        for imu_idx in 0..5 {
            let base = 10 + imu_idx * 12;
            let mut axes = [0i32; 6];

            for axis_idx in 0..6 {
                let off = base + axis_idx * 2;
                axes[axis_idx] = i16::from_le_bytes([buffer[off], buffer[off + 1]]) as i32;
            }
            
            imu_values.0[imu_idx] = ImuReading {
                acc: [axes[0], axes[1], axes[2]],
                gyro: [axes[3], axes[4], axes[5]],
            };
        }

        let millis = u32::from_le_bytes([buffer[70], buffer[71], buffer[72], buffer[73]]);
        let millis = TimeDelta::milliseconds(millis as i64);

        FlexSensorGloveNotification {
            dt: dt_start + millis,
            flex_values: FingersFlexValues(flex_values),
            imu_values,
        }
    }
}

impl Display for FlexSensorGloveNotification {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}: flex={:?}, imu={:?}",
            self.dt, self.flex_values, self.imu_values
        )
    }
}

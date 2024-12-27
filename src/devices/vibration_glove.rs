use std::ops::Range;

use anyhow::bail;
use btleplug::api::{Characteristic, Peripheral, WriteType};
use btleplug::platform::Peripheral as PlatformPeripheral;
use chrono::{DateTime, Duration, Local};
use rand::Rng;
use uuid::Uuid;

use crate::opt::Opt;

const VIBRATION_GLOVE_CHAR_UUID: Uuid = Uuid::from_u128(0x7f2697eb_0b07_40c7_ab91_a17be8b47650);

const DEFAULT_RANDOM_MODE_VIBRATION_INTENSITY: u8 = 150;
const DEFAULT_RANDOM_MODE_VIBRATION_DURATION: i64 = 100;

const DEFAULT_RANDOM_MODE_DELAY_RANGE: Range<i64> = 2000..4000;

const DEFAULT_DESINHIBITION_MODE_VIBRATION_INTENSITY: u8 = 60;
const DEFAULT_DESINHIBITION_MODE_VIBRATION_DURATION: i64 = 5000;
const DEFAULT_DESINHIBITION_MODE_VIBRATION_DELAY: i64 = 5000;

pub enum VibrationMode {
    Desinhibition(DesinhibitionVibrationModeConfig),
    _SynapticResponse,
    Random(RandomVibrationModeConfig),
}

pub struct DesinhibitionVibrationModeConfig {
    pub vibration_intensity: u8,
    pub vibration_duration: i64,
    pub vibration_delay: i64,

    last_vibration_start_time: DateTime<Local>,
}

impl DesinhibitionVibrationModeConfig {
    pub fn process_moved_fingers(&mut self, time: DateTime<Local>) -> FingersVibrationIntensity {
        if self.last_vibration_start_time
            + Duration::milliseconds(self.vibration_delay + self.vibration_duration)
            >= time
        {
            self.last_vibration_start_time = time;
            return [self.vibration_intensity; 5];
        }

        if self.last_vibration_start_time + Duration::milliseconds(self.vibration_duration) <= time
        {
            return [self.vibration_intensity; 5];
        }

        [0; 5]
    }
}

pub struct RandomVibrationModeConfig {
    pub delay_range: Range<i64>,

    pub vibration_duration: i64,
    pub vibration_intensity: u8,

    current_finger_start_time: Option<(u8, DateTime<Local>)>,

    next_time: DateTime<Local>,

    rng: rand::rngs::ThreadRng,
}

impl RandomVibrationModeConfig {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let delay_range = DEFAULT_RANDOM_MODE_DELAY_RANGE;
        let next_time =
            chrono::Local::now() + Duration::milliseconds(rng.gen_range(delay_range.clone()));

        Self {
            delay_range,

            vibration_duration: DEFAULT_RANDOM_MODE_VIBRATION_DURATION,
            vibration_intensity: DEFAULT_RANDOM_MODE_VIBRATION_INTENSITY,

            current_finger_start_time: None,
            next_time,

            rng,
        }
    }

    fn process_moved_fingers(&mut self, time: DateTime<Local>) -> FingersVibrationIntensity {
        if let Some((finger, start_time)) = self.current_finger_start_time {
            if start_time + Duration::milliseconds(self.vibration_duration) >= time {
                self.current_finger_start_time = None;
                self.next_time =
                    time + Duration::milliseconds(self.rng.gen_range(self.delay_range.clone()));
            } else {
                return self.vibration_for_one_finger(finger);
            }
        }

        if self.next_time >= time {
            let next_finger = self.rng.gen_range(0..5);
            self.current_finger_start_time = Some((next_finger, chrono::Local::now()));
            return self.vibration_for_one_finger(next_finger);
        }

        [0; 5]
    }

    fn vibration_for_one_finger(&self, finger_index: u8) -> FingersVibrationIntensity {
        let mut fingers_intensity = [0; 5];
        fingers_intensity[finger_index as usize] = self.vibration_intensity;
        fingers_intensity
    }
}

pub struct VibrationGlove {
    peripheral: PlatformPeripheral,
    write_char: Characteristic,

    last_state: FingersVibrationIntensity,
    vibration_intensity: u8,

    pub vibration_mode: VibrationMode,
}

pub type FingersVibrationIntensity = [u8; 5];

impl VibrationGlove {
    pub async fn new(opt: &Opt) -> anyhow::Result<Self> {
        let peripheral = super::find_ble_device(&opt.input_glove_name, opt.verbose).await?;

        if let Err(err) = peripheral.connect().await {
            bail!("Error connecting to the vibration glove, skipping: {}", err);
        }

        let write_char = super::find_characteristic(&peripheral, VIBRATION_GLOVE_CHAR_UUID).await?;

        Ok(Self {
            peripheral,
            write_char,
            last_state: [0; 5],
            vibration_intensity: opt.vibration_intensity,
            vibration_mode: VibrationMode::Desinhibition(DesinhibitionVibrationModeConfig {
                vibration_intensity: DEFAULT_DESINHIBITION_MODE_VIBRATION_INTENSITY,
                vibration_duration: DEFAULT_DESINHIBITION_MODE_VIBRATION_DURATION,
                vibration_delay: DEFAULT_DESINHIBITION_MODE_VIBRATION_DELAY,
                last_vibration_start_time: chrono::Local::now(),
            }),
        })
    }

    pub fn set_vibration_mode(&mut self, vibration_mode: VibrationMode) {
        self.vibration_mode = vibration_mode;
    }

    pub async fn process_moved_fingers(
        &mut self,
        moved_fingers: &[bool; 5],
        time: DateTime<Local>,
    ) -> anyhow::Result<()> {
        let vibration_state = match &mut self.vibration_mode {
            VibrationMode::_SynapticResponse => moved_fingers.map(|is_moving| {
                if is_moving {
                    self.vibration_intensity
                } else {
                    0
                }
            }),
            VibrationMode::Random(config) => config.process_moved_fingers(time),
            VibrationMode::Desinhibition(config) => config.process_moved_fingers(time),
        };

        self.update_state(vibration_state).await
    }

    pub async fn update_state(&mut self, data: FingersVibrationIntensity) -> anyhow::Result<()> {
        if data == self.last_state {
            return Ok(());
        }

        self.peripheral
            .write(&self.write_char, &data, WriteType::WithoutResponse)
            .await?;

        self.last_state = data;

        Ok(())
    }

    pub fn get_last_state(&self) -> FingersVibrationIntensity {
        self.last_state
    }
}

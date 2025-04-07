use anyhow::bail;
use btleplug::api::{Characteristic, Peripheral};
use btleplug::platform::Peripheral as PlatformPeripheral;
use futures::StreamExt;
use uuid::Uuid;

use crate::opt::Opt;
use crate::parser::FlexSensorGloveNotification;
use crate::print_info;

const _FLEX_SENSOR_GLOVE_SERVICE_UUID: Uuid =
    Uuid::from_u128(0xf5874094_9074_4bb6_9257_f3593d73d836);

const FLEX_SENSOR_GLOVE_CHAR_UUID: Uuid = Uuid::from_u128(0xa81ed63c_cf54_4742_a27a_f398228acd90);

pub struct FlexSensorGlove {
    notify_char: Characteristic,
    connect_time: chrono::DateTime<chrono::Local>,

    /// The peripheral is an Option to be able to take it when dropping
    peripheral: Option<PlatformPeripheral>,
}

impl FlexSensorGlove {
    pub async fn new(opt: &Opt) -> anyhow::Result<Self> {
        let peripheral = super::find_ble_device(&opt.output_glove_name, opt.verbose).await?;

        if opt.verbose {
            print_info("FlexSensorGlove found connecting...");
        }

        if !peripheral.is_connected().await? {
            if let Err(err) = peripheral.connect().await {
                bail!(
                    "Error connecting to the flex sensor glove, skipping: {}",
                    err
                );
            }
        }

        if opt.verbose {
            print_info("Connected to the flex sensor glove");
        }

        let notify_char =
            super::find_characteristic(&peripheral, FLEX_SENSOR_GLOVE_CHAR_UUID).await?;

        Ok(Self {
            peripheral: Some(peripheral),
            notify_char,
            connect_time: chrono::Local::now(),
        })
    }

    pub async fn get_notifications_stream(
        &self,
    ) -> anyhow::Result<impl futures::Stream<Item = FlexSensorGloveNotification> + '_> {
        self.peripheral
            .as_ref()
            .unwrap()
            .subscribe(&self.notify_char)
            .await?;

        Ok(self
            .peripheral
            .as_ref()
            .unwrap()
            .notifications()
            .await?
            .map(|notification| {
                FlexSensorGloveNotification::from_buffer(&notification.value, self.connect_time)
            }))
    }
}

impl Drop for FlexSensorGlove {
    fn drop(&mut self) {
        let peripheral = self.peripheral.take();
        if peripheral.is_none() {
            return;
        }

        tokio::spawn(async {
            if let Err(err) = peripheral.unwrap().disconnect().await {
                print_info(&format!(
                    "Error disconnecting from the flex sensor glove: {}",
                    err
                ));
            }
        });
    }
}

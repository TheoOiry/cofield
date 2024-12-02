use anyhow::{anyhow, bail};
use btleplug::api::{
    Central, CentralEvent, Characteristic, Manager as _, Peripheral, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral as PlatformPeripheral};
use futures::StreamExt;
use uuid::Uuid;

use crate::opt::Opt;
use crate::parser::FlexSensorGloveNotification;
use crate::print_info;

const _FLEX_SENSOR_GLOVE_SERVICE_UUID: Uuid =
    Uuid::from_u128(0xf5874094_9074_4bb6_9257_f3593d73d836);

const FLEX_SENSOR_GLOVE_CHAR_UUID: Uuid = Uuid::from_u128(0xa81ed63c_cf54_4742_a27a_f398228acd90);
const VIBRATION_GLOVE_CHAR_UUID: Uuid = Uuid::from_u128(0x7f2697eb_0b07_40c7_ab91_a17be8b47650);

pub struct FlexSensorGlove {
    peripheral: PlatformPeripheral,
    notify_char: Characteristic,
    connect_time: chrono::DateTime<chrono::Local>,
}

impl FlexSensorGlove {
    pub async fn new(opt: &Opt) -> anyhow::Result<Self> {
        let peripheral = find_ble_device(&opt.output_glove_name, opt.verbose).await?;

        if let Err(err) = peripheral.connect().await {
            bail!(
                "Error connecting to the flex sensor glove, skipping: {}",
                err
            );
        }

        let notify_char = find_characteristic(&peripheral, FLEX_SENSOR_GLOVE_CHAR_UUID).await?;

        Ok(Self {
            peripheral,
            notify_char,
            connect_time: chrono::Local::now(),
        })
    }

    pub async fn get_notifications_stream(
        &self,
    ) -> anyhow::Result<impl futures::Stream<Item = FlexSensorGloveNotification> + '_> {
        self.peripheral.subscribe(&self.notify_char).await?;
        Ok(self.peripheral.notifications().await?.map(|notification| {
            FlexSensorGloveNotification::from_buffer(&notification.value, self.connect_time)
        }))
    }
}

pub struct VibrationGlove {
    peripheral: PlatformPeripheral,
    write_char: Characteristic,
    last_state: FingersVibrationIntensity,
}

pub type FingersVibrationIntensity = [u8; 5];

impl VibrationGlove {
    pub async fn new(opt: &Opt) -> anyhow::Result<Self> {
        let peripheral = find_ble_device(&opt.input_glove_name, opt.verbose).await?;

        if let Err(err) = peripheral.connect().await {
            bail!("Error connecting to the vibration glove, skipping: {}", err);
        }

        let write_char = find_characteristic(&peripheral, VIBRATION_GLOVE_CHAR_UUID).await?;

        Ok(Self {
            peripheral,
            write_char,
            last_state: [0; 5],
        })
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
}

async fn find_characteristic(
    peripheral: &PlatformPeripheral,
    char_uuid: Uuid,
) -> anyhow::Result<Characteristic> {
    peripheral.discover_services().await?;

    peripheral
        .characteristics()
        .into_iter()
        .find(|c| c.uuid == char_uuid)
        .ok_or(anyhow!("Notify characteristic not found"))
}

async fn find_ble_device(device_name: &str, verbose: bool) -> anyhow::Result<PlatformPeripheral> {
    let adapter = find_central(verbose).await?;
    let mut events = adapter.events().await?;

    adapter.start_scan(ScanFilter::default()).await?;

    if verbose {
        print_info(&format!(
            "Statrting scan with adapter: {}",
            adapter.adapter_info().await?
        ));
    }

    while let Some(event) = events.next().await {
        let CentralEvent::DeviceDiscovered(id) = event else {
            continue;
        };

        let peripheral = adapter.peripheral(&id).await?;
        let properties = peripheral.properties().await?.unwrap();

        if verbose {
            print_info(&format!("Found device: {:?}", properties.local_name));
        }

        if properties.local_name == Some(device_name.into()) {
            adapter.stop_scan().await?;
            return Ok(peripheral);
        }
    }

    bail!("device {device_name} not found")
}

async fn find_central(verbose: bool) -> anyhow::Result<Adapter> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        bail!("No Bluetooth adapters found");
    }

    if verbose {
        print_info(&format!("Found adapters {}", adapter_list.len()));
    }

    Ok(adapter_list[0].clone())
}

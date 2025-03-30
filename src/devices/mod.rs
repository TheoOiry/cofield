use anyhow::{anyhow, bail};
use btleplug::api::{Central, CentralEvent, Characteristic, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral as PlatformPeripheral};
use futures::StreamExt;
use uuid::Uuid;

use crate::print_info;

pub mod flex_sensor_glove;
pub mod vibration_glove;

const _FLEX_SENSOR_GLOVE_SERVICE_UUID: Uuid =
    Uuid::from_u128(0xf5874094_9074_4bb6_9257_f3593d73d836);

pub use vibration_glove::*;

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

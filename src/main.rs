use core::str;

use anyhow::{anyhow, bail};
use btleplug::api::{Central, CentralEvent, Characteristic, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral as PlatformPeripheral};
use clap::Parser;
use console::style;
use dotenv::dotenv;
use futures::stream::StreamExt;
use opt::Opt;
use parser::FlexSensorGloveNotification;
use uuid::Uuid;

mod opt;
mod output;
mod parser;

const FLEX_SENSOR_GLOVE_SERVICE_UUID: Uuid =
    Uuid::from_u128(0xf5874094_9074_4bb6_9257_f3593d73d836);

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Opt::parse();

    if let Err(error) = run(args).await {
        eprintln!("{} {}", style("ERROR:").bold().red(), error);
        std::process::exit(1);
    }
}

fn print_info(str: &str) {
    eprintln!("{} {str}", style("INFO:").bold().cyan());
}

async fn run(opt: Opt) -> anyhow::Result<()> {
    let peripheral = scan_for_flex_sensor_glove(&opt.device_name, opt.verbose).await?;

    if opt.verbose {
        print_info("Peripheral found, connecting...");
    }

    if let Err(err) = peripheral.connect().await {
        bail!("Error connecting to peripheral, skipping: {}", err);
    }

    if opt.verbose {
        print_info("Connected to peripheral");
    }

    let dt_start = chrono::Local::now();

    peripheral.discover_services().await?;
    let notify_char = find_notify_characteristic(&peripheral).await?;

    peripheral.subscribe(&notify_char).await?;

    let mut notification_stream = peripheral.notifications().await?;
    let mut output_writer = opt.output_format.create_writer();

    if opt.verbose {
        print_info("Reading notifications...");
    }

    while let Some(notification) = notification_stream.next().await {
        let notification = FlexSensorGloveNotification::from_buffer(&notification.value, dt_start);
        output_writer.write_row(&notification)?;
    }

    Ok(())
}

async fn scan_for_flex_sensor_glove(device_name: &str, verbose: bool) -> anyhow::Result<PlatformPeripheral> {
    let adapter = find_central(verbose).await?;
    let mut events = adapter.events().await?;

    adapter.start_scan(ScanFilter::default()).await?;

    if verbose {
        print_info(&format!("Statrting scan with adapter: {}", adapter.adapter_info().await?));
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

async fn find_notify_characteristic(
    peripheral: &PlatformPeripheral,
) -> anyhow::Result<Characteristic> {
    peripheral.discover_services().await?;
    peripheral
        .services()
        .iter()
        .filter(|s| s.uuid == FLEX_SENSOR_GLOVE_SERVICE_UUID)
        .flat_map(|s| s.characteristics.iter())
        .next()
        .ok_or(anyhow!("Notify characteristic not found"))
        .cloned()
}

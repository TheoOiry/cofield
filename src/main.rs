use std::io;

use clap::Parser;
use cofield_receiver::{
    flex_sensor_glove::FlexSensorGlove, print_info, FlexSensorGloveNotification, Opt, Process,
    VibrationGlove,
};
use console::style;
use dotenv::dotenv;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Opt::parse();

    if let Err(error) = run(args).await {
        eprintln!("{} {}", style("ERROR:").bold().red(), error);
        std::process::exit(1);
    }
}

async fn run(opt: Opt) -> anyhow::Result<()> {
    if opt.input_from_stdin {
        return run_with_stdin(opt).await;
    }

    let flex_sensor_glove = FlexSensorGlove::new(&opt).await?;
    let notification_stream = Box::pin(flex_sensor_glove.get_notifications_stream().await?);

    let mut vibration_glove = match &opt.input_glove_name {
        Some(input_glove_name) => Some(VibrationGlove::new(input_glove_name, &opt).await?),
        None => None,
    };

    let output_writer = opt.output_format.create_writer();

    if opt.verbose {
        print_info("Reading notifications...");
    }

    let mut process = Process::new(
        notification_stream,
        opt.aggregation_size,
        opt.fingers_sensibility,
    )
    .await?;

    if let Some(vibration_glove) = &mut vibration_glove {
        process.set_vibration_glove(vibration_glove);
    }

    process.set_output_raw_data(opt.output_raw_data)?;
    process.set_output_writer(output_writer);

    #[cfg(feature = "lsl")]
    if opt.lsl {
        let lsl_stream_outlet = cofield_receiver::lsl_setup::setup_stream_outlet()?;
        process.set_lsl_stream_outlet(lsl_stream_outlet);
    }

    process.run().await?;

    Ok(())
}

async fn run_with_stdin(opt: Opt) -> anyhow::Result<()> {
    let output_writer = opt.output_format.create_writer();
    let notification_stream = get_stdin_csv_notification_stream().await;

    let mut process = Process::new(
        notification_stream,
        opt.aggregation_size,
        opt.fingers_sensibility,
    )
    .await?;

    process.set_output_writer(output_writer);
    process.set_output_raw_data(opt.output_raw_data)?;

    process.run().await?;

    Ok(())
}

async fn get_stdin_csv_notification_stream(
) -> futures::stream::BoxStream<'static, FlexSensorGloveNotification> {
    let notifications: Vec<FlexSensorGloveNotification> = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin())
        .into_deserialize()
        .map(|row| row.unwrap())
        .collect();

    futures::stream::iter(notifications).boxed()
}

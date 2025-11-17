use core::str;
use std::{io, sync::Arc};

use clap::Parser;
use cofield_receiver::{
    flex_sensor_glove::FlexSensorGlove, FlexSensorGloveNotification, Opt, Process,
};
use console::style;
use dotenv::dotenv;
use futures::{stream::BoxStream, StreamExt};
use tokio::sync::Mutex;

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
    // We print out logs to stderr to not interfere with stdout data
    eprintln!("{} {str}", style("INFO:").bold().cyan());
}

async fn run(opt: Opt) -> anyhow::Result<()> {
    if opt.input_from_stdin {
        return run_with_stdin(opt).await;
    }

    let flex_sensor_glove = FlexSensorGlove::new(&opt).await?;
    let notification_stream = Box::pin(flex_sensor_glove.get_notifications_stream().await?);

    let output_writer = opt.output_format.create_writer();

    if opt.verbose {
        print_info("Reading notifications...");
    }

    let mut process = Process::new(
        notification_stream,
        opt.fingers_sensibility,
    )
    .await;

    process.set_aggregator(Arc::new(Mutex::new(opt.get_mean_aggregator())));
    process.set_output_writer(Arc::new(Mutex::new(Some(output_writer))));

    #[cfg(feature = "lsl")]
    if opt.lsl {
        let lsl_stream_outlet = lsl_setup::setup_stream_outlet()?;
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
        opt.fingers_sensibility,
    )
    .await;

    process.set_aggregator(Arc::new(Mutex::new(opt.get_mean_aggregator())));
    process.set_output_writer(Arc::new(Mutex::new(Some(output_writer))));

    process.run().await?;

    Ok(())
}

async fn get_stdin_csv_notification_stream() -> BoxStream<'static, FlexSensorGloveNotification> {
    let notifications: Vec<FlexSensorGloveNotification> = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin())
        .into_deserialize()
        .map(|row| row.unwrap())
        .collect();

    futures::stream::iter(notifications).boxed()
}

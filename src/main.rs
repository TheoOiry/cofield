use core::str;

use clap::Parser;
use console::style;
use devices::{FingersVibrationIntensity, FlexSensorGlove, VibrationGlove};
use dotenv::dotenv;
use futures::stream::StreamExt;
use opt::Opt;
use aggregator::MeanAggregator;

mod aggregator;
mod devices;
mod opt;
mod output;
mod parser;


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
    let flex_sensor_glove = FlexSensorGlove::new(&opt).await?;
    let mut vibration_glove = VibrationGlove::new(&opt).await?;

    let mut notification_stream = flex_sensor_glove.get_notifications_stream().await?;
    let mut output_writer = opt.output_format.create_writer();

    if opt.verbose {
        print_info("Reading notifications...");
    }

    let init_data = notification_stream.by_ref().take(opt.aggregation_size).collect().await;
    let mut aggregator = MeanAggregator::new(init_data);

    while let Some(notification) = notification_stream.next().await {
        let mut vibration_state: FingersVibrationIntensity = [0; 5];

        let aggregated_notification = aggregator.push_and_aggregate(notification.clone());

        aggregated_notification
            .flex_values
            .0
            .iter()
            .enumerate()
            .for_each(|(i, &value)| {
                vibration_state[i] = if value > opt.fingers_sensibility.0[i] {
                    opt.vibration_intensity
                } else {
                    0
                };
            });

        vibration_glove.update_state(vibration_state).await?;

        output_writer.write_row(&notification)?;
    }

    Ok(())
}

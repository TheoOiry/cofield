use core::str;

use clap::Parser;
use console::style;
use devices::{FlexSensorGlove, VibrationGlove};
use dotenv::dotenv;
use opt::Opt;
use process::Process;

mod aggregator;
mod devices;
mod lsl_setup;
mod opt;
mod output;
mod parser;
mod patterns;
mod process;

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

    let mut vibration_glove = match &opt.input_glove_name {
        Some(input_glove_name) => Some(VibrationGlove::new(input_glove_name, &opt).await?),
        None => None,
    };

    let output_writer = opt.output_format.create_writer();

    if opt.verbose {
        print_info("Reading notifications...");
    }

    let mut process = Process::new(
        &flex_sensor_glove,
        opt.aggregation_size,
        opt.fingers_sensibility,
    )
    .await?;

    if let Some(vibration_glove) = &mut vibration_glove {
        process.set_vibration_glove(vibration_glove);
    }

    process.set_output_writer(output_writer);

    if opt.lsl {
        let lsl_stream_outlet = lsl_setup::setup_stream_outlet()?;
        process.set_lsl_stream_outlet(lsl_stream_outlet);
    }

    process.run().await?;

    Ok(())
}

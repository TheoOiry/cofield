use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Opt {
    #[arg(short, long, value_enum, default_value_t=OutputFormat::default())]
    pub output_format: OutputFormat,

    #[arg(long, short, default_value = "FlexSensorGlove")]
    pub device_name: String,
}

#[derive(Copy, Clone, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Pretty,
    Csv,
}

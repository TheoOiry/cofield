use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Opt {
    #[arg(short, long, value_enum, default_value_t=OutputFormat::default())]
    pub output_format: OutputFormat,

    #[arg(long, default_value = "FlexSensorGlove")]
    pub output_glove_name: String,

    #[arg(long, default_value = "VibrationGlove")]
    pub input_glove_name: String,

    #[arg(long, default_value = "10")]
    pub aggregation_size: usize,

    #[arg(long, default_value = "15")]
    pub sensibility: u32,

    #[arg(long, default_value = "255")]
    pub vibration_intensity: u8,

    #[arg(long, short, default_value = "false")]
    pub verbose: bool,
}

#[derive(Copy, Clone, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Pretty,
    Csv,
}

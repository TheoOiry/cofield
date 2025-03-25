use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Opt {
    /// Read from stdin to get the glove notifications in csv format.
    /// In this mode, the gloves are not used and lsl is disabled
    #[arg(long, default_value = "false")]
    pub input_from_stdin: bool,

    #[arg(short, long, value_enum, default_value_t=OutputFormat::default())]
    pub output_format: OutputFormat,

    #[arg(long, default_value = "FlexSensorGlove")]
    pub output_glove_name: String,

    #[arg(long)]
    pub input_glove_name: Option<String>,

    #[arg(long, default_value = "10")]
    pub aggregation_size: usize,

    #[arg(long, default_value = "[15, 15, 15, 15, 15]")]
    pub fingers_sensibility: FingersSensibility,

    #[arg(long, default_value = "255")]
    pub vibration_intensity: u8,

    #[arg(long, short, default_value = "false")]
    pub verbose: bool,

    #[arg(long, default_value = "false")]
    pub lsl: bool,
}

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct FingersSensibility(pub [u32; 5]);

impl std::str::FromStr for FingersSensibility {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|e| format!("error parsing my struct: {}", e))
    }
}

#[derive(Copy, Clone, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Pretty,
    Csv,
}

use std::{
    fmt::{self, Display, Formatter},
    io::Stdout,
};

use serde::Serialize;

use crate::{opt::OutputFormat, parser::FlexSensorGloveNotification};

#[derive(Serialize, Debug)]
pub struct OutputRow<'a> {
    pub notification: &'a FlexSensorGloveNotification,
    pub vibration_state: &'a [u8; 5],
}

impl<'a> Display for OutputRow<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}: values: {:?} vibrations: {:?}",
            self.notification.dt, self.notification.flex_values, self.vibration_state
        )
    }
}

pub trait OutputWriter {
    fn write_row(&mut self, record: &OutputRow) -> anyhow::Result<()>;
}

struct PrettyWriter;

impl OutputWriter for PrettyWriter {
    fn write_row(&mut self, record: &OutputRow) -> anyhow::Result<()> {
        println!("{record}");
        Ok(())
    }
}

impl OutputWriter for csv::Writer<Stdout> {
    fn write_row(&mut self, record: &OutputRow) -> anyhow::Result<()> {
        self.serialize(record)?;
        Ok(())
    }
}

impl OutputFormat {
    pub fn create_writer(&self) -> Box<dyn OutputWriter> {
        match self {
            OutputFormat::Pretty => Box::new(PrettyWriter),
            OutputFormat::Csv => Box::new(
                csv::WriterBuilder::new()
                    .has_headers(false)
                    .from_writer(std::io::stdout()),
            ),
        }
    }
}

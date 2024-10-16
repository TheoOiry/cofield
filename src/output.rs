use std::io::Stdout;

use crate::{opt::OutputFormat, parser::FlexSensorGloveNotification};

pub trait OutputWriter {
    fn write_row(&mut self, record: &FlexSensorGloveNotification) -> anyhow::Result<()>;
}

struct PrettyWriter;

impl OutputWriter for PrettyWriter {
    fn write_row(&mut self, record: &FlexSensorGloveNotification) -> anyhow::Result<()> {
        println!("{record}");
        Ok(())
    }
}

impl OutputWriter for csv::Writer<Stdout> {
    fn write_row(&mut self, record: &FlexSensorGloveNotification) -> anyhow::Result<()> {
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

mod aggregator;
mod devices;

#[cfg(feature = "lsl")]
mod lsl_setup;

mod opt;
mod output;
mod parser;
mod patterns;
mod process;

pub use devices::*;

#[cfg(feature = "lsl")]
pub use lsl_setup::*;

pub use aggregator::*;
pub use opt::*;
pub use output::*;
pub use parser::*;
pub use patterns::*;
pub use process::*;

use console::style;

pub fn print_info(str: &str) {
    eprintln!("{} {str}", style("INFO:").bold().cyan());
}

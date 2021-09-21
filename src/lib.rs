use anyhow::{Context, Result};

pub mod gatherer;
pub mod printer;
pub mod runner;
pub mod types;

pub use gatherer::{Gatherer, YamlGatherer};
pub use printer::{ColorPrinter, Printer};
pub use runner::{DefaultRunner, Runner};

pub fn run(gatherer: impl Gatherer, runner: impl Runner, printer: impl Printer) -> Result<()> {
    let collection = gatherer.gather().context("Failed to parse YAML files")?;
    let report = runner
        .run_all_tests(collection)
        .context("Failed to run tests")?;

    printer
        .print_report(&report)
        .context("Failed to write report")?;

    Ok(())
}

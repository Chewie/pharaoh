use anyhow::{Context, Result};

use clap::{App, Arg};

mod gatherer;
mod printer;
mod runner;
mod types;

use gatherer::{yaml::YamlGatherer, Gatherer};
use printer::Printer;
use runner::Runner;

pub fn run() -> Result<()> {
    let matches = build_args().get_matches();
    let search_dir = matches.value_of("search_dir").unwrap_or(".");

    let gatherer = YamlGatherer::new(search_dir.to_string());
    let runner = Runner::new();
    let printer = Printer::new();

    let collection = gatherer.gather().context("Failed to parse YAML files")?;
    let report = runner
        .run_all_tests(collection)
        .context("Failed to run tests")?;

    printer
        .print_report(&report, &mut std::io::stdout())
        .context("Failed to write report")?;

    Ok(())
}

fn build_args() -> App<'static, 'static> {
    App::new("Pharaoh").arg(Arg::with_name("search_dir").index(1).default_value("."))
}

use std::error::Error;

use clap::{App, Arg};

mod gatherer;
mod printer;
mod runner;
mod testcase;

use gatherer::{yaml::YamlGatherer, Gatherer};

pub fn run() -> Result<(), Box<dyn Error>> {
    let matches = build_args().get_matches();

    let search_dir = matches.value_of("search_dir").unwrap_or(".");

    let gatherer = YamlGatherer::new(search_dir.to_string());

    let testfiles = gatherer.gather()?;

    if testfiles.testsuites.is_empty() {
        println!("No test case found. Exiting.");
        return Ok(());
    }

    let report = runner::run_all_tests(&testfiles.testsuites)?;

    printer::print_report(&report);

    Ok(())
}

fn build_args() -> App<'static, 'static> {
    App::new("Pharaoh").arg(Arg::with_name("search_dir").index(1).default_value("."))
}

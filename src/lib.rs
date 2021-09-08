use std::error::Error;

use clap::{App, Arg};

mod gather;
mod printer;
mod runner;
mod testcase;

pub fn run() -> Result<(), Box<dyn Error>> {
    let matches = build_args().get_matches();

    let search_dir = matches.value_of("search_dir").unwrap_or(".");

    let testfiles = gather::gather_testfiles(search_dir)?;

    if testfiles.is_empty() {
        println!("No test case found. Exiting.");
        return Ok(());
    }

    let report = runner::run_all_tests(&testfiles)?;

    printer::print_report(&report);

    Ok(())
}

fn build_args() -> App<'static, 'static> {
    App::new("Pharaoh").arg(Arg::with_name("search_dir").index(1).default_value("."))
}

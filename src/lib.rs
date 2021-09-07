use std::error::Error;

use clap::{App, Arg};

mod test_case;
use test_case::TestFile;

mod gather;

mod printer;

pub fn run() -> Result<(), Box<dyn Error>> {
    let matches = build_args().get_matches();

    let search_dir = matches.value_of("search_dir").unwrap_or(".");

    let testfiles = gather::gather_testfiles(search_dir)?;

    if testfiles.is_empty() {
        println!("No test case found. Exiting.");
        return Ok(());
    }

    for testfile in testfiles {
        run_testfile(testfile);
    }

    Ok(())
}

fn build_args() -> App<'static, 'static> {
    App::new("Pharaoh").arg(Arg::with_name("search_dir").index(1).default_value("."))
}

fn run_testfile(testfile: TestFile) {
    println!("Running tests for {}", testfile.name);
    for test in &testfile.tests {
        //printer::run_test(&testfile, &test);
    }
}

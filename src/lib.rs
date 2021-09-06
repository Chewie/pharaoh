use std::error::Error;

use globwalk::{GlobWalkerBuilder, DirEntry};
use clap::{App, Arg};
use serde_yaml::Value;
use serde::Deserialize;

mod test_case;
use test_case::{TestCase, TestFile};

pub fn run() -> Result<(), Box<dyn Error>> {
    let matches = build_args().get_matches();

    let search_dir = matches.value_of("search_dir").unwrap_or(".");

    let entries = get_yamls(search_dir)?;

    if entries.is_empty() {
        println!("No test case found. Exiting.");
        return Ok(());
    }
    for entry in entries {
        let testfile = get_testfile_from_entry(search_dir, entry)?;
        run_testfile(testfile);
    }
    Ok(())
}

fn build_args() -> App<'static, 'static> {
    App::new("Pharaoh")
        .arg(Arg::with_name("search_dir")
            .index(1)
            .default_value("."))
}


fn get_yamls(search_dir: &str) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    Ok(GlobWalkerBuilder::from_patterns(
        search_dir,
        &["**/*.yaml", "**/*.yml"]
    )
        .min_depth(1)
        .build()?
        .into_iter()
        .filter_map(Result::ok)
        .collect())
}

fn run_testfile(testfile: TestFile) {
    println!("Running tests for {}", testfile.name);
    for test in testfile.tests {
        println!("{}: OK", test.name);
    }
}


fn get_testfile_from_entry(search_dir: &str, entry : DirEntry) -> Result<TestFile, Box<dyn Error>> {
    let file = std::fs::File::open(entry.path())?;
    let filename = entry.path().with_extension("");
    let filename = filename.strip_prefix(search_dir)?;
    let filename = filename.display().to_string();

    let mut result = TestFile{name: filename, tests: vec!()};

    for document in serde_yaml::Deserializer::from_reader(file) {
        let value = Value::deserialize(document)?;
        let test_case : TestCase = serde_yaml::from_value(value)?;
        result.tests.push(test_case);
    }
    Ok(result)
}

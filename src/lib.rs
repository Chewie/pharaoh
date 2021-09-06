use std::error::Error;

use globwalk::{GlobWalkerBuilder, DirEntry};
use clap::{App, Arg};

pub fn run() -> Result<(), Box<dyn Error>> {
    let matches = build_args().get_matches();

    let search_dir = matches.value_of("search_dir").unwrap_or(".");

    let entries = get_yamls(search_dir)?;

    if entries.is_empty() {
        println!("No test case found. Exiting.");
        return Ok(());
    }
    for entry in entries {
        handle_entry(entry);
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


fn handle_entry(entry : DirEntry) {
    println!("Running tests for {}", entry.path().display());
}

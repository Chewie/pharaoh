use anyhow::Result;
use clap::*;

use pharaoh::ColorPrinter;
use pharaoh::DefaultRunner;
use pharaoh::YamlGatherer;

fn main() -> Result<()> {
    let matches = build_args().get_matches();
    let search_dir = matches.value_of("search_dir").unwrap_or(".");

    let gatherer = YamlGatherer::new(search_dir.to_string());
    let runner = DefaultRunner::new();
    let printer = ColorPrinter::new(std::io::stdout());

    pharaoh::run(gatherer, runner, printer)
}

fn build_args() -> App<'static, 'static> {
    clap::app_from_crate!().arg(
        Arg::with_name("search_dir")
            .index(1)
            .help("The directory in which YAML are searched")
            .default_value("."),
    )
}

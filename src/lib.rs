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

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    use crate::types::result::TestReport;
    use crate::types::testcase::TestSuiteCollection;

    #[test]
    fn test_run() {
        // GIVEN
        let mut gatherer = gatherer::MockGatherer::new();
        gatherer
            .expect_gather()
            .times(1)
            .return_once(move || Ok(a_testsuite_collection()));

        let mut runner = runner::MockRunner::new();
        runner
            .expect_run_all_tests()
            .with(predicate::eq(a_testsuite_collection()))
            .times(1)
            .return_once(move |_| Ok(the_resulting_report()));

        let mut printer = printer::MockPrinter::new();
        printer
            .expect_print_report()
            .with(predicate::eq(the_resulting_report()))
            .times(1)
            .return_once(move |_| Ok(()));

        // WHEN
        let run_result = run(gatherer, runner, printer);

        // THEN
        assert!(run_result.is_ok());
    }

    fn a_testsuite_collection() -> TestSuiteCollection {
        TestSuiteCollection::default()
    }

    fn the_resulting_report() -> TestReport {
        TestReport::default()
    }
}

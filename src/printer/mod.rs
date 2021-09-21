use anyhow::Result;
use colored::Colorize;

mod formatter;

use crate::types::result::TestReport;
use formatter::{DefaultFormatter, Formatter};

pub trait Printer {
    fn print_report(&self, report: &TestReport, writer: impl std::io::Write) -> Result<()>;
}

pub struct ColorPrinter<F: Formatter> {
    formatter: F,
}

impl ColorPrinter<DefaultFormatter> {
    pub fn new() -> Self {
        Self::with_formatter(DefaultFormatter::new())
    }
}

impl Default for ColorPrinter<DefaultFormatter> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: Formatter> Printer for ColorPrinter<F> {
    fn print_report(&self, report: &TestReport, mut writer: impl std::io::Write) -> Result<()> {
        if report.testsuites.is_empty() {
            writeln!(writer, "No test case found. Exiting.")?;
            return Ok(());
        }
        let mut failures = vec![];

        for suite_result in &report.testsuites {
            writeln!(writer, "Running tests for {}", suite_result.name)?;
            for test_result in &suite_result.results {
                let successful = test_result.is_successful();
                let success_msg = match successful {
                    true => "OK".green(),
                    false => "FAILED".red(),
                };
                writeln!(writer, "test {} ... {}", test_result.name, success_msg)?;
                if !successful {
                    failures.push(test_result);
                }
            }
        }

        if !failures.is_empty() {
            writeln!(writer, "\nfailures:\n")?;
            for failure in failures {
                writeln!(writer, "---- {} ----", failure.name)?;
                writeln!(writer, "{}", self.formatter.compute_summary(failure))?;
            }
        }
        Ok(())
    }
}

impl<F: Formatter> ColorPrinter<F> {
    pub fn with_formatter(formatter: F) -> Self {
        ColorPrinter { formatter }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::formatdoc;
    use mockall::*;

    use crate::types::result::{TestResultBuilder, TestSuiteResult};

    #[test]
    fn test_new_calls_with_formatter() {
        // GIVEN
        let formatter = DefaultFormatter::new();

        // WHEN
        let printer = ColorPrinter::new();

        // THEN
        assert_eq!(formatter, printer.formatter);
    }

    #[test]
    fn test_print_report_empty() {
        // GIVEN
        let report = TestReport { testsuites: vec![] };
        let mut result = Vec::new();
        let printer = ColorPrinter::new();

        // WHEN
        printer.print_report(&report, &mut result).unwrap();

        // THEN
        assert_eq!(
            "No test case found. Exiting.\n",
            std::str::from_utf8(&result).unwrap()
        );
    }

    #[test]
    fn test_print_report_successful_test() {
        // GIVEN
        let report = TestReport {
            testsuites: vec![TestSuiteResult {
                name: "mysuite".to_string(),
                results: vec![TestResultBuilder::default()
                    .name("mytestcase")
                    .build()
                    .unwrap()],
            }],
        };
        let mut result = Vec::new();
        let printer = ColorPrinter::new();

        // WHEN
        printer.print_report(&report, &mut result).unwrap();

        // THEN
        assert_eq!(
            formatdoc! {r#"
            Running tests for mysuite
            test mytestcase ... {ok}
            "#, ok="OK".green()},
            std::str::from_utf8(&result).unwrap()
        );
    }

    #[test]
    fn test_print_report_failing_test() {
        // GIVEN
        let mut result = Vec::new();
        let failing_test = TestResultBuilder::default()
            .name("failingtest")
            .expected_stdout("foo")
            .actual_stdout("bar")
            .build()
            .unwrap();

        let report = TestReport {
            testsuites: vec![TestSuiteResult {
                name: "mysuite".to_string(),
                results: vec![failing_test.clone()],
            }],
        };
        let mut mock_formatter = formatter::MockFormatter::new();
        mock_formatter
            .expect_compute_summary()
            .with(predicate::eq(failing_test))
            .times(1)
            .return_const("FAIL\n");

        let printer = ColorPrinter::with_formatter(mock_formatter);

        // WHEN
        printer.print_report(&report, &mut result).unwrap();

        // THEN
        pretty_assertions::assert_eq!(
            formatdoc! {r#"
            Running tests for mysuite
            test failingtest ... {failed}

            failures:

            ---- failingtest ----
            FAIL

            "#, failed="FAILED".red()},
            std::str::from_utf8(&result).unwrap()
        );
    }
}

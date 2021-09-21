use colored::Colorize;
use indoc::printdoc;

use crate::types::result::TestReport;

mod formatter;

use formatter::Formatter;

pub struct Printer {
    formatter: Formatter,
}

impl Printer {
    pub fn new() -> Self {
        Printer {
            formatter: Formatter::new(),
        }
    }

    pub fn print_report(&self, report: &TestReport) {
        if report.testsuites.is_empty() {
            println!("No test case found. Exiting.");
            return;
        }
        let mut failures = vec![];

        for suite_result in &report.testsuites {
            println!("Running tests for {}", suite_result.name);
            for test_result in &suite_result.results {
                let successful = test_result.is_successful();
                let success_msg = match successful {
                    true => "OK".green(),
                    false => "FAILED".red(),
                };
                println!("test {} ... {}", test_result.name, success_msg);
                if !successful {
                    failures.push(test_result);
                }
            }
        }

        if !failures.is_empty() {
            println!("\nfailures:\n");
            for failure in failures {
                printdoc! {r#"
            ---- {} ----
            {}
            "#,
                failure.name,
                self.formatter.compute_summary(failure)};
            }
        }
    }
}

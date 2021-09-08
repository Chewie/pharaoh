use colored::Colorize;
use indoc::{formatdoc, printdoc};
use similar::{ChangeTag, TextDiff};

use crate::runner::{TestReport, TestResult};

pub fn print_report(report: &TestReport) {
    let mut failures = vec![];

    for suite_result in &report.testsuites {
        println!("Running tests for {}", suite_result.name);
        for test_result in &suite_result.results {
            let successful = test_result.is_successful();
            println!("{}", format_oneliner(test_result, successful));
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
            compute_summary(failure)};
        }
    }
}

fn compute_summary(result: &TestResult) -> String {
    vec![
        compute_status(result.expected_status, result.actual_status),
        compute_diff("stdout", &result.expected_stdout, &result.actual_stdout),
        compute_diff("stderr", &result.expected_stderr, &result.actual_stderr),
    ]
    .join("")
}

fn compute_status(expected: i32, actual: i32) -> String {
    match expected == actual {
        true => String::new(),
        false => formatdoc!(
            r#"
            {} differs:
            expected: {}
            actual: {}
            "#,
            "status code".yellow(),
            expected,
            actual
        ),
    }
}

fn compute_diff(name: &str, expected: &str, actual: &str) -> String {
    let mut diff_summary = vec![];
    let diff = TextDiff::from_lines(expected, actual);
    if !diff.ops().to_vec().is_empty() {
        diff_summary.push(formatdoc!(
            r#"
            {} differs:
            {} expected
            {} actual
            "#,
            name.yellow(),
            "---".green(),
            "+++".red(),
        ));
    }
    for change in diff.iter_all_changes() {
        diff_summary.push(match change.tag() {
            ChangeTag::Delete => format!("-{}", change).green().to_string(),
            ChangeTag::Insert => format!("+{}", change).red().to_string(),
            ChangeTag::Equal => format!(" {}", change),
        });
    }
    diff_summary.join("")
}

fn format_oneliner(result: &TestResult, success: bool) -> String {
    let success_msg = match success {
        true => "OK".green(),
        false => "FAILED".red(),
    };

    format!("test {} ... {}", result.name, success_msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_summary_successful() {
        // GIVEN
        let result = TestResult::from_name("mytest");

        // WHEN
        let summary = compute_summary(&result);

        // THEN
        assert_eq!("".to_string(), summary);
    }

    #[test]
    fn test_compute_summary_code_differs() {
        // GIVEN
        let mut result = TestResult::from_name("mytest");
        result.expected_status = 0;
        result.actual_status = 1;

        // WHEN
        let summary = compute_summary(&result);

        // THEN
        assert_eq!(
            formatdoc! {r#"
            {} differs:
            expected: 0
            actual: 1
            "#, "status code".yellow()},
            summary
        );
    }

    #[test]
    fn test_compute_summary_stdout_differs() {
        // GIVEN
        let mut result = TestResult::from_name("mytest");
        result.expected_stdout = "foo".to_string();
        result.actual_stdout = "fou".to_string();

        // WHEN
        let summary = compute_summary(&result);

        // THEN
        let diff = format!("{}{}", "-foo\n".green(), "+fou\n".red());

        assert_eq!(
            formatdoc! {r#"
            {stdout} differs:
            {expected} expected
            {actual} actual
            {diff}"#,
            stdout="stdout".yellow(),
            expected="---".green(),
            actual="+++".red(),
            diff=diff},
            summary
        );
    }

    #[test]
    fn test_compute_summary_stderr_differs() {
        // GIVEN
        let mut result = TestResult::from_name("mytest");
        result.expected_stderr = "foo".to_string();
        result.actual_stderr = "fou".to_string();

        // WHEN
        let summary = compute_summary(&result);

        // THEN
        let diff = format!("{}{}", "-foo\n".green(), "+fou\n".red());

        assert_eq!(
            formatdoc! {r#"
            {stderr} differs:
            {expected} expected
            {actual} actual
            {diff}"#,
            stderr="stderr".yellow(),
            expected="---".green(),
            actual="+++".red(),
            diff=diff},
            summary
        );
    }

    #[test]
    fn test_compute_summary_everything_differs() {
        // GIVEN
        let result = TestResult {
            name: "mytest".into(),
            expected_stdout: "foo".to_string(),
            actual_stdout: "fou".to_string(),
            expected_stderr: "bar".to_string(),
            actual_stderr: "baz".to_string(),
            expected_status: 0,
            actual_status: 1,
        };

        // WHEN
        let summary = compute_summary(&result);

        // THEN
        let stdout_diff = format!("{}{}", "-foo\n".green(), "+fou\n".red());
        let stderr_diff = format!("{}{}", "-bar\n".green(), "+baz\n".red());
        assert_eq!(
            formatdoc! {r#"
                {status_code} differs:
                expected: 0
                actual: 1
                {stdout} differs:
                {expected} expected
                {actual} actual
                {stdout_diff}{stderr} differs:
                {expected} expected
                {actual} actual
                {stderr_diff}"#,
            status_code="status code".yellow(),
            stdout="stdout".yellow(),
            stderr="stderr".yellow(),
            expected="---".green(),
            actual="+++".red(),
            stdout_diff=stdout_diff,
            stderr_diff=stderr_diff},
            summary
        );
    }

    #[test]
    fn test_format_oneliner_success() {
        // GIVEN
        let result = TestResult::from_name("mysuite::mytest");
        let success = true;

        // WHEN
        let oneliner = format_oneliner(&result, success);

        // THEN
        assert_eq!(
            format!("test mysuite::mytest ... {}", "OK".green()),
            oneliner
        );
    }

    #[test]
    fn test_format_oneliner_failure() {
        // GIVEN
        let result = TestResult::from_name("anothersuite::anothertest");
        let success = false;

        // WHEN
        let oneliner = format_oneliner(&result, success);

        // THEN
        assert_eq!(
            format!("test anothersuite::anothertest ... {}", "FAILED".red()),
            oneliner
        );
    }
}

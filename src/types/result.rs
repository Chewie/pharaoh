//! # Test results
//!
//! This module contains the various structs representing the result of a run:
//!
//! * A [TestResult] represents the result of a single test.
//! * A [TestSuiteResult] is a collection of [TestResult]
//! * A [TestReport] is the entirety of all [TestSuiteResult]s
use derive_builder::Builder;
use std::process::Output;

use crate::types::testcase::TestCase;

/// The result of a test run.
///
/// This is usually part of a [TestSuiteResult]
#[derive(Debug, Eq, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct TestResult {
    /// The name of the test
    pub name: String,
    /// The stdout that was expected in the [TestCase]
    #[builder(default)]
    pub expected_stdout: String,
    /// The stdout that was actually obtained from the run
    #[builder(default)]
    pub actual_stdout: String,
    #[builder(default)]
    /// The stderr that was expected in the [TestCase]
    pub expected_stderr: String,
    #[builder(default)]
    /// The stderr that was actually obtained from the run
    pub actual_stderr: String,
    #[builder(default)]
    /// The exit status that was expected in the [TestCase]
    pub expected_status: i32,
    /// The exit status that was actually obtained from the run
    #[builder(default)]
    pub actual_status: i32,
}

/// A collection of [TestResult]s
///
/// This is usually part of a [TestReport]
#[derive(Eq, PartialEq, Debug)]
pub struct TestSuiteResult {
    /// The name of the testsuite
    pub name: String,
    /// The results that are part of that test suite
    pub results: Vec<TestResult>,
}

/// A collection of [TestSuiteResult]s
///
/// This usually represents the entirety of your results
#[derive(Eq, PartialEq, Debug, Default)]
pub struct TestReport {
    /// The testsuites that are part of that report
    pub testsuites: Vec<TestSuiteResult>,
}

impl TestResult {
    /// Construct a [TestResult] from a [TestCase] and an [Output]
    pub fn from_output(testcase: TestCase, output: Output) -> TestResult {
        TestResult {
            name: testcase.name,
            expected_stdout: testcase.stdout,
            actual_stdout: String::from_utf8(output.stdout).unwrap(),
            expected_stderr: testcase.stderr,
            actual_stderr: String::from_utf8(output.stderr).unwrap(),
            expected_status: testcase.status,
            // FIXME: ugly, wait for unix_process_wait_more in future versions
            actual_status: output.status.code().unwrap_or(129),
        }
    }

    /// Calculates whether a [TestResult] is successful or not
    ///
    /// A [TestResult] is deemed successful if the stdout, stderr and exit status match their
    /// expected and actual values.
    pub fn is_successful(&self) -> bool {
        self.expected_status == self.actual_status
            && self.expected_stdout == self.actual_stdout
            && self.expected_stderr == self.actual_stderr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_construct_result_from_output() {
        // GIVEN
        let testcase = TestCase {
            name: "mysuite::mycase".to_string(),
            cmd: "echo -n".to_string(),
            stdin: "my string".to_string(),
            stdout: "expected stdout".to_string(),
            stderr: "expected stderr".to_string(),
            status: 0,
        };

        let output = Output {
            status: ExitStatus::from_raw(1 << 8), // Exit status are bits from 8 to 15
            stdout: "actual stdout".as_bytes().to_vec(),
            stderr: "actual stderr".as_bytes().to_vec(),
        };
        // WHEN
        let result = TestResult::from_output(testcase, output);

        // THEN
        assert_eq!(
            TestResult {
                name: "mysuite::mycase".to_string(),
                expected_stdout: "expected stdout".to_string(),
                expected_stderr: "expected stderr".to_string(),
                actual_stdout: "actual stdout".to_string(),
                actual_stderr: "actual stderr".to_string(),
                expected_status: 0,
                actual_status: 1,
            },
            result
        );
    }

    #[test]
    fn test_is_successful() {
        // GIVEN
        let result = TestResult {
            name: "mytestcase".to_string(),
            expected_stdout: "expected stdout".to_string(),
            expected_stderr: "expected stderr".to_string(),
            actual_stdout: "expected stdout".to_string(),
            actual_stderr: "expected stderr".to_string(),
            expected_status: 0,
            actual_status: 0,
        };
        // WHEN
        let successful = result.is_successful();
        // THEN
        assert_eq!(true, successful);
    }
}

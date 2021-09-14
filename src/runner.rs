use std::process::{Command, Output, Stdio};
use std::io::Write;

use crate::testcase::{TestCase, TestFile};

#[derive(Debug, Eq, PartialEq)]
pub struct TestResult {
    pub name: String,
    pub expected_stdout: String,
    pub actual_stdout: String,
    pub expected_stderr: String,
    pub actual_stderr: String,
    pub expected_status: i32,
    pub actual_status: i32,
}

impl TestResult {
    pub fn from_output(testcase: &TestCase, output: &Output) -> TestResult {
        TestResult {
            name: testcase.name.clone(),
            expected_stdout: testcase.stdout.clone(),
            actual_stdout: String::from_utf8(output.stdout.clone()).unwrap(),
            expected_stderr: testcase.stderr.clone(),
            actual_stderr: String::from_utf8(output.stderr.clone()).unwrap(),
            expected_status: testcase.status,
            // FIXME: ugly, wait for unix_process_wait_more in future versions
            actual_status: output.status.code().unwrap_or(129),
        }
    }

    pub fn is_successful(&self) -> bool {
        self.expected_status == self.actual_status
            && self.expected_stdout == self.actual_stdout
            && self.expected_stderr == self.actual_stderr
    }
}

pub struct TestSuiteResult {
    pub name: String,
    pub results: Vec<TestResult>,
}

pub struct TestReport {
    pub testsuites: Vec<TestSuiteResult>,
}

pub fn run_all_tests(testfiles: &[TestFile]) -> Result<TestReport, std::io::Error> {
    Ok(TestReport {
        testsuites: testfiles
            .iter()
            .map(|testfile| run_testfile(testfile))
            .collect::<Result<_, std::io::Error>>()?,
    })
}

fn run_testcase(testcase: &TestCase) -> Result<Output, std::io::Error> {
    let mut child = Command::new("/bin/sh")
        .args(vec!["-c", &testcase.cmd])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(testcase.stdin.as_bytes())?;
    //drop(child_stdin);

    child.wait_with_output()
}

fn run_testfile(testfile: &TestFile) -> Result<TestSuiteResult, std::io::Error> {
    Ok(TestSuiteResult {
        name: testfile.name.clone(),
        results: testfile
            .tests
            .iter()
            .map(|testcase| Ok(TestResult::from_output(testcase, &run_testcase(testcase)?)))
            .collect::<Result<_, std::io::Error>>()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;

    use pretty_assertions::assert_eq;

    impl TestResult {
        pub fn from_name(name: &str) -> TestResult {
            TestResult {
                name: name.to_string(),
                expected_stdout: "".to_string(),
                actual_stdout: "".to_string(),
                expected_stderr: "".to_string(),
                actual_stderr: "".to_string(),
                expected_status: 0,
                actual_status: 0,
            }
        }
    }

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
        let result = TestResult::from_output(&testcase, &output);

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
        let testcase = TestResult::from_name("mytest");
        // WHEN
        let successful = testcase.is_successful();
        // THEN
        assert_eq!(true, successful);
    }
}

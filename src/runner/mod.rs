use anyhow::Result;

mod executor;

use crate::types::result::{TestReport, TestResult, TestSuiteResult};
use crate::types::testcase::{TestCase, TestSuite, TestSuiteCollection};
use executor::{Executor, SimpleExecutor};

pub struct Runner<E: Executor> {
    executor: E,
}

impl Runner<SimpleExecutor> {
    pub fn new() -> Self {
        Self::with_executor(SimpleExecutor::new())
    }
}

impl<E: Executor> Runner<E> {
    pub fn with_executor(executor: E) -> Self {
        Runner { executor }
    }

    pub fn run_all_tests(&self, collection: TestSuiteCollection) -> Result<TestReport> {
        Ok(TestReport {
            testsuites: collection
                .testsuites
                .into_iter()
                .map(|testsuite| self.run_testsuite(testsuite))
                .collect::<Result<_>>()?,
        })
    }

    fn run_testsuite(&self, testsuite: TestSuite) -> Result<TestSuiteResult> {
        Ok(TestSuiteResult {
            name: testsuite.name.clone(),
            results: testsuite
                .tests
                .into_iter()
                .map(|testcase| self.output_from_testcase(testcase))
                .collect::<Result<_>>()?,
        })
    }

    fn output_from_testcase(&self, testcase: TestCase) -> Result<TestResult> {
        let output = self.executor.execute(&testcase)?;
        Ok(TestResult::from_output(testcase, output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;
    use std::process::Output;

    struct DummyExecutor {
        outputs: RefCell<VecDeque<Result<Output>>>,
    }

    impl DummyExecutor {
        fn new(outputs: Vec<Result<Output>>) -> Self {
            DummyExecutor {
                outputs: RefCell::new(VecDeque::from(outputs)),
            }
        }
    }

    impl Executor for DummyExecutor {
        fn execute(&self, _testcase: &TestCase) -> Result<Output> {
            self.outputs.borrow_mut().pop_front().unwrap()
        }
    }

    #[test]
    fn test_run_all_tests() {
        // GIVEN
        let executor = DummyExecutor::new(vec![
            Ok(Output {
                status: ExitStatus::from_raw(0 << 8), // Exit status are bits from 8 to 15
                stdout: "foo\n".as_bytes().to_vec(),
                stderr: "".as_bytes().to_vec(),
            }),
            Ok(Output {
                status: ExitStatus::from_raw(0 << 8), // Exit status are bits from 8 to 15
                stdout: "bar\n".as_bytes().to_vec(),
                stderr: "".as_bytes().to_vec(),
            }),
            Ok(Output {
                status: ExitStatus::from_raw(0 << 8), // Exit status are bits from 8 to 15
                stdout: "baz\n".as_bytes().to_vec(),
                stderr: "".as_bytes().to_vec(),
            }),
        ]);
        let runner = Runner::with_executor(executor);

        let collection = TestSuiteCollection {
            testsuites: vec![
                TestSuite {
                    name: "mysuite".to_string(),
                    tests: vec![
                        TestCase {
                            name: "mytest".to_string(),
                            cmd: "printf 'foo\n'".to_string(),
                            stdin: "".to_string(),
                            stdout: "foo\n".to_string(),
                            stderr: "".to_string(),
                            status: 0,
                        },
                        TestCase {
                            name: "anothertest".to_string(),
                            cmd: "printf 'bar\n'".to_string(),
                            stdin: "".to_string(),
                            stdout: "bar\n".to_string(),
                            stderr: "".to_string(),
                            status: 0,
                        },
                    ],
                },
                TestSuite {
                    name: "anothersuite".to_string(),
                    tests: vec![TestCase {
                        name: "yetanothertest".to_string(),
                        cmd: "printf 'baz\n'".to_string(),
                        stdin: "".to_string(),
                        stdout: "baz\n".to_string(),
                        stderr: "".to_string(),
                        status: 0,
                    }],
                },
            ],
        };

        // WHEN
        let result = runner.run_all_tests(collection).unwrap();
        // THEN
        assert_eq!(
            TestReport {
                testsuites: vec![
                    TestSuiteResult {
                        name: "mysuite".to_string(),
                        results: vec![
                            TestResult {
                                name: "mytest".to_string(),
                                expected_stdout: "foo\n".to_string(),
                                actual_stdout: "foo\n".to_string(),
                                expected_stderr: "".to_string(),
                                actual_stderr: "".to_string(),
                                expected_status: 0,
                                actual_status: 0
                            },
                            TestResult {
                                name: "anothertest".to_string(),
                                expected_stdout: "bar\n".to_string(),
                                actual_stdout: "bar\n".to_string(),
                                expected_stderr: "".to_string(),
                                actual_stderr: "".to_string(),
                                expected_status: 0,
                                actual_status: 0
                            },
                        ]
                    },
                    TestSuiteResult {
                        name: "anothersuite".to_string(),
                        results: vec![TestResult {
                            name: "yetanothertest".to_string(),
                            expected_stdout: "baz\n".to_string(),
                            actual_stdout: "baz\n".to_string(),
                            expected_stderr: "".to_string(),
                            actual_stderr: "".to_string(),
                            expected_status: 0,
                            actual_status: 0
                        },]
                    },
                ]
            },
            result
        );
    }

    #[test]
    fn test_new_calls_with_executor() {
        // GIVEN
        let executor = SimpleExecutor::new();
        // WHEN
        let runner = Runner::new();
        // THEN
        assert_eq!(executor, runner.executor);
    }
}

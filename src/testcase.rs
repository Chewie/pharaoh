//! # Test cases
//!
//! This module contains the various structs representing the test cases that we want to run:
//!
//! * A [TestCase] represents the specification for a single test.
//! * A [TestSuite] is a collection of TestCases, typically all tests from a given YAML file.
//! * A [TestSuiteCollection] is the entirety of all the TestSuites, typically all YAML files in a
//!   directory.

use anyhow::Result;
pub use serde::{Deserialize, Serialize};
use serde_yaml::Value;

/// The specification for a test run.
///
/// This is usually part of a [TestSuite]
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub cmd: String,
    #[serde(default)]
    pub stdin: String,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub status: i32,
}

/// A collection of TestCases
///
/// This is usually part of a [TestSuiteCollection]
#[derive(Eq, PartialEq, Debug)]
pub struct TestSuite {
    pub name: String,
    pub tests: Vec<TestCase>,
}

impl TestSuite {
    pub fn from_reader(reader: &mut impl std::io::Read, name: String) -> Result<Self> {
        Ok(TestSuite {
            tests: serde_yaml::Deserializer::from_reader(reader)
                .map(|doc| {
                    let value = Value::deserialize(doc)?;
                    let mut test_case: TestCase = serde_yaml::from_value(value)?;
                    test_case.name = format!("{}::{}", name, test_case.name);
                    Ok(test_case)
                })
                .collect::<Result<Vec<TestCase>>>()?,
            name,
        })
    }
}

/// A collection of TestSuites
///
/// This usually represents the entirety of your tests, for example all the YAML files in a given
/// directory.
pub struct TestSuiteCollection {
    pub testsuites: Vec<TestSuite>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;
    use std::io::Cursor;

    #[test]
    fn test_testsuite_from_reader() {
        let mut doc = Cursor::new(indoc! {r#"
            name: cat should work
            cmd: cat
            stdin: |
              this is a line
            stdout: |
             this is a line
            status: 0
        "#});

        let result = TestSuite::from_reader(&mut doc, "mytestsuite".to_string()).unwrap();

        assert_eq!(
            TestSuite {
                name: "mytestsuite".to_string(),
                tests: vec![TestCase {
                    name: "mytestsuite::cat should work".to_string(),
                    cmd: "cat".to_string(),
                    stdin: "this is a line\n".to_string(),
                    stdout: "this is a line\n".to_string(),
                    stderr: "".to_string(),
                    status: 0
                }]
            },
            result
        );
    }
    #[test]
    fn test_testsuite_from_reader_multiple_documents() {
        let mut doc = Cursor::new(indoc! {r#"
            name: a first test
            cmd: echo
            ---
            name: a second test
            cmd: printf
        "#});

        let result = TestSuite::from_reader(&mut doc, "mytestsuite".to_string()).unwrap();

        assert_eq!(
            TestSuite {
                name: "mytestsuite".to_string(),
                tests: vec![
                    TestCase {
                        name: "mytestsuite::a first test".to_string(),
                        cmd: "echo".to_string(),
                        stdin: "".to_string(),
                        stdout: "".to_string(),
                        stderr: "".to_string(),
                        status: 0
                    },
                    TestCase {
                        name: "mytestsuite::a second test".to_string(),
                        cmd: "printf".to_string(),
                        stdin: "".to_string(),
                        stdout: "".to_string(),
                        stderr: "".to_string(),
                        status: 0
                    },
                ]
            },
            result
        );
    }

    #[test]
    fn test_testsuite_from_reader_invalid_yaml() {
        let mut doc = Cursor::new(indoc! {r#"
            foo: bar
        "#});

        let result = TestSuite::from_reader(&mut doc, "testsuite".to_string());

        assert!(result.is_err());
    }
}

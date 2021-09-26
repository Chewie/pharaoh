//! # Test cases
//!
//! This module contains the various structs representing the test cases that we want to run:
//!
//! * A [TestCase] represents the specification for a single test.
//! * A [TestSuite] is a collection of [TestCase]s, typically all tests from a given YAML file.
//! * A [TestSuiteCollection] is the entirety of all the [TestSuite]s, typically all YAML files in a
//!   directory.

pub use serde::{Deserialize, Serialize};

/// The specification for a test run.
///
/// This is usually part of a [TestSuite]
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    /// The name of the test case
    pub name: String,
    /// The command to be executed
    pub cmd: String,
    /// The stdin that will be fed to the command
    #[serde(default)]
    pub stdin: String,
    /// The expected stdout after the test case is executed
    #[serde(default)]
    pub stdout: String,
    /// The expected stderr after the test case is executed
    #[serde(default)]
    pub stderr: String,
    /// The expected exit status after the test case is executed
    #[serde(default)]
    pub status: i32,
}

/// A collection of [TestCase]s
///
/// This is usually part of a [TestSuiteCollection]
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TestSuite {
    /// The name of the testsuite, typically the name of the file containing the test cases
    pub name: String,
    /// The test cases that are part of that test suite
    pub tests: Vec<TestCase>,
}

/// A collection of [TestSuite]s
///
/// This usually represents the entirety of your tests, for example all the YAML files in a given
/// directory.
#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct TestSuiteCollection {
    /// The test suites that are part of that collection
    pub testsuites: Vec<TestSuite>,
}

impl TestSuiteCollection {
    /// Construct a [TestSuiteCollection] from a collection of [TestSuite]s
    pub fn new<I>(testsuites: I) -> Self
    where
        I: IntoIterator<Item = TestSuite>,
    {
        TestSuiteCollection {
            testsuites: testsuites.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_from_testsuites() {
        // GIVEN
        let testsuites = vec![
            TestSuite {
                name: "suite1".to_string(),
                tests: vec![],
            },
            TestSuite {
                name: "suite2".to_string(),
                tests: vec![],
            },
        ];

        // WHEN
        let result = TestSuiteCollection::new(testsuites);

        // THEN
        assert_eq!(
            TestSuiteCollection {
                testsuites: vec![
                    TestSuite {
                        name: "suite1".to_string(),
                        tests: vec![],
                    },
                    TestSuite {
                        name: "suite2".to_string(),
                        tests: vec![],
                    }
                ]
            },
            result
        );
    }
}

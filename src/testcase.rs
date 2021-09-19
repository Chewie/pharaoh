//! # Test cases
//!
//! This module contains the various structs representing the test cases that we want to run:
//!
//! * A [TestCase] represents the specification for a single test.
//! * A [TestSuite] is a collection of TestCases, typically all tests from a given YAML file.
//! * A [TestSuiteCollection] is the entirety of all the TestSuites, typically all YAML files in a
//!   directory.

pub use serde::{Deserialize, Serialize};

/// The specification for a test run.
///
/// This is usually part of a [TestSuite]
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Debug)]
pub struct TestSuite {
    pub name: String,
    pub tests: Vec<TestCase>,
}

/// A collection of TestSuites
///
/// This usually represents the entirety of your tests, for example all the YAML files in a given
/// directory.
pub struct TestSuiteCollection {
    pub testsuites: Vec<TestSuite>,
}

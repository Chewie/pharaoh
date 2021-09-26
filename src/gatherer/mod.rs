//! [TestSuiteCollection] gathering
use crate::types::testcase::TestSuiteCollection;
use anyhow::Result;

pub mod yaml;
pub use yaml::YamlGatherer;

/// A trait to regroup all structs able to gather [TestSuiteCollection]s from somewhere
#[mockall::automock]
pub trait Gatherer {
    /// Gather testcases from an implementation-specific place to produce a [TestSuiteCollection]
    fn gather(&self) -> Result<TestSuiteCollection>;
}

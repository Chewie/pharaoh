use crate::types::testcase::TestSuiteCollection;
use anyhow::Result;

pub mod yaml;
pub use yaml::YamlGatherer;

#[mockall::automock]
pub trait Gatherer {
    fn gather(&self) -> Result<TestSuiteCollection>;
}

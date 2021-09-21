use crate::types::testcase::TestSuiteCollection;
use anyhow::Result;

pub mod yaml;
pub use yaml::YamlGatherer;

pub trait Gatherer {
    fn gather(&self) -> Result<TestSuiteCollection>;
}

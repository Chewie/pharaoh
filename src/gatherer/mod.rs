use crate::types::testcase::TestSuiteCollection;
use anyhow::Result;

pub mod yaml;

pub trait Gatherer {
    fn gather(&self) -> Result<TestSuiteCollection>;
}

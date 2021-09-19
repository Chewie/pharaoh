use crate::testcase::TestSuiteCollection;
use std::error::Error;

pub mod yaml;

pub trait Gatherer {
    fn gather(&self) -> Result<TestSuiteCollection, Box<dyn Error>>;
}

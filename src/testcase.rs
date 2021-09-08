pub use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub struct TestFile {
    pub name: String,
    pub tests: Vec<TestCase>,
}

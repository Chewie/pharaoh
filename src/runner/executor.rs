use anyhow::Result;
use std::io::Write;
use std::process::{Command, Output, Stdio};

use crate::types::testcase::TestCase;

pub trait Executor {
    fn execute(&self, testcase: &TestCase) -> Result<Output>;
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct SimpleExecutor {}

impl SimpleExecutor {
    pub fn new() -> Self {
        SimpleExecutor {}
    }
}

impl Executor for SimpleExecutor {
    fn execute(&self, testcase: &TestCase) -> Result<Output> {
        let mut child = Command::new("/bin/sh")
            .args(vec!["-c", &testcase.cmd])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(testcase.stdin.as_bytes())?;

        Ok(child.wait_with_output()?)
    }
}

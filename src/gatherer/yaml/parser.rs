use anyhow::Result;
use serde::Deserialize;
use serde_yaml::Value;
use std::fs;
use std::path;

use crate::types::testcase::{TestCase, TestSuite};

pub trait Parser {
    fn from_file(&self, path: &path::Path, name: String) -> Result<TestSuite>;
}

#[derive(Eq, PartialEq, Debug, Default)]
pub struct DefaultParser {}

impl DefaultParser {
    pub fn new() -> Self {
        DefaultParser {}
    }

    pub fn from_reader(&self, reader: &mut impl std::io::Read, name: String) -> Result<TestSuite> {
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

impl Parser for DefaultParser {
    fn from_file(&self, path: &path::Path, name: String) -> Result<TestSuite> {
        let mut file = fs::File::open(path)?;

        self.from_reader(&mut file, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;
    use std::io::Cursor;

    #[test]
    fn test_from_reader() {
        // GIVEN
        let mut doc = Cursor::new(indoc! {r#"
            name: cat should work
            cmd: cat
            stdin: |
              this is a line
            stdout: |
             this is a line
            status: 0
        "#});

        let parser = DefaultParser::new();

        // WHEN
        let result = parser
            .from_reader(&mut doc, "mytestsuite".to_string())
            .unwrap();

        // THEN
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
    fn test_from_reader_multiple_documents() {
        // GIVEN
        let mut doc = Cursor::new(indoc! {r#"
            name: a first test
            cmd: echo
            ---
            name: a second test
            cmd: printf
        "#});

        let parser = DefaultParser::new();

        // WHEN
        let result = parser
            .from_reader(&mut doc, "mytestsuite".to_string())
            .unwrap();

        // THEN
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
    fn test_from_reader_invalid_yaml() {
        // GIVEN
        let mut doc = Cursor::new(indoc! {r#"
            foo: bar
        "#});

        let parser = DefaultParser::new();

        // WHEN
        let result = parser.from_reader(&mut doc, "testsuite".to_string());

        // THEN
        assert!(result.is_err());
    }
}

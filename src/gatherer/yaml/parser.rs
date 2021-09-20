use anyhow::Result;
use serde::Deserialize;
use serde_yaml::Value;

use crate::testcase::{TestCase, TestSuite};

pub fn from_reader(reader: &mut impl std::io::Read, name: String) -> Result<TestSuite> {
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

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;
    use std::io::Cursor;

    #[test]
    fn test_from_reader() {
        let mut doc = Cursor::new(indoc! {r#"
            name: cat should work
            cmd: cat
            stdin: |
              this is a line
            stdout: |
             this is a line
            status: 0
        "#});

        let result = from_reader(&mut doc, "mytestsuite".to_string()).unwrap();

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
        let mut doc = Cursor::new(indoc! {r#"
            name: a first test
            cmd: echo
            ---
            name: a second test
            cmd: printf
        "#});

        let result = from_reader(&mut doc, "mytestsuite".to_string()).unwrap();

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
        let mut doc = Cursor::new(indoc! {r#"
            foo: bar
        "#});

        let result = from_reader(&mut doc, "testsuite".to_string());

        assert!(result.is_err());
    }
}

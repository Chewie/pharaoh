//! Implementation of a Gatherer for YAML files
use anyhow::Result;
use std::path;

use crate::gatherer::Gatherer;
use crate::types::testcase::{TestSuite, TestSuiteCollection};

mod parser;
mod utils;
mod walker;

use parser::DefaultParser;
use walker::DefaultWalker;

/// Gather testcase from YAML files in a directory
pub struct YamlGatherer<Parser: parser::Parser, Walker: walker::Walker> {
    search_dir: String,
    parser: Parser,
    walker: Walker,
}

impl YamlGatherer<DefaultParser, DefaultWalker> {
    /// Constructs a new [YamlGatherer]
    pub fn new(search_dir: String) -> Self {
        Self::with_dependencies(search_dir, DefaultParser::new(), DefaultWalker::new())
    }
}

impl<Parser, Walker> YamlGatherer<Parser, Walker>
where
    Parser: parser::Parser,
    Walker: walker::Walker,
{
    fn with_dependencies(search_dir: String, parser: Parser, walker: Walker) -> Self {
        YamlGatherer {
            search_dir,
            parser,
            walker,
        }
    }
    fn get_testsuite_from_path(&self, path: &path::Path) -> Result<TestSuite> {
        let testsuite_name = utils::get_stem(path, &self.search_dir);
        self.parser.parse_file(path, testsuite_name)
    }
}

impl<Parser, Walker> Gatherer for YamlGatherer<Parser, Walker>
where
    Parser: parser::Parser,
    Walker: walker::Walker,
{
    fn gather(&self) -> Result<TestSuiteCollection> {
        let entries = self.walker.walk(&self.search_dir)?;

        // FIXME: a more elegant way to leave early without collecting into a vec?
        let testsuites: Vec<TestSuite> = entries
            .into_iter()
            .map(|path| self.get_testsuite_from_path(&path))
            .collect::<Result<_>>()?;

        Ok(TestSuiteCollection::new(testsuites))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyParser {}

    impl DummyParser {
        fn new() -> Self {
            DummyParser {}
        }
    }

    impl parser::Parser for DummyParser {
        fn parse_file(&self, _path: &path::Path, name: String) -> Result<TestSuite> {
            Ok(TestSuite {
                name,
                tests: vec![],
            })
        }
    }

    use std::path::PathBuf;

    struct DummyWalker {
        dummy_result: Vec<PathBuf>,
    }

    impl DummyWalker {
        fn new(dummy_result: Vec<PathBuf>) -> Self {
            DummyWalker { dummy_result }
        }
    }

    impl walker::Walker for DummyWalker {
        fn walk(&self, _search_dir: &str) -> Result<Vec<PathBuf>> {
            Ok(self.dummy_result.clone())
        }
    }

    #[test]
    fn test_gather() {
        // GIVEN
        let parser = DummyParser::new();
        let walker = DummyWalker::new(vec![PathBuf::from("./foo.yaml")]);
        let gatherer = YamlGatherer::with_dependencies(".".to_string(), parser, walker);

        // WHEN
        let collection = gatherer.gather();

        // THEN
        assert_eq!(
            TestSuiteCollection {
                testsuites: vec![TestSuite {
                    name: "foo".to_string(),
                    tests: vec![]
                }]
            },
            collection.unwrap()
        );
    }

    #[test]
    fn test_new_calls_with_dependencies() {
        // GIVEN
        let parser = DefaultParser::new();
        let walker = DefaultWalker::new();

        // WHEN
        let gatherer = YamlGatherer::new(".".to_string());

        // THEN
        assert_eq!(parser, gatherer.parser);
        assert_eq!(walker, gatherer.walker);
    }
}

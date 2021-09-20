use anyhow::Result;
use std::path;

use crate::gatherer::Gatherer;
use crate::testcase::{TestSuite, TestSuiteCollection};

mod parser;
mod walker;

use parser::DefaultParser;
use walker::DefaultWalker;

pub struct YamlGatherer<Parser: parser::Parser, Walker: walker::Walker> {
    search_dir: String,
    parser: Parser,
    walker: Walker,
}

impl YamlGatherer<DefaultParser, DefaultWalker> {
    pub fn new(search_dir: String) -> Self {
        Self::with_dependencies(search_dir, DefaultParser::new(), DefaultWalker::new())
    }
}

impl<Parser, Walker> YamlGatherer<Parser, Walker>
where
    Parser: parser::Parser,
    Walker: walker::Walker,
{
    pub fn with_dependencies(search_dir: String, parser: Parser, walker: Walker) -> Self {
        YamlGatherer {
            search_dir,
            parser,
            walker,
        }
    }
    fn get_testsuite_from_path(&self, path: &path::Path) -> Result<TestSuite> {
        let testsuite_name = self.get_testsuite_name(path);
        self.parser.from_file(path, testsuite_name)
    }

    fn get_testsuite_name(&self, path: &path::Path) -> String {
        let filename = path.with_extension("");
        let filename = filename.strip_prefix(&self.search_dir).unwrap(); // Cannot fail
        let filename = filename.display().to_string();

        filename
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
            .collect::<Result<_, _>>()?;

        Ok(TestSuiteCollection::new(testsuites.into_iter()))
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
        fn from_file(&self, _path: &path::Path, name: String) -> Result<TestSuite> {
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
    fn test_get_name_with_leading_dot() {
        let gatherer = YamlGatherer::new(".".to_string());
        let path = path::Path::new("./foo.yaml");

        assert_eq!("foo", gatherer.get_testsuite_name(path));
    }

    #[test]
    fn test_get_name_subdir() {
        let gatherer = YamlGatherer::new("foo".to_string());
        let path = path::Path::new("foo/bar.yaml");

        assert_eq!("bar", gatherer.get_testsuite_name(path));
    }

    #[test]
    fn test_get_name_subdir_trailing_slash() {
        let gatherer = YamlGatherer::new("foo/".to_string());
        let path = path::Path::new("foo/bar.yaml");

        assert_eq!("bar", gatherer.get_testsuite_name(path));
    }

    #[test]
    fn test_get_name_sub_yaml() {
        let gatherer = YamlGatherer::new(".".to_string());
        let path = path::Path::new("./foo/bar.yaml");

        assert_eq!("foo/bar", gatherer.get_testsuite_name(path));
    }

    #[test]
    fn test_get_name_sub_yaml_in_subdir() {
        let gatherer = YamlGatherer::new("subdir/".to_string());
        let path = path::Path::new("subdir/foo/bar.yaml");

        assert_eq!("foo/bar", gatherer.get_testsuite_name(path));
    }
}

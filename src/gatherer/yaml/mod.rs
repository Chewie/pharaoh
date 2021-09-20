use anyhow::Result;
use std::marker::PhantomData;
use std::path;

use crate::gatherer::Gatherer;
use crate::testcase::{TestSuite, TestSuiteCollection};

mod parser;
mod walker;

use parser::DefaultParser;
use walker::DefaultWalker;

pub type YamlGatherer = GenericYamlGatherer<DefaultParser, DefaultWalker>;

pub struct GenericYamlGatherer<Parser: parser::Parser, Walker: walker::Walker> {
    search_dir: String,
    _parser_type: PhantomData<Parser>,
    _walker_type: PhantomData<Walker>,
}

impl<Parser, Walker> GenericYamlGatherer<Parser, Walker>
where
    Parser: parser::Parser,
    Walker: walker::Walker,
{
    pub fn new(search_dir: String) -> Self {
        GenericYamlGatherer {
            search_dir,
            _parser_type: PhantomData,
            _walker_type: PhantomData,
        }
    }

    fn get_testsuite_from_path(&self, path: &path::Path) -> Result<TestSuite> {
        let testsuite_name = self.get_testsuite_name(path);
        Parser::from_file(path, testsuite_name)
    }

    fn get_testsuite_name(&self, path: &path::Path) -> String {
        let filename = path.with_extension("");
        let filename = filename.strip_prefix(&self.search_dir).unwrap(); // Cannot fail
        let filename = filename.display().to_string();

        filename
    }
}

impl<Parser, Walker> Gatherer for GenericYamlGatherer<Parser, Walker>
where
    Parser: parser::Parser,
    Walker: walker::Walker,
{
    fn gather(&self) -> Result<TestSuiteCollection> {
        let entries = Walker::walk(&self.search_dir)?;

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

    impl parser::Parser for DummyParser {
        fn from_file(_path: &path::Path, name: String) -> Result<TestSuite> {
            Ok(TestSuite {
                name,
                tests: vec![],
            })
        }
    }

    use std::path::PathBuf;

    struct DummyWalker {}

    impl walker::Walker for DummyWalker {
        fn walk(_search_dir: &str) -> Result<Vec<PathBuf>> {
            Ok(vec![PathBuf::from("./foo.yaml")])
        }
    }

    #[test]
    fn test_gather() {
        // GIVEN
        let gatherer = GenericYamlGatherer::<DummyParser, DummyWalker>::new(".".to_string());

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

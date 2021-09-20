use std::fs;
use std::path;

use globwalk::{DirEntry, GlobWalkerBuilder};

use crate::gatherer::Gatherer;
use crate::testcase::{TestSuite, TestSuiteCollection};
use anyhow::Result;

pub struct YamlGatherer {
    search_dir: String,
}

impl YamlGatherer {
    pub fn new(search_dir: String) -> Self {
        YamlGatherer { search_dir }
    }

    fn get_yamls(&self) -> Result<Vec<DirEntry>> {
        Ok(
            GlobWalkerBuilder::from_patterns(&self.search_dir, &["**/*.yaml", "**/*.yml"])
                .min_depth(1)
                .sort_by(|a, b| a.path().cmp(b.path()))
                .build()?
                .into_iter()
                .filter_map(Result::ok)
                .collect(),
        )
    }

    fn get_testsuite_from_entry(&self, entry: DirEntry) -> Result<TestSuite> {
        let mut file = fs::File::open(entry.path())?;

        let testsuite_name = self.get_testsuite_name(entry.path());

        TestSuite::from_reader(&mut file, testsuite_name)
    }

    fn get_testsuite_name(&self, path: &path::Path) -> String {
        let filename = path.with_extension("");
        let filename = filename.strip_prefix(&self.search_dir).unwrap(); // Cannot fail
        let filename = filename.display().to_string();

        filename
    }
}

impl Gatherer for YamlGatherer {
    fn gather(&self) -> Result<TestSuiteCollection> {
        let entries = self.get_yamls()?;

        let mut result: Vec<TestSuite> = vec![];
        for entry in entries {
            let testsuite = self.get_testsuite_from_entry(entry)?;
            result.push(testsuite);
        }
        Ok(TestSuiteCollection { testsuites: result })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

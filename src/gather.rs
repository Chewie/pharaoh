use std::error::Error;
use std::path;
use std::fs;

use globwalk::{GlobWalkerBuilder, DirEntry};
use serde_yaml::Value;
use serde::Deserialize;

use crate::test_case::{TestCase, TestFile};

pub fn gather_testfiles(search_dir: &str) -> Result<Vec<TestFile>, Box<dyn Error>> {
    let entries = get_yamls(search_dir)?;

    let mut result : Vec<TestFile> = vec!();
    for entry in entries {
        let testfile = get_testfile_from_entry(search_dir, entry)?;
        result.push(testfile);
    }
    Ok(result)
}



fn get_yamls(search_dir: &str) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    Ok(GlobWalkerBuilder::from_patterns(
        search_dir,
        &["**/*.yaml", "**/*.yml"]
    )
        .min_depth(1)
        .build()?
        .into_iter()
        .filter_map(Result::ok)
        .collect())
}


fn get_testfile_from_entry(search_dir: &str, entry : DirEntry) -> Result<TestFile, Box<dyn Error>> {
    let file = fs::File::open(entry.path())?;

    let test_name = get_test_name(search_dir, entry.path());

    let mut result = TestFile{name: test_name, tests: vec!()};

    for document in serde_yaml::Deserializer::from_reader(file) {
        let value = Value::deserialize(document)?;
        let test_case : TestCase = serde_yaml::from_value(value)?;
        result.tests.push(test_case);
    }
    Ok(result)
}


fn get_test_name(search_dir: &str, path: &path::Path) -> String {
    let filename = path.with_extension("");
    let filename = filename.strip_prefix(search_dir).unwrap(); // Cannot fail
    let filename = filename.display().to_string();

    filename
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name_with_leading_dot() {
        let search_dir = ".";
        let path = path::Path::new("./foo.yaml");

        assert_eq!("foo", get_test_name(search_dir, path));
    }

    #[test]
    fn test_get_name_subdir() {
        let search_dir = "foo";
        let path = path::Path::new("foo/bar.yaml");

        assert_eq!("bar", get_test_name(search_dir, path));
    }

    #[test]
    fn test_get_name_subdir_trailing_slash() {
        let search_dir = "foo/";
        let path = path::Path::new("foo/bar.yaml");

        assert_eq!("bar", get_test_name(search_dir, path));
    }

    #[test]
    fn test_get_name_sub_yaml() {
        let search_dir = ".";
        let path = path::Path::new("./foo/bar.yaml");

        assert_eq!("foo/bar", get_test_name(search_dir, path));
    }

    #[test]
    fn test_get_name_sub_yaml_in_subdir() {
        let search_dir = "subdir/";
        let path = path::Path::new("subdir/foo/bar.yaml");

        assert_eq!("foo/bar", get_test_name(search_dir, path));
    }
}

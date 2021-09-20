use std::path;

pub fn get_stem(path: &path::Path, search_dir: &str) -> String {
    let filename = path.with_extension("");
    let filename = filename
        .strip_prefix(search_dir)
        .expect("path is not part of search dir");
    let filename = filename.display().to_string();

    filename
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stub_with_leading_dot() {
        let path = path::Path::new("./foo.yaml");
        let search_dir = ".";

        assert_eq!("foo", get_stem(path, search_dir));
    }

    #[test]
    fn test_get_stub_subdir() {
        let path = path::Path::new("foo/bar.yaml");
        let search_dir = "foo";

        assert_eq!("bar", get_stem(path, search_dir));
    }

    #[test]
    fn test_get_stub_subdir_trailing_slash() {
        let path = path::Path::new("foo/bar.yaml");
        let search_dir = "foo/";

        assert_eq!("bar", get_stem(path, search_dir));
    }

    #[test]
    fn test_get_stub_sub_yaml() {
        let path = path::Path::new("./foo/bar.yaml");
        let search_dir = ".";

        assert_eq!("foo/bar", get_stem(path, search_dir));
    }

    #[test]
    fn test_get_stub_sub_yaml_in_subdir() {
        let path = path::Path::new("subdir/foo/bar.yaml");
        let search_dir = "subdir/";

        assert_eq!("foo/bar", get_stem(path, search_dir));
    }

    #[test]
    #[should_panic(expected = "path is not part of search dir")]
    fn test_get_stub_not_part_of_search_dir() {
        let path = path::Path::new("anotherdir/foo.yaml");
        let search_dir = "mydir/";

        get_stem(path, search_dir);
    }
}

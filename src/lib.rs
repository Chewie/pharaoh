use walkdir::DirEntry;
use std::path;

pub trait DirEntryTrait {
    fn path(&self) -> &path::Path;
}

pub fn is_a_yaml(entry: &impl DirEntryTrait) -> bool {
    let extension = entry.path().extension().unwrap();

    extension == "yaml" || extension == "yml"
}

impl DirEntryTrait for DirEntry {
    fn path(&self) -> &path::Path {
        self.path()
    }
}



#[cfg(test)]
mod tests {

    struct DummyEntry {
        path: &'static str
    }

    impl DummyEntry {
        fn new(path: &'static str) -> Self {
            DummyEntry { path: path }
        }
    }

    impl DirEntryTrait for DummyEntry {
        fn path(&self) -> &path::Path {
            path::Path::new(self.path)
        }
    }

    use super::*;

    #[test]
    fn test_dotyaml_is_a_yaml() {
        let entry = DummyEntry::new("foo.yaml");

        assert_eq!(true, is_a_yaml(&entry));
    }

    #[test]
    fn test_dotyml_is_a_yaml() {
        let entry = DummyEntry::new("foo.yml");

        assert_eq!(true, is_a_yaml(&entry));
    }

    #[test]
    fn test_dotbar_is_not_a_yaml() {
        let entry = DummyEntry::new("foo.bar");

        assert_eq!(false, is_a_yaml(&entry));
    }
}

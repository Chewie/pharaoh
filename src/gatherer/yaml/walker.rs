use anyhow::Result;
use globwalk::GlobWalkerBuilder;
use std::path::PathBuf;

pub trait Walker {
    fn walk(search_dir: &str) -> Result<Vec<PathBuf>>;
}

pub struct DefaultWalker {}

impl Walker for DefaultWalker {
    fn walk(search_dir: &str) -> Result<Vec<PathBuf>> {
        Ok(
            GlobWalkerBuilder::from_patterns(search_dir, &["**/*.yaml", "**/*.yml"])
                .min_depth(1)
                .sort_by(|a, b| a.path().cmp(b.path()))
                .build()?
                .into_iter()
                .filter_map(Result::ok)
                .map(|entry| entry.into_path())
                .collect(),
        )
    }
}

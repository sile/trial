use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct RepositoryOpt {
    #[structopt(long)]
    pub repository_dir: Option<PathBuf>,
}

#[derive(Debug)]
pub struct Repository {
    root_dir: PathBuf,
}

impl Repository {
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }
}

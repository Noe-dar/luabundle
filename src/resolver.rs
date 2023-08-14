use std::path::{Path, PathBuf};

use anyhow::anyhow;

const SUPPORTED_EXTENSIONS: [&str; 1] = ["lua"];

pub fn resolve<P: AsRef<Path>, S: AsRef<str>>(id: S, search_dir: P) -> anyhow::Result<PathBuf> {
    let search_dir = search_dir.as_ref();
    let id = id.as_ref();

    let mut search_paths = SUPPORTED_EXTENSIONS.iter().map(|ext| {
        let mut path = search_dir.join(id);
        path.set_extension(ext);
        path
    });

    search_paths
        .find(|path| path.exists())
        .ok_or(anyhow!("{} does not exist", id))
}

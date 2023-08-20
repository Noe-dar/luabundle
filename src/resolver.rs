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

#[cfg(test)]
mod tests {
    use std::{
        fs::{create_dir, File},
        io,
    };

    use tempfile::{Builder, TempDir};

    use crate::resolver::resolve;

    fn create_tempdir() -> io::Result<TempDir> {
        Builder::new().prefix("resolve_test").tempdir()
    }

    #[test]
    fn test_resolve() -> io::Result<()> {
        let tempdir = create_tempdir()?;

        let base_path = tempdir.path();
        
        let expected_path = base_path.join("test.lua");
        File::create(&expected_path)?;

        assert_eq!(resolve("test", base_path).unwrap(), expected_path);
        Ok(())
    }

    #[test]
    fn test_deep_resolve() -> io::Result<()> {
        let tempdir = create_tempdir()?;

        let base_path = tempdir.path();

        let a_path = base_path.join("a");
        let b_path = a_path.join("b");
        create_dir(&a_path)?;
        create_dir(&b_path)?;

        let expected_path = b_path.join("test.lua");

        File::create(&expected_path)?;

        assert_eq!(resolve("a/b/test", base_path).unwrap(), expected_path);
        Ok(())
    }
}

use std::path::{Path, PathBuf};

use anyhow::bail;
use log::debug;

#[derive(Debug, Clone, Default)]
pub struct Resolver {
    pattern: String,
}

impl Resolver {
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }

    fn compile_pattern(&self, virtual_file_name: &str) -> Vec<PathBuf> {
        self.pattern
            .split(";")
            .map(|pattern| PathBuf::from(pattern.replace("?", virtual_file_name)))
            .collect()
    }

    pub fn resolve<P: AsRef<Path>>(&self, virtual_file_name: &str, dir: P) -> anyhow::Result<PathBuf> {
        let dir = dir.as_ref();
        let patterns = self.compile_pattern(virtual_file_name);
        let path = patterns
            .into_iter()
            .map(|pattern| dir.join(pattern))
            .find(|pattern| pattern.exists());

        match path {
            Some(resolved) => {
                debug!(
                    "module \"{}\" resolved as {:?}",
                    virtual_file_name, resolved
                );
                Ok(resolved)
            }
            None => bail!("{virtual_file_name} not found"),
        }
    }
}

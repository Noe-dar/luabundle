use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    include: Vec<PathBuf>,
    #[serde(default = "default_pattern")]
    pattern: String,
}

fn default_pattern() -> String {
    "?;?.lua".to_string()
}

// very bad code here, will be rewritten soon

pub mod bundler;
pub mod comment_patcher;
pub mod config;
pub mod pattern_builder;
pub mod resolver;
pub mod utils;
pub mod visitors;

use anyhow::Result;
use argh::FromArgs;
use bundler::Bundler;
use std::{fs::File, path::PathBuf};

#[derive(FromArgs, Clone, Debug)]
/// Bundle lua code
pub struct BundlerArgs {
    /// input file path
    #[argh(positional)]
    pub input: PathBuf,

    /// output name
    #[argh(option, short = 'o')]
    pub output: Option<String>,
}

fn main() -> Result<()> {
    env_logger::init();

    let bundler_args = argh::from_env::<BundlerArgs>();

    let file = File::create(bundler_args.output.unwrap_or("bundle.lua".to_string()))?;

    let mut bundler = Bundler::new(file);
    bundler.bundle(bundler_args.input)?;

    Ok(())
}

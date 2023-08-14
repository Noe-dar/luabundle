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

#[derive(FromArgs)]
/// Bundle lua code
pub struct BundlerArgs {
    /// input file
    #[argh(positional)]
    pub input: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();

    let bundler_args = argh::from_env::<BundlerArgs>();

    let file = File::create("bundle.lua")?;

    let mut bundler = Bundler::new(file);
    bundler.bundle(bundler_args.input)?;

    Ok(())
}

pub mod bundler;
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
    /// path to the input file
    #[argh(positional)]
    pub input: PathBuf,

    /// name of the output file
    #[argh(option, short = 'o', default = r#"String::from("bundle.lua")"#)]
    pub output: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let bundler_args = argh::from_env::<BundlerArgs>();

    let file = File::create(bundler_args.output)?;

    let mut bundler = Bundler::new(file);
    bundler.bundle(bundler_args.input)?;

    Ok(())
}

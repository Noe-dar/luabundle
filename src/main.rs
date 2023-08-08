// very bad code here, will be rewritten soon

pub mod bundler;
pub mod comment_patcher;
pub mod config;
pub mod pattern_builder;
pub mod resolver;
pub mod visitors;
pub mod utils;

use anyhow::Result;
use argh::FromArgs;
use bundler::Bundler;
use log::debug;
use pattern_builder::LuaPathBuilder;
use resolver::Resolver;
use std::{
    fs::File,
    path::PathBuf,
};

#[derive(FromArgs)]
/// Bundle lua code
pub struct BundlerArgs {
    /// input file
    #[argh(positional)]
    pub input: PathBuf,

    #[argh(option)]
    /// include dir
    include: Vec<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::init();

    let bundler_args = argh::from_env::<BundlerArgs>();

    let mut path_builder = LuaPathBuilder::new();

    path_builder.add("?");
    path_builder.add("?.lua");

    for include_dir in bundler_args.include {
        path_builder.add(include_dir.join("?.lua").display().to_string());
        path_builder.add(include_dir.join("?").display().to_string());
    }

    let path = path_builder.build();

    debug!("search path: \"{}\"", path);

    let resolver = Resolver::new(
        path
    );
    
    let file = File::create("bundle.lua")?;

    let mut bundler = Bundler::new(file, resolver);
    bundler.bundle(bundler_args.input)?;

    Ok(())
}

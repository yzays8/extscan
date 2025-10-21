#![deny(unsafe_code)]

mod cli;

use clap::Parser as _;

use cli::{Args, EngineType};
use extscan::Config;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.magic_file.is_some() && args.engine != EngineType::Libmagic {
        eprintln!("--magic-file is ignored when using the Magika engine.");
    }

    let engine_type = match args.engine {
        EngineType::Libmagic => extscan::EngineType::LibMagic,
        EngineType::Magika => extscan::EngineType::Magika,
    };
    let config = Config {
        file_path: args.path,
        engine_type,
        magic_file: args.magic_file,
        recursive: args.recursive,
        ignore: args.ignore,
    };

    extscan::App::new(config).run()?;

    Ok(())
}

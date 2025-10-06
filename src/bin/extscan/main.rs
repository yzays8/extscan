#![deny(unsafe_code)]

mod cli;

use clap::Parser as _;

use cli::Args;
use extscan::Config;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = Config {
        files: args.files,
        magic_file: args.magic_file,
        recursive: args.recursive,
        no_summary: args.no_summary,
        yes: args.yes,
        no: args.no,
    };
    extscan::App::new(config).run()?;
    Ok(())
}

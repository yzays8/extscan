use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use crate::{error::Result, scanner};

#[derive(Debug, Clone)]
pub struct Config {
    pub file_path: String,
    pub engine_type: EngineType,
    pub magic_file: Option<String>,
    pub recursive: bool,
    pub ignore: bool,
}

#[derive(Debug, Clone)]
pub enum EngineType {
    LibMagic,
    Magika,
}

#[derive(Debug)]
pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        App { config }
    }

    pub fn run(&self) -> Result<()> {
        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::default_spinner());
        pb.enable_steady_tick(Duration::from_millis(100));

        let summary = scanner::build_scanner(&self.config).scan()?;
        pb.finish_and_clear();

        summary.print();

        Ok(())
    }
}

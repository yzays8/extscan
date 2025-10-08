use crate::{error::Result, scanner};

#[derive(Debug, Clone)]
pub struct Config {
    pub file_path: String,
    pub engine_type: EngineType,
    pub magic_file: Option<String>,
    pub recursive: bool,
    pub no_summary: bool,
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
        let summary = scanner::build_scanner(&self.config).scan()?;

        if !self.config.no_summary {
            summary.print();
        }

        Ok(())
    }
}

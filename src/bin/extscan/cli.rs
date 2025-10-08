use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Input path")]
    pub path: String,

    #[arg(short, long, default_value_t = EngineType::Libmagic, help = "File type detection engine to use")]
    pub engine: EngineType,

    #[arg(long, help = "Use the specified magic file for file type detection")]
    pub magic_file: Option<String>,

    #[arg(short, long, help = "Check files and directories recursively")]
    pub recursive: bool,

    #[arg(long, help = "Suppress summary output after checking")]
    pub no_summary: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum EngineType {
    Libmagic,
    Magika,
}

impl std::fmt::Display for EngineType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EngineType::Libmagic => write!(f, "libmagic"),
            EngineType::Magika => write!(f, "magika"),
        }
    }
}

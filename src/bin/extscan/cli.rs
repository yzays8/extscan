use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Input path")]
    pub file: String,

    #[arg(long, help = "Use the specified magic file for file type detection")]
    pub magic_file: Option<String>,

    #[arg(short, long, help = "Check files and directories recursively")]
    pub recursive: bool,

    #[arg(long, help = "Suppress summary output after checking")]
    pub no_summary: bool,
}

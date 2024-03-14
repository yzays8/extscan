use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Input file(s)")]
    pub files: Vec<String>,

    #[arg(long, help = "Use the specified magic file for file type detection")]
    pub magic_file: Option<String>,

    #[arg(short, long, help = "Treat file extensions as case-sensitive")]
    pub strict: bool,

    #[arg(short, long, help = "Check files and directories recursively")]
    pub recursive: bool,

    #[arg(long, help = "Suppress summary output after checking")]
    pub no_summary: bool,

    #[arg(short, long, conflicts_with = "no", help = "Yes to all")]
    pub yes: bool,

    #[arg(short, long, help = "No to all")]
    pub no: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}

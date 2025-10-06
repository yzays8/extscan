use crate::{error::Result, scanner};

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub magic_file: Option<String>,
    pub recursive: bool,
    pub no_summary: bool,
    pub yes: bool,
    pub no: bool,
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
        let scan_summary = scanner::scan(&self.config)?;

        if !self.config.no_summary {
            scanner::print_summary(&scan_summary);
        }

        loop {
            if self.config.yes {
                scanner::fix_exts(&scan_summary)?;
                break;
            }
            if self.config.no {
                break;
            }

            let mut input = String::new();
            println!("Would you like to fix these mismatched file extensions? [y/n]");
            std::io::stdin().read_line(&mut input)?;
            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => {
                    scanner::fix_exts(&scan_summary)?;
                    break;
                }
                "n" | "no" => break,
                _ => continue,
            }
        }
        Ok(())
    }
}

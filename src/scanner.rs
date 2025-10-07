use std::{collections::HashMap, path::PathBuf};

use colored::Colorize;

use crate::{app::Config, error::Result, magic::LibMagicScanner};

#[derive(Debug)]
pub struct ScanSummary {
    /// <file path, expected extension>
    pub mismatched_files: HashMap<PathBuf, String>,
    pub total_num: usize,
}

impl ScanSummary {
    pub fn print(&self) {
        println!("\nTotal files: {}", self.total_num);
        println!("\nMismatched Files: {}", self.mismatched_files.len());
        for (file_name, expected_ext) in &self.mismatched_files {
            println!(
                "  {} (expected: {})",
                file_name.display(),
                expected_ext.green()
            );
        }
    }
}

pub trait Scanner {
    fn scan(&self) -> Result<ScanSummary>;
}

pub fn build_scanner(config: &Config) -> Result<impl Scanner> {
    let scanner = LibMagicScanner::new(config.clone());
    Ok(scanner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan() {
        let config = Config {
            file_path: "tests/data/".to_string(),
            magic_file: None,
            recursive: false,
            no_summary: false,
        };
        let summary = build_scanner(&config).unwrap().scan().unwrap();
        assert_eq!(summary.total_num, 4);
        assert_eq!(summary.mismatched_files.len(), 3);
    }

    #[test]
    fn scan_recursively() {
        let config = Config {
            file_path: "tests/data/".to_string(),
            magic_file: None,
            recursive: true,
            no_summary: false,
        };
        let summary = build_scanner(&config).unwrap().scan().unwrap();
        assert_eq!(summary.total_num, 9);
        assert_eq!(summary.mismatched_files.len(), 6);
    }
}

use std::{collections::HashMap, path::PathBuf};

use colored::Colorize;

use crate::{
    app::{Config, EngineType},
    error::Result,
    magic::LibMagicScanner,
    magika::MagikaScanner,
};

#[derive(Debug)]
pub struct ScanSummary {
    /// <file path, expected extension(s)>
    pub mismatched_files: HashMap<PathBuf, Vec<String>>,
    pub total_num: usize,
}

impl ScanSummary {
    pub fn print(&self) {
        println!("\nTotal files: {}", self.total_num);
        println!("\nMismatched Files: {}", self.mismatched_files.len());
        for (file_name, expected_exts) in &self.mismatched_files {
            println!(
                "  {} (expected: {})",
                file_name.display(),
                expected_exts.join(", ").green()
            );
        }
    }
}

pub trait Scanner {
    fn scan(&self) -> Result<ScanSummary>;
}

pub fn build_scanner(config: &Config) -> Box<dyn Scanner> {
    match config.engine_type {
        EngineType::LibMagic => Box::new(LibMagicScanner::new(config.clone())),
        EngineType::Magika => Box::new(MagikaScanner::new(config.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan() {
        let config = Config {
            file_path: "tests/data/".to_string(),
            engine_type: EngineType::LibMagic,
            magic_file: None,
            recursive: false,
            no_summary: false,
        };
        let summary = build_scanner(&config).scan().unwrap();
        assert_eq!(summary.total_num, 4);
        assert_eq!(summary.mismatched_files.len(), 3);
    }

    #[test]
    fn scan_recursively() {
        let config = Config {
            file_path: "tests/data/".to_string(),
            engine_type: EngineType::LibMagic,
            magic_file: None,
            recursive: true,
            no_summary: false,
        };
        let summary = build_scanner(&config).scan().unwrap();
        assert_eq!(summary.total_num, 9);
        assert_eq!(summary.mismatched_files.len(), 6);
    }
}

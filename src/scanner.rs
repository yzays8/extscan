use std::collections::HashMap;

use colored::Colorize;

use crate::{app::Config, error::Result, magic::LibMagicScanner};

#[derive(Debug)]
pub struct SummaryInfo {
    pub mismatched_files: HashMap<String, String>,
    pub total_num: usize,
}

impl SummaryInfo {
    pub fn print_summary(&self) {
        println!("\nTotal files: {}", self.total_num);
        println!("\nMismatched Files: {}", self.mismatched_files.len());
        for (file_name, expected_ext) in &self.mismatched_files {
            println!("  {} (expected: {})", file_name, expected_ext.green());
        }
    }
}

pub trait Scanner {
    fn scan(&self) -> Result<SummaryInfo>;
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
        let args = Config {
            file_path: "tests/data/".to_string(),
            magic_file: None,
            recursive: false,
            no_summary: false,
        };
        let summ_info = build_scanner(&args).unwrap().scan().unwrap();
        assert_eq!(summ_info.total_num, 4);
        assert_eq!(summ_info.mismatched_files.len(), 3);
    }

    #[test]
    fn scan_recursively() {
        let args = Config {
            file_path: "tests/data/".to_string(),
            magic_file: None,
            recursive: true,
            no_summary: false,
        };
        let summ_info = build_scanner(&args).unwrap().scan().unwrap();
        assert_eq!(summ_info.total_num, 9);
        assert_eq!(summ_info.mismatched_files.len(), 6);
    }
}

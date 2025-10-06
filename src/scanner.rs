use std::{collections::HashMap, fs, path::Path};

use crate::{
    app::Config,
    detector::{self, FileTypeDetector},
    error::Result,
};

#[derive(Debug)]
pub struct SummaryInfo {
    pub mismatched_files: HashMap<String, String>,
    pub total_num: usize,
    pub empty_num: usize,
    pub unknown_num: usize,
    pub dir_num: usize,
}

pub fn scan(config: &Config) -> Result<SummaryInfo> {
    // <filename, expected_ext>
    let mut mismatched_files: HashMap<String, String> = HashMap::new();
    let detector = detector::get_detector()?;

    let mut total_num = 0;
    let mut empty_num = 0;
    let mut unknown_num = 0;
    let mut dir_num = 0;

    for filename in &config.files {
        let filename = fs::canonicalize(filename)?.to_str().unwrap().to_string();
        let path = Path::new(&filename);
        path.try_exists()?;

        total_num += 1;

        if path.is_dir() {
            println!("[directory] {}", &filename);
            dir_num += 1;
            if config.recursive {
                let files = fs::read_dir(&filename)?
                    .map(|res| res.map(|e| e.path().to_str().unwrap().to_string()))
                    .collect::<std::result::Result<Vec<String>, std::io::Error>>()?;
                let summ_info = scan(&Config {
                    files,
                    magic_file: config.magic_file.clone(),
                    recursive: config.recursive,
                    no_summary: config.no_summary,
                    yes: config.yes,
                    no: config.no,
                })?;
                mismatched_files.extend(summ_info.mismatched_files);
                total_num += summ_info.total_num;
                empty_num += summ_info.empty_num;
                unknown_num += summ_info.unknown_num;
                dir_num += summ_info.dir_num;
            }
            continue;
        }

        if fs::metadata(path)?.len() == 0 {
            println!("[empty]     {}", &filename);
            empty_num += 1;
            continue;
        }

        let e = detector.detect(&filename)?;
        let expected_exts = e.split('/').collect::<Vec<&str>>();
        if expected_exts[0] == "???" {
            println!("[unknown]   {}", &filename);
            unknown_num += 1;
            continue;
        }

        match path.extension() {
            Some(actual_ext) => {
                if !expected_exts.contains(&actual_ext.to_str().unwrap().to_lowercase().as_str()) {
                    println!(
                        "[mismatch]  {} (expected: {} actual: {})",
                        &filename,
                        &expected_exts[0],
                        &actual_ext.to_str().unwrap()
                    );
                    mismatched_files.insert(filename, String::from(expected_exts[0]));
                }
            }
            None => {
                println!(
                    "[mismatch]  {} (expected: {} actual: None)",
                    &filename, &expected_exts[0]
                );
                mismatched_files.insert(filename, String::from(expected_exts[0]));
            }
        }
    }

    Ok(SummaryInfo {
        mismatched_files,
        total_num,
        empty_num,
        unknown_num,
        dir_num,
    })
}

pub fn fix_extensions(mismatched_files: &HashMap<String, String>) -> Result<()> {
    for (filename, expected_ext) in mismatched_files {
        let path = Path::new(&filename);
        let new_filename = path.with_extension(expected_ext);
        fs::rename(path, &new_filename)?;
        println!("Renamed {} to {}", filename, new_filename.to_str().unwrap());
    }
    Ok(())
}

pub fn print_summary(info: &SummaryInfo) {
    println!("\n================ Scan Results ================");
    println!("Total files: {}", info.total_num);
    println!("Empty files: {}", info.empty_num);
    println!("Unknown files: {}", info.unknown_num);
    println!("Directories: {}", info.dir_num);
    println!(
        "Other files: {}",
        info.total_num - info.empty_num - info.unknown_num - info.dir_num
    );
    println!(
        "\nFiles with mismatched extensions: {}",
        info.mismatched_files.len()
    );
    for (filename, expected_ext) in &info.mismatched_files {
        println!("  {} (expected: {})", filename, expected_ext);
    }
    println!("==============================================");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan() {
        let files = fs::read_dir("tests/data/")
            .unwrap()
            .map(|res| res.map(|e| e.path().to_str().unwrap().to_string()))
            .collect::<std::result::Result<Vec<String>, std::io::Error>>()
            .unwrap();
        let args = Config {
            files,
            magic_file: None,
            recursive: false,
            no_summary: false,
            yes: false,
            no: false,
        };
        let summ_info = scan(&args).unwrap();
        assert_eq!(summ_info.total_num, 5);
        assert_eq!(summ_info.empty_num, 1);
        assert_eq!(summ_info.unknown_num, 1);
        assert_eq!(summ_info.dir_num, 1);
        assert_eq!(summ_info.mismatched_files.len(), 2);
    }

    #[test]
    fn test_scan_rec() {
        let files = fs::read_dir("tests/data/")
            .unwrap()
            .map(|res| res.map(|e| e.path().to_str().unwrap().to_string()))
            .collect::<std::result::Result<Vec<String>, std::io::Error>>()
            .unwrap();
        let args = Config {
            files,
            magic_file: None,
            recursive: true,
            no_summary: false,
            yes: false,
            no: false,
        };
        let summ_info = scan(&args).unwrap();
        assert_eq!(summ_info.total_num, 11);
        assert_eq!(summ_info.empty_num, 2);
        assert_eq!(summ_info.unknown_num, 2);
        assert_eq!(summ_info.dir_num, 2);
        assert_eq!(summ_info.mismatched_files.len(), 4);
    }
}

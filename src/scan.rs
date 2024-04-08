use crate::magic::get_exts;
use crate::cli::Args;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct SummaryInfo {
    mismatched_files: HashMap<String, String>,
    total_num: usize,
    empty_num: usize,
    unknown_num: usize,
    dir_num: usize,
}

pub fn scan(args: &Args) -> Result<SummaryInfo, Box<dyn Error>> {
    // <filename, expected_ext>
    let mut mismatched_files: HashMap<String, String> = HashMap::new();

    let mut total_num = 0;
    let mut empty_num = 0;
    let mut unknown_num = 0;
    let mut dir_num = 0;

    for filename in &args.files {
        let filename = fs::canonicalize(filename)?.to_str().unwrap().to_string();
        let path = Path::new(&filename);
        if !path.exists() {
            return Err(format!("{} does not exist", &filename).into());
        }

        total_num += 1;

        if path.is_dir() {
            println!("[directory] {}", &filename);
            dir_num += 1;
            if args.recursive {
                let files = fs::read_dir(&filename)?
                    .map(|res| res.map(|e| e.path().to_str().unwrap().to_string()))
                    .collect::<Result<Vec<String>, std::io::Error>>()?;
                let summ_info = scan(&Args {
                    files,
                    magic_file: args.magic_file.clone(),
                    recursive: args.recursive,
                    no_summary: args.no_summary,
                    yes: args.yes,
                    no: args.no,
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

        let e = get_exts(&filename)?;
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

pub fn fix_exts(info: &SummaryInfo) -> Result<(), Box<dyn Error>> {
    for (filename, expected_ext) in &info.mismatched_files {
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
        "\nFiles with mismatched extensions: {} files",
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
            .collect::<Result<Vec<String>, std::io::Error>>()
            .unwrap();
        let args = Args {
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
            .collect::<Result<Vec<String>, std::io::Error>>()
            .unwrap();
        let args = Args {
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

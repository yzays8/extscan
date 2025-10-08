use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
};

use colored::Colorize;
use ignore::WalkBuilder;
use magika::Session;
use rayon::prelude::*;

use crate::{
    app::Config,
    detector::FileTypeDetector,
    error::{Error, Result},
    scanner::{ScanSummary, Scanner},
};

#[derive(Debug)]
pub struct MagikaDetector {
    session: Session,
}

impl MagikaDetector {
    pub fn build() -> Result<Self> {
        Ok(Self {
            session: Session::new()?,
        })
    }
}

impl FileTypeDetector for MagikaDetector {
    fn detect(&mut self, file_path: &Path) -> Result<Vec<String>> {
        Ok(self
            .session
            .identify_file_sync(file_path)?
            .info()
            .extensions
            .iter()
            .copied()
            .map(|s| s.to_string())
            .collect())
    }
}

#[derive(Debug)]
pub struct MagikaScanner {
    config: Config,
}

impl MagikaScanner {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

thread_local! {
    static DETECTOR: RefCell<Option<MagikaDetector>> = const {RefCell::new(None)};
}

impl Scanner for MagikaScanner {
    fn scan(&self) -> Result<ScanSummary> {
        let paths = WalkBuilder::new(&self.config.file_path)
            .hidden(false)
            .ignore(false)
            .git_ignore(false)
            .parents(false)
            .follow_links(false)
            .max_depth(if self.config.recursive { None } else { Some(1) })
            .build()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_some_and(|t| t.is_file()))
            // Skip empty files.
            .filter(|e| e.metadata().map(|m| m.len()).unwrap_or(0) > 0)
            // Skip files without extensions.
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| !ext.is_empty())
            })
            .map(|e| e.into_path())
            .collect::<Vec<_>>();

        let mismatched_files = paths
            .par_iter()
            .map(|path| -> Result<HashMap<PathBuf, Vec<String>>> {
                DETECTOR.with(|detector_rc| {
                    let mut detector_opt = detector_rc.borrow_mut();
                    if detector_opt.is_none() {
                        *detector_opt = Some(MagikaDetector::build().unwrap());
                    }
                    let detector = detector_opt.as_mut().unwrap();
                    let expected_exts = detector.detect(path)?;
                    let actual_ext = path
                        .extension()
                        .and_then(|e| e.to_str())
                        .ok_or(Error::Other("Failed to get file extension".to_string()))?
                        .to_lowercase();
                    let is_mismatch = !expected_exts.iter().any(|e| e == &actual_ext);
                    if is_mismatch {
                        println!(
                            "{}  {} (expected: {} actual: {})",
                            "[mismatch]".red(),
                            &path.display(),
                            &expected_exts.join(", ").green(),
                            &actual_ext.red(),
                        );
                        let mut mismatched_files = HashMap::new();
                        mismatched_files.insert(path.clone(), expected_exts);
                        return Ok(mismatched_files);
                    }
                    Ok(HashMap::new())
                })
            })
            .try_reduce(
                HashMap::new,
                |mut acc_mismatched, res_mismatched| -> Result<HashMap<PathBuf, Vec<String>>> {
                    acc_mismatched.extend(res_mismatched);
                    Ok(acc_mismatched)
                },
            )?;

        Ok(ScanSummary {
            mismatched_files,
            total_num: paths.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_jpg() {
        let mut detector = MagikaDetector::build().unwrap();
        let expected = "jpg";
        let actual = detector.detect(Path::new("tests/data/jpg.pdf")).unwrap();
        assert!(actual.iter().any(|e| e == expected));
    }
}

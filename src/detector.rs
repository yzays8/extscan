use std::path::Path;

use crate::error::Result;

pub trait FileTypeDetector {
    fn detect(&mut self, file_path: &Path) -> Result<Vec<String>>;
}

use std::path::Path;

use crate::error::Result;

pub trait FileTypeDetector {
    fn detect(&self, file_path: &Path) -> Result<String>;
}

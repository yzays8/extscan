use crate::{error::Result, magic::LibMagicDetector};

pub trait FileTypeDetector {
    fn detect(&self, filename: &str) -> Result<String>;
}

pub fn get_detector() -> Result<impl FileTypeDetector> {
    let detector = LibMagicDetector::build()?;
    Ok(detector)
}

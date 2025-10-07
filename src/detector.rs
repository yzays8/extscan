use crate::{app::Config, error::Result, magic::LibMagicDetector};

pub trait FileTypeDetector {
    fn detect(&self, filename: &str) -> Result<String>;
}

pub fn build_detector(config: &Config) -> Result<impl FileTypeDetector> {
    let detector = LibMagicDetector::build(config.clone())?;
    Ok(detector)
}

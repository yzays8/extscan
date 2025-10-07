use crate::error::Result;

pub trait FileTypeDetector {
    fn detect(&self, filename: &str) -> Result<String>;
}

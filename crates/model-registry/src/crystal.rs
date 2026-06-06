use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};

pub struct CrystalImage {
    path: PathBuf,
}

impl CrystalImage {
    pub fn open(path: &Path) -> Result<Self> {
        if !path.exists() {
            anyhow::bail!("Crystal image not found: {}", path.display());
        }
        Ok(Self { path: path.to_path_buf() })
    }

    pub fn read_file(&self, inner_path: &str) -> Result<Vec<u8>> {
        // In a real implementation, this would mount the squashfs and read the file.
        // For now, assume the model.bin is directly in the same directory.
        let full = self.path.parent().unwrap().join(inner_path);
        Ok(std::fs::read(full)?)
    }
}

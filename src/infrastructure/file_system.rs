use async_trait::async_trait;
use std::fs;
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use crate::domain::{DomainError, InstallationRepository};

pub struct FileSystemRepository;

impl FileSystemRepository {
    pub fn new() -> Self {
        Self
    }
    
    fn extract_vsix(&self, vsix_data: &[u8], target_dir: &Path) -> Result<(), DomainError> {
        let cursor = std::io::Cursor::new(vsix_data);
        let mut archive = ZipArchive::new(cursor)
            .map_err(|e| DomainError::InvalidExtensionFormat(e.to_string()))?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| DomainError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
            
            let outpath = target_dir.join(file.name());
            
            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(parent) = outpath.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut outfile = fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl InstallationRepository for FileSystemRepository {
    async fn install_vscode(&self, vsix_data: &[u8], extension_id: &str) -> Result<(), DomainError> {
        let extensions_dir = self.get_vscode_extensions_dir()?;
        let extension_dir = extensions_dir.join(extension_id);
        
        if extension_dir.exists() {
            fs::remove_dir_all(&extension_dir)?;
        }
        
        fs::create_dir_all(&extension_dir)?;
        self.extract_vsix(vsix_data, &extension_dir)?;
        
        Ok(())
    }
    
    async fn install_cursor(&self, vsix_data: &[u8], extension_id: &str) -> Result<(), DomainError> {
        let extensions_dir = self.get_cursor_extensions_dir()?;
        let extension_dir = extensions_dir.join(extension_id);
        
        if extension_dir.exists() {
            fs::remove_dir_all(&extension_dir)?;
        }
        
        fs::create_dir_all(&extension_dir)?;
        self.extract_vsix(vsix_data, &extension_dir)?;
        
        Ok(())
    }
    
    fn get_vscode_extensions_dir(&self) -> Result<PathBuf, DomainError> {
        let home = dirs::home_dir()
            .ok_or_else(|| DomainError::DirectoryNotFound("Home directory not found".to_string()))?;
        
        let extensions_dir = if cfg!(target_os = "windows") {
            home.join(".vscode").join("extensions")
        } else if cfg!(target_os = "macos") {
            home.join(".vscode").join("extensions")
        } else {
            home.join(".vscode").join("extensions")
        };
        
        if !extensions_dir.exists() {
            fs::create_dir_all(&extensions_dir)?;
        }
        
        Ok(extensions_dir)
    }
    
    fn get_cursor_extensions_dir(&self) -> Result<PathBuf, DomainError> {
        let home = dirs::home_dir()
            .ok_or_else(|| DomainError::DirectoryNotFound("Home directory not found".to_string()))?;
        
        let extensions_dir = if cfg!(target_os = "windows") {
            home.join("AppData").join("Roaming").join("Cursor").join("User").join("extensions")
        } else if cfg!(target_os = "macos") {
            home.join("Library").join("Application Support").join("Cursor").join("User").join("extensions")
        } else {
            home.join(".config").join("Cursor").join("User").join("extensions")
        };
        
        if !extensions_dir.exists() {
            fs::create_dir_all(&extensions_dir)?;
        }
        
        Ok(extensions_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_extensions_directory_paths() {
        let repo = FileSystemRepository::new();
        
        let vscode_dir = repo.get_vscode_extensions_dir();
        assert!(vscode_dir.is_ok());
        
        let cursor_dir = repo.get_cursor_extensions_dir();
        assert!(cursor_dir.is_ok());
    }
}
use crate::domain::{
    DomainError, IdeType, InstallationDetector, InstallationExecutor, InstallationMethod,
    InstallationStrategy,
};
use async_trait::async_trait;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use zip::ZipArchive;

/// Default implementation of the installation detector
pub struct SystemInstallationDetector;

impl SystemInstallationDetector {
    pub fn new() -> Self {
        Self
    }

    /// Checks if a command exists in the system PATH
    fn is_command_available(&self, command: &str) -> Option<PathBuf> {
        which::which(command).ok()
    }

    /// Gets the extensions directory for file system installation
    fn get_extensions_directory(&self, ide_type: &IdeType) -> Result<PathBuf, DomainError> {
        let home = dirs::home_dir().ok_or_else(|| {
            DomainError::DirectoryNotFound("Home directory not found".to_string())
        })?;

        let extensions_dir = match ide_type {
            IdeType::VsCode => home.join(".vscode").join("extensions"),
            IdeType::Cursor => {
                if cfg!(target_os = "windows") {
                    home.join("AppData")
                        .join("Roaming")
                        .join("Cursor")
                        .join("User")
                        .join("extensions")
                } else if cfg!(target_os = "macos") {
                    home.join("Library")
                        .join("Application Support")
                        .join("Cursor")
                        .join("User")
                        .join("extensions")
                } else {
                    home.join(".config")
                        .join("Cursor")
                        .join("User")
                        .join("extensions")
                }
            }
        };

        Ok(extensions_dir)
    }
}

impl Default for SystemInstallationDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InstallationDetector for SystemInstallationDetector {
    async fn detect_method(&self, ide_type: &IdeType) -> Result<InstallationMethod, DomainError> {
        // First, try to find the CLI command in PATH
        if let Some(command_path) = self.is_command_available(ide_type.command_name()) {
            return Ok(InstallationMethod::CliCommand { command_path });
        }

        // Fallback to file system installation
        let extensions_dir = self.get_extensions_directory(ide_type)?;
        Ok(InstallationMethod::FileSystem { extensions_dir })
    }
}

#[async_trait]
impl InstallationDetector for &SystemInstallationDetector {
    async fn detect_method(&self, ide_type: &IdeType) -> Result<InstallationMethod, DomainError> {
        (**self).detect_method(ide_type).await
    }
}

/// Default implementation of the installation executor
pub struct SystemInstallationExecutor;

impl SystemInstallationExecutor {
    pub fn new() -> Self {
        Self
    }

    /// Installs by extracting to file system
    async fn install_via_filesystem(
        &self,
        extensions_dir: &Path,
        extension_id: &str,
        vsix_data: &[u8],
    ) -> Result<(), DomainError> {
        let target_dir = extensions_dir.join(extension_id);

        // Remove existing installation if present
        if target_dir.exists() {
            fs::remove_dir_all(&target_dir)?;
        }

        // Create target directory
        fs::create_dir_all(&target_dir)?;

        // Extract VSIX archive
        self.extract_vsix(vsix_data, &target_dir)?;

        Ok(())
    }

    /// Extracts VSIX archive to target directory
    fn extract_vsix(&self, vsix_data: &[u8], target_dir: &Path) -> Result<(), DomainError> {
        let cursor = std::io::Cursor::new(vsix_data);
        let mut archive = ZipArchive::new(cursor)
            .map_err(|e| DomainError::InvalidExtensionFormat(e.to_string()))?;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| DomainError::IoError(std::io::Error::other(e)))?;

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

    /// Saves VSIX data to a temporary file for CLI installation
    async fn save_vsix_to_temp(
        &self,
        extension_id: &str,
        vsix_data: &[u8],
    ) -> Result<PathBuf, DomainError> {
        let temp_dir = std::env::temp_dir();
        let vsix_filename = format!("{}.vsix", extension_id.replace('.', "-"));
        let vsix_path = temp_dir.join(vsix_filename);

        let mut file = fs::File::create(&vsix_path)?;
        file.write_all(vsix_data)?;

        Ok(vsix_path)
    }
}

impl Default for SystemInstallationExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InstallationExecutor for SystemInstallationExecutor {
    async fn execute(
        &self,
        strategy: &InstallationStrategy,
        extension_id: &str,
        vsix_data: &[u8],
    ) -> Result<(), DomainError> {
        match &strategy.method {
            InstallationMethod::CliCommand { command_path } => {
                // Save VSIX to temporary file
                let vsix_path = self.save_vsix_to_temp(extension_id, vsix_data).await?;

                // Install using CLI command with the VSIX file
                let result = Command::new(command_path)
                    .arg("--install-extension")
                    .arg(&vsix_path)
                    .output()
                    .map_err(DomainError::IoError)?;

                // Clean up temporary file
                let _ = fs::remove_file(&vsix_path);

                if !result.status.success() {
                    let error_msg = String::from_utf8_lossy(&result.stderr);
                    return Err(DomainError::InstallationFailed(format!(
                        "Failed to install extension via CLI: {}",
                        error_msg
                    )));
                }

                Ok(())
            }
            InstallationMethod::FileSystem { extensions_dir } => {
                // Ensure extensions directory exists
                if !extensions_dir.exists() {
                    fs::create_dir_all(extensions_dir)?;
                }

                self.install_via_filesystem(extensions_dir, extension_id, vsix_data)
                    .await
            }
        }
    }
}

#[async_trait]
impl InstallationExecutor for &SystemInstallationExecutor {
    async fn execute(
        &self,
        strategy: &InstallationStrategy,
        extension_id: &str,
        vsix_data: &[u8],
    ) -> Result<(), DomainError> {
        (**self).execute(strategy, extension_id, vsix_data).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_extensions_directory_paths() {
        let detector = SystemInstallationDetector::new();

        // Test VSCode directory
        let vscode_dir = detector.get_extensions_directory(&IdeType::VsCode);
        assert!(vscode_dir.is_ok());
        let path = vscode_dir.unwrap();
        assert!(path.to_string_lossy().contains(".vscode"));
        assert!(path.to_string_lossy().contains("extensions"));

        // Test Cursor directory
        let cursor_dir = detector.get_extensions_directory(&IdeType::Cursor);
        assert!(cursor_dir.is_ok());
        let path = cursor_dir.unwrap();
        assert!(path.to_string_lossy().contains("Cursor"));
        assert!(path.to_string_lossy().contains("extensions"));
    }

    #[tokio::test]
    async fn test_save_vsix_to_temp() {
        let executor = SystemInstallationExecutor::new();
        let test_data = b"test vsix data";
        let extension_id = "test.extension";

        let result = executor.save_vsix_to_temp(extension_id, test_data).await;
        assert!(result.is_ok());

        let path = result.unwrap();
        assert!(path.exists());
        assert!(path.to_string_lossy().contains("test-extension.vsix"));

        // Clean up
        let _ = fs::remove_file(path);
    }

    #[tokio::test]
    async fn test_extract_vsix_with_valid_zip() {
        use std::io::Write;
        use zip::ZipWriter;
        use zip::write::FileOptions;

        let executor = SystemInstallationExecutor::new();

        // Create a valid ZIP archive in memory
        let mut zip_data = Vec::new();
        {
            let mut zip = ZipWriter::new(std::io::Cursor::new(&mut zip_data));
            zip.start_file::<_, ()>("package.json", FileOptions::default())
                .unwrap();
            zip.write_all(b"{\"name\": \"test\"}").unwrap();
            zip.finish().unwrap();
        }

        // Create temporary directory for extraction
        let temp_dir = TempDir::new().unwrap();
        let target_dir = temp_dir.path().join("test_extension");

        // Extract the ZIP
        let result = executor.extract_vsix(&zip_data, &target_dir);
        assert!(result.is_ok());

        // Verify extraction
        let package_json = target_dir.join("package.json");
        assert!(package_json.exists());
    }
}

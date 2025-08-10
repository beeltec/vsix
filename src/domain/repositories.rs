use crate::domain::entities::{Extension, SearchResult};
use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait ExtensionRepository {
    async fn search(
        &self,
        query: &str,
        marketplace_url: Option<&str>,
    ) -> Result<SearchResult, DomainError>;
    async fn get_extension(
        &self,
        id: &str,
        marketplace_url: Option<&str>,
    ) -> Result<Extension, DomainError>;
    async fn download(
        &self,
        extension: &Extension,
        target_platform: Option<&str>,
    ) -> Result<Vec<u8>, DomainError>;
}

#[async_trait]
pub trait InstallationRepository {
    async fn install_vscode(&self, vsix_data: &[u8], extension_id: &str)
    -> Result<(), DomainError>;
    async fn install_cursor(&self, vsix_data: &[u8], extension_id: &str)
    -> Result<(), DomainError>;

    /// Gets the `VSCode` extensions directory path
    ///
    /// # Errors
    ///
    /// Returns an error if the home directory cannot be determined
    fn get_vscode_extensions_dir(&self) -> Result<std::path::PathBuf, DomainError>;

    /// Gets the Cursor extensions directory path
    ///
    /// # Errors
    ///
    /// Returns an error if the home directory cannot be determined
    fn get_cursor_extensions_dir(&self) -> Result<std::path::PathBuf, DomainError>;
}

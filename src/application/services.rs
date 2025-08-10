use crate::application::use_cases::{InstallExtensionUseCase, SearchExtensionUseCase};
use crate::domain::{DomainError, ExtensionRepository, InstallationRepository};
use crate::infrastructure::{FileSystemRepository, MarketplaceClient};

pub struct ApplicationService {
    marketplace_client: MarketplaceClient,
    file_system_repo: FileSystemRepository,
}

impl Default for ApplicationService {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationService {
    #[must_use]
    pub fn new() -> Self {
        Self {
            marketplace_client: MarketplaceClient::new(),
            file_system_repo: FileSystemRepository::new(),
        }
    }

    /// Searches for extensions in the marketplace
    ///
    /// # Errors
    ///
    /// Returns an error if the query is empty or if the search fails
    pub async fn search_extensions(
        &self,
        query: &str,
        marketplace_url: Option<&str>,
    ) -> Result<crate::domain::SearchResult, DomainError> {
        let use_case = SearchExtensionUseCase::new(&self.marketplace_client);
        use_case.execute(query, marketplace_url).await
    }

    /// Installs an extension
    ///
    /// # Errors
    ///
    /// Returns an error if the extension ID is invalid, extension not found, or installation fails
    pub async fn install_extension(
        &self,
        extension_id: &str,
        use_cursor: bool,
        marketplace_url: Option<&str>,
    ) -> Result<(), DomainError> {
        let use_case =
            InstallExtensionUseCase::new(&self.marketplace_client, &self.file_system_repo);
        use_case
            .execute(extension_id, use_cursor, marketplace_url)
            .await
    }

    /// Downloads an extension to a file
    ///
    /// # Errors
    ///
    /// Returns an error if the extension is not found or download fails
    pub async fn download_extension(
        &self,
        extension_id: &str,
        output_dir: &str,
        marketplace_url: Option<&str>,
    ) -> Result<std::path::PathBuf, DomainError> {
        use std::fs;
        use std::path::Path;

        let extension = self
            .marketplace_client
            .get_extension(extension_id, marketplace_url)
            .await?;
        let vsix_data = self.marketplace_client.download(&extension, None).await?;

        let output_path = Path::new(output_dir);
        if !output_path.exists() {
            fs::create_dir_all(output_path)?;
        }

        let file_name = format!("{}-{}.vsix", extension_id, extension.version);
        let file_path = output_path.join(file_name);

        fs::write(&file_path, vsix_data)?;

        Ok(file_path)
    }
}

impl ExtensionRepository for &MarketplaceClient {
    fn search<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        query: &'life1 str,
        marketplace_url: Option<&'life2 str>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<crate::domain::SearchResult, DomainError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        (**self).search(query, marketplace_url)
    }

    fn get_extension<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        id: &'life1 str,
        marketplace_url: Option<&'life2 str>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<crate::domain::Extension, DomainError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        (**self).get_extension(id, marketplace_url)
    }

    fn download<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        extension: &'life1 crate::domain::Extension,
        target_platform: Option<&'life2 str>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Vec<u8>, DomainError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        (**self).download(extension, target_platform)
    }
}

impl InstallationRepository for &FileSystemRepository {
    fn install_vscode<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        vsix_data: &'life1 [u8],
        extension_id: &'life2 str,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<(), DomainError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        (**self).install_vscode(vsix_data, extension_id)
    }

    fn install_cursor<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        vsix_data: &'life1 [u8],
        extension_id: &'life2 str,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<(), DomainError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        (**self).install_cursor(vsix_data, extension_id)
    }

    fn get_vscode_extensions_dir(&self) -> Result<std::path::PathBuf, DomainError> {
        (**self).get_vscode_extensions_dir()
    }

    fn get_cursor_extensions_dir(&self) -> Result<std::path::PathBuf, DomainError> {
        (**self).get_cursor_extensions_dir()
    }
}

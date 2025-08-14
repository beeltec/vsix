use crate::application::install_use_case::ImprovedInstallExtensionUseCase;
use crate::application::use_cases::{InstallExtensionUseCase, SearchExtensionUseCase};
use crate::domain::{DomainError, ExtensionRepository, InstallationRepository};
use crate::infrastructure::{
    FileSystemRepository, MarketplaceClient, SystemInstallationDetector, SystemInstallationExecutor,
};

/// Main application service that provides high-level operations for extension management.
/// 
/// This service acts as a facade to the application's use cases, coordinating between
/// the domain layer and infrastructure services.
/// 
/// # Examples
/// 
/// ```no_run
/// use vsix::application::ApplicationService;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let service = ApplicationService::new();
///     
///     // Search for Python extensions
///     let results = service.search_extensions("python", None).await?;
///     
///     // Install the Python extension to VSCode
///     service.install_extension("ms-python.python", false, None).await?;
///     
///     // Install an extension to Cursor instead
///     service.install_extension("rust-lang.rust-analyzer", true, None).await?;
///     
///     Ok(())
/// }
/// ```
pub struct ApplicationService {
    marketplace_client: MarketplaceClient,
    file_system_repo: FileSystemRepository,
    installation_detector: SystemInstallationDetector,
    installation_executor: SystemInstallationExecutor,
}

impl Default for ApplicationService {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationService {
    /// Creates a new instance of the ApplicationService with default configurations.
    /// 
    /// This constructor initializes all necessary infrastructure components:
    /// - Marketplace client for searching and downloading extensions
    /// - File system repository for direct installation
    /// - Installation detector for finding CLI tools
    /// - Installation executor for running CLI commands
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use vsix::application::ApplicationService;
    /// 
    /// let service = ApplicationService::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            marketplace_client: MarketplaceClient::new(),
            file_system_repo: FileSystemRepository::new(),
            installation_detector: SystemInstallationDetector::new(),
            installation_executor: SystemInstallationExecutor::new(),
        }
    }

    /// Searches for extensions in the marketplace.
    ///
    /// # Arguments
    /// 
    /// * `query` - The search query string (e.g., "python", "rust", "vim")
    /// * `marketplace_url` - Optional custom marketplace URL (defaults to VSCode marketplace)
    /// 
    /// # Returns
    /// 
    /// Returns a `SearchResult` containing matching extensions and total count.
    /// 
    /// # Errors
    ///
    /// Returns a `DomainError` if:
    /// - The query is empty
    /// - Network request fails
    /// - Marketplace returns invalid data
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use vsix::application::ApplicationService;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let service = ApplicationService::new();
    /// let results = service.search_extensions("rust", None).await?;
    /// 
    /// for extension in &results.extensions {
    ///     println!("{} by {}", extension.display_name, extension.publisher);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_extensions(
        &self,
        query: &str,
        marketplace_url: Option<&str>,
    ) -> Result<crate::domain::SearchResult, DomainError> {
        let use_case = SearchExtensionUseCase::new(&self.marketplace_client);
        use_case.execute(query, marketplace_url).await
    }

    /// Installs an extension using the legacy method (direct filesystem extraction).
    /// 
    /// This method bypasses CLI tools and directly extracts the VSIX archive to the
    /// extensions directory. Consider using `install_extension` instead for better
    /// compatibility with CLI tools.
    /// 
    /// # Arguments
    /// 
    /// * `extension_id` - The extension ID in format "publisher.name"
    /// * `use_cursor` - If `true`, installs to Cursor; if `false`, installs to VSCode
    /// * `marketplace_url` - Optional custom marketplace URL
    ///
    /// # Errors
    ///
    /// Returns a `DomainError` if:
    /// - Extension ID format is invalid
    /// - Extension is not found in the marketplace
    /// - Installation directory cannot be accessed
    /// - VSIX extraction fails
    pub async fn install_extension_legacy(
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

    /// Installs an extension using automatic method detection.
    /// 
    /// This method automatically detects if `code` or `cursor` CLI commands are available
    /// in the system PATH and uses them for installation. If CLI tools are not available,
    /// it falls back to direct filesystem extraction.
    ///
    /// # Arguments
    /// 
    /// * `extension_id` - The extension ID in format "publisher.name" (e.g., "ms-python.python")
    /// * `use_cursor` - If `true`, installs to Cursor; if `false`, installs to VSCode
    /// * `marketplace_url` - Optional custom marketplace URL
    /// 
    /// # Installation Methods
    /// 
    /// 1. **CLI Installation** (preferred): Uses `code --install-extension` or `cursor --install-extension`
    /// 2. **Filesystem Installation** (fallback): Extracts VSIX directly to extensions directory
    /// 
    /// # Errors
    ///
    /// Returns a `DomainError` if:
    /// - Extension ID format is invalid (must be "publisher.name")
    /// - Extension is not found in the marketplace
    /// - Download fails
    /// - Installation fails
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use vsix::application::ApplicationService;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let service = ApplicationService::new();
    /// 
    /// // Install to VSCode
    /// service.install_extension("rust-lang.rust-analyzer", false, None).await?;
    /// 
    /// // Install to Cursor
    /// service.install_extension("ms-python.python", true, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn install_extension(
        &self,
        extension_id: &str,
        use_cursor: bool,
        marketplace_url: Option<&str>,
    ) -> Result<(), DomainError> {
        let use_case = ImprovedInstallExtensionUseCase::new(
            &self.marketplace_client,
            &self.installation_detector,
            &self.installation_executor,
        );
        use_case
            .execute(extension_id, use_cursor, marketplace_url)
            .await
    }

    /// Downloads an extension as a VSIX file to the specified directory.
    /// 
    /// This method downloads the extension package without installing it, useful for:
    /// - Offline installation
    /// - Archiving extensions
    /// - Manual distribution
    /// 
    /// # Arguments
    /// 
    /// * `extension_id` - The extension ID in format "publisher.name"
    /// * `output_dir` - Directory path where the VSIX file will be saved
    /// * `marketplace_url` - Optional custom marketplace URL
    /// 
    /// # Returns
    /// 
    /// Returns the full path to the downloaded VSIX file.
    /// 
    /// # Errors
    ///
    /// Returns a `DomainError` if:
    /// - Extension is not found in the marketplace
    /// - Download fails due to network issues
    /// - Output directory cannot be created
    /// - File cannot be written to disk
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use vsix::application::ApplicationService;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let service = ApplicationService::new();
    /// 
    /// // Download extension to current directory
    /// let path = service.download_extension("rust-lang.rust-analyzer", "./downloads", None).await?;
    /// println!("Downloaded to: {}", path.display());
    /// # Ok(())
    /// # }
    /// ```
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

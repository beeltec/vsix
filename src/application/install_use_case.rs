use crate::domain::{
    DomainError, ExtensionRepository, IdeType, InstallationDetector, InstallationExecutor,
    InstallationStrategy,
};

/// Use case for installing extensions with improved architecture
pub struct ImprovedInstallExtensionUseCase<E, D, X>
where
    E: ExtensionRepository,
    D: InstallationDetector,
    X: InstallationExecutor,
{
    extension_repo: E,
    detector: D,
    executor: X,
}

impl<E, D, X> ImprovedInstallExtensionUseCase<E, D, X>
where
    E: ExtensionRepository,
    D: InstallationDetector,
    X: InstallationExecutor,
{
    pub fn new(extension_repo: E, detector: D, executor: X) -> Self {
        Self {
            extension_repo,
            detector,
            executor,
        }
    }

    /// Executes the install use case with automatic method detection
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Extension ID format is invalid
    /// - Extension is not found
    /// - Installation method cannot be detected
    /// - Installation fails
    pub async fn execute(
        &self,
        extension_id: &str,
        use_cursor: bool,
        marketplace_url: Option<&str>,
    ) -> Result<(), DomainError> {
        // Validate extension ID format
        self.validate_extension_id(extension_id)?;

        // Determine IDE type
        let ide_type = if use_cursor {
            IdeType::Cursor
        } else {
            IdeType::VsCode
        };

        // Detect installation method
        let method = self.detector.detect_method(&ide_type).await?;
        let strategy = InstallationStrategy::new(ide_type, method);

        // Get extension metadata
        let extension = self
            .extension_repo
            .get_extension(extension_id, marketplace_url)
            .await?;

        // Download VSIX data
        let vsix_data = self.extension_repo.download(&extension, None).await?;

        // Execute installation
        self.executor
            .execute(&strategy, &extension.unique_identifier(), &vsix_data)
            .await
    }

    /// Validates the extension ID format
    fn validate_extension_id(&self, extension_id: &str) -> Result<(), DomainError> {
        let parts: Vec<&str> = extension_id.split('.').collect();
        if parts.len() != 2 {
            return Err(DomainError::InvalidExtensionFormat(
                "Extension ID must be in format 'publisher.name'".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Extension, InstallationMethod, SearchResult};
    use async_trait::async_trait;
    use std::path::PathBuf;

    struct MockExtensionRepo {
        should_fail: bool,
    }

    #[async_trait]
    impl ExtensionRepository for MockExtensionRepo {
        async fn search(
            &self,
            _query: &str,
            _marketplace_url: Option<&str>,
        ) -> Result<SearchResult, DomainError> {
            unimplemented!()
        }

        async fn get_extension(
            &self,
            _id: &str,
            _marketplace_url: Option<&str>,
        ) -> Result<Extension, DomainError> {
            if self.should_fail {
                Err(DomainError::ExtensionNotFound("test".to_string()))
            } else {
                Ok(Extension {
                    id: "1".to_string(),
                    name: "test".to_string(),
                    publisher: "publisher".to_string(),
                    version: "1.0.0".to_string(),
                    display_name: "Test Extension".to_string(),
                    description: Some("Test description".to_string()),
                    downloads: 100,
                })
            }
        }

        async fn download(
            &self,
            _extension: &Extension,
            _target_platform: Option<&str>,
        ) -> Result<Vec<u8>, DomainError> {
            if self.should_fail {
                Err(DomainError::NetworkError("download failed".to_string()))
            } else {
                Ok(vec![1, 2, 3, 4])
            }
        }
    }

    struct MockDetector {
        use_cli: bool,
    }

    #[async_trait]
    impl InstallationDetector for MockDetector {
        async fn detect_method(
            &self,
            _ide_type: &IdeType,
        ) -> Result<InstallationMethod, DomainError> {
            if self.use_cli {
                Ok(InstallationMethod::CliCommand {
                    command_path: PathBuf::from("/usr/local/bin/code"),
                })
            } else {
                Ok(InstallationMethod::FileSystem {
                    extensions_dir: PathBuf::from("/home/user/.vscode/extensions"),
                })
            }
        }
    }

    struct MockExecutor {
        should_fail: bool,
    }

    #[async_trait]
    impl InstallationExecutor for MockExecutor {
        async fn execute(
            &self,
            _strategy: &InstallationStrategy,
            _extension_id: &str,
            _vsix_data: &[u8],
        ) -> Result<(), DomainError> {
            if self.should_fail {
                Err(DomainError::InstallationFailed(
                    "mock installation failed".to_string(),
                ))
            } else {
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn test_invalid_extension_id_format() {
        let repo = MockExtensionRepo { should_fail: false };
        let detector = MockDetector { use_cli: true };
        let executor = MockExecutor { should_fail: false };

        let use_case = ImprovedInstallExtensionUseCase::new(repo, detector, executor);

        // Test with invalid ID (no dot separator)
        let result = use_case.execute("invalidid", false, None).await;
        assert!(result.is_err());

        match result {
            Err(DomainError::InvalidExtensionFormat(msg)) => {
                assert!(msg.contains("publisher.name"));
            }
            _ => panic!("Expected InvalidExtensionFormat error"),
        }
    }

    #[tokio::test]
    async fn test_successful_cli_installation() {
        let repo = MockExtensionRepo { should_fail: false };
        let detector = MockDetector { use_cli: true };
        let executor = MockExecutor { should_fail: false };

        let use_case = ImprovedInstallExtensionUseCase::new(repo, detector, executor);

        let result = use_case.execute("publisher.extension", false, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_successful_filesystem_installation() {
        let repo = MockExtensionRepo { should_fail: false };
        let detector = MockDetector { use_cli: false };
        let executor = MockExecutor { should_fail: false };

        let use_case = ImprovedInstallExtensionUseCase::new(repo, detector, executor);

        let result = use_case.execute("publisher.extension", true, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_extension_not_found() {
        let repo = MockExtensionRepo { should_fail: true };
        let detector = MockDetector { use_cli: true };
        let executor = MockExecutor { should_fail: false };

        let use_case = ImprovedInstallExtensionUseCase::new(repo, detector, executor);

        let result = use_case.execute("publisher.extension", false, None).await;
        assert!(result.is_err());

        match result {
            Err(DomainError::ExtensionNotFound(_)) => {}
            _ => panic!("Expected ExtensionNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_installation_failure() {
        let repo = MockExtensionRepo { should_fail: false };
        let detector = MockDetector { use_cli: true };
        let executor = MockExecutor { should_fail: true };

        let use_case = ImprovedInstallExtensionUseCase::new(repo, detector, executor);

        let result = use_case.execute("publisher.extension", false, None).await;
        assert!(result.is_err());

        match result {
            Err(DomainError::InstallationFailed(_)) => {}
            _ => panic!("Expected InstallationFailed error"),
        }
    }

    #[tokio::test]
    async fn test_cursor_installation() {
        let repo = MockExtensionRepo { should_fail: false };
        let detector = MockDetector { use_cli: false };
        let executor = MockExecutor { should_fail: false };

        let use_case = ImprovedInstallExtensionUseCase::new(repo, detector, executor);

        // Test with use_cursor = true
        let result = use_case.execute("publisher.extension", true, None).await;
        assert!(result.is_ok());
    }
}

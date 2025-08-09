use crate::domain::{SearchResult, DomainError, ExtensionRepository, InstallationRepository, Architecture};

pub struct SearchExtensionUseCase<R: ExtensionRepository> {
    repository: R,
}

impl<R: ExtensionRepository> SearchExtensionUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    
    pub async fn execute(&self, query: &str, marketplace_url: Option<&str>) -> Result<SearchResult, DomainError> {
        if query.trim().is_empty() {
            return Err(DomainError::ParseError("Search query cannot be empty".to_string()));
        }
        
        self.repository.search(query, marketplace_url).await
    }
}

pub struct InstallExtensionUseCase<E: ExtensionRepository, I: InstallationRepository> {
    extension_repo: E,
    installation_repo: I,
}

impl<E: ExtensionRepository, I: InstallationRepository> InstallExtensionUseCase<E, I> {
    pub fn new(extension_repo: E, installation_repo: I) -> Self {
        Self {
            extension_repo,
            installation_repo,
        }
    }
    
    pub async fn execute(
        &self,
        extension_id: &str,
        use_cursor: bool,
        marketplace_url: Option<&str>,
    ) -> Result<(), DomainError> {
        let parts: Vec<&str> = extension_id.split('.').collect();
        if parts.len() != 2 {
            return Err(DomainError::InvalidExtensionFormat(
                "Extension ID must be in format 'publisher.name'".to_string()
            ));
        }
        
        let extension = self.extension_repo.get_extension(extension_id, marketplace_url).await?;
        
        let architecture = Architecture::detect();
        let platform = architecture.to_platform_string();
        
        let vsix_data = self.extension_repo.download(&extension, platform).await?;
        
        if use_cursor {
            self.installation_repo.install_cursor(&vsix_data, &extension.unique_identifier()).await
        } else {
            self.installation_repo.install_vscode(&vsix_data, &extension.unique_identifier()).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockExtensionRepo {
        search_result: Option<SearchResult>,
        should_fail: bool,
    }
    
    #[async_trait]
    impl ExtensionRepository for MockExtensionRepo {
        async fn search(&self, _query: &str, _marketplace_url: Option<&str>) -> Result<SearchResult, DomainError> {
            if self.should_fail {
                Err(DomainError::NetworkError("Mock error".to_string()))
            } else {
                Ok(self.search_result.clone().unwrap_or(SearchResult {
                    extensions: vec![],
                    total_count: 0,
                }))
            }
        }
        
        async fn get_extension(&self, _id: &str, _marketplace_url: Option<&str>) -> Result<Extension, DomainError> {
            unimplemented!()
        }
        
        async fn download(&self, _extension: &Extension, _target_platform: Option<&str>) -> Result<Vec<u8>, DomainError> {
            unimplemented!()
        }
    }
    
    #[tokio::test]
    async fn test_search_use_case_with_empty_query() {
        let repo = MockExtensionRepo {
            search_result: None,
            should_fail: false,
        };
        let use_case = SearchExtensionUseCase::new(repo);
        
        let result = use_case.execute("", None).await;
        assert!(result.is_err());
        
        match result {
            Err(DomainError::ParseError(msg)) => {
                assert_eq!(msg, "Search query cannot be empty");
            }
            _ => panic!("Expected ParseError"),
        }
    }
    
    #[tokio::test]
    async fn test_search_use_case_success() {
        let expected_result = SearchResult {
            extensions: vec![
                Extension {
                    id: "1".to_string(),
                    name: "python".to_string(),
                    publisher: "ms-python".to_string(),
                    version: "1.0.0".to_string(),
                    display_name: "Python".to_string(),
                    description: Some("Python support".to_string()),
                    downloads: 1000,
                }
            ],
            total_count: 1,
        };
        
        let repo = MockExtensionRepo {
            search_result: Some(expected_result.clone()),
            should_fail: false,
        };
        let use_case = SearchExtensionUseCase::new(repo);
        
        let result = use_case.execute("python", None).await.unwrap();
        assert_eq!(result.extensions.len(), 1);
        assert_eq!(result.extensions[0].name, "python");
    }
}
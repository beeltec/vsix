#[cfg(test)]
mod tests {
    use super::super::marketplace_client::MarketplaceClient;
    use crate::domain::ExtensionRepository;
    
    #[tokio::test]
    async fn test_search_nginx_extensions() {
        let client = MarketplaceClient::new();
        let result = client.search("nginx", None).await;
        
        assert!(result.is_ok(), "Search should succeed");
        let search_result = result.unwrap();
        assert!(search_result.total_count > 0, "Should find nginx extensions");
        
        let has_nginx_conf = search_result.extensions.iter()
            .any(|ext| ext.id.contains("ahmadalli.vscode-nginx-conf"));
        assert!(has_nginx_conf, "Should find ahmadalli.vscode-nginx-conf extension");
    }
    
    #[tokio::test]
    async fn test_get_specific_extension() {
        let client = MarketplaceClient::new();
        let result = client.get_extension("ahmadalli.vscode-nginx-conf", None).await;
        
        assert!(result.is_ok(), "Should be able to get specific extension");
        let extension = result.unwrap();
        assert_eq!(extension.id, "ahmadalli.vscode-nginx-conf");
        assert_eq!(extension.publisher, "ahmadalli");
        assert_eq!(extension.name, "vscode-nginx-conf");
    }
    
    #[tokio::test]
    async fn test_download_extension() {
        let client = MarketplaceClient::new();
        let extension = client.get_extension("ahmadalli.vscode-nginx-conf", None).await
            .expect("Should get extension");
        
        let download_result = client.download(&extension, None).await;
        assert!(download_result.is_ok(), "Download should succeed");
        
        let data = download_result.unwrap();
        assert!(data.len() > 0, "Should download data");
        
        // VSIX files are ZIP archives, so should start with PK
        assert_eq!(&data[0..2], b"PK", "Should be a valid VSIX/ZIP file");
    }
}
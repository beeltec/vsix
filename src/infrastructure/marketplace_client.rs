use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use crate::domain::{Extension, SearchResult, DomainError, ExtensionRepository};

pub struct MarketplaceClient {
    client: Client,
}

impl MarketplaceClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl ExtensionRepository for MarketplaceClient {
    async fn search(&self, query: &str, marketplace_url: Option<&str>) -> Result<SearchResult, DomainError> {
        let base_url = marketplace_url.unwrap_or("https://marketplace.visualstudio.com");
        let url = format!("{}/_apis/public/gallery/extensionquery", base_url);
        
        let request_body = json!({
            "filters": [{
                "criteria": [
                    {
                        "filterType": 8,
                        "value": "Microsoft.VisualStudio.Code"
                    },
                    {
                        "filterType": 10,
                        "value": query
                    }
                ]
            }],
            "assetTypes": [],
            "flags": 914
        });
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json;api-version=7.2-preview.1")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| DomainError::NetworkError(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(DomainError::NetworkError(format!("HTTP {}", response.status())));
        }
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| DomainError::ParseError(e.to_string()))?;
        
        let extensions = parse_search_results(&json)?;
        
        Ok(SearchResult {
            total_count: extensions.len(),
            extensions,
        })
    }
    
    async fn get_extension(&self, id: &str, marketplace_url: Option<&str>) -> Result<Extension, DomainError> {
        let search_result = self.search(id, marketplace_url).await?;
        
        search_result.extensions
            .into_iter()
            .find(|ext| ext.unique_identifier() == id)
            .ok_or_else(|| DomainError::ExtensionNotFound(id.to_string()))
    }
    
    async fn download(&self, extension: &Extension, target_platform: Option<&str>) -> Result<Vec<u8>, DomainError> {
        let url = extension.download_url(target_platform);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::NetworkError(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(DomainError::NetworkError(format!("HTTP {}", response.status())));
        }
        
        response.bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| DomainError::NetworkError(e.to_string()))
    }
}

fn parse_search_results(json: &serde_json::Value) -> Result<Vec<Extension>, DomainError> {
    let mut extensions = Vec::new();
    
    if let Some(results) = json["results"].as_array() {
        for result in results {
            if let Some(exts) = result["extensions"].as_array() {
                for ext in exts {
                    if let Ok(extension) = parse_extension(ext) {
                        extensions.push(extension);
                    }
                }
            }
        }
    }
    
    Ok(extensions)
}

fn parse_extension(json: &serde_json::Value) -> Result<Extension, DomainError> {
    let publisher = json["publisher"]["publisherName"]
        .as_str()
        .ok_or_else(|| DomainError::ParseError("Missing publisher name".to_string()))?;
    
    let name = json["extensionName"]
        .as_str()
        .ok_or_else(|| DomainError::ParseError("Missing extension name".to_string()))?;
    
    let display_name = json["displayName"]
        .as_str()
        .unwrap_or(name);
    
    let description = json["shortDescription"]
        .as_str()
        .map(|s| s.to_string());
    
    let version = json["versions"]
        .as_array()
        .and_then(|v| v.first())
        .and_then(|v| v["version"].as_str())
        .unwrap_or("latest")
        .to_string();
    
    let downloads = json["statistics"]
        .as_array()
        .and_then(|stats| {
            stats.iter()
                .find(|s| s["statisticName"].as_str() == Some("install"))
                .and_then(|s| s["value"].as_u64())
        })
        .unwrap_or(0);
    
    Ok(Extension {
        id: format!("{}.{}", publisher, name),
        name: name.to_string(),
        publisher: publisher.to_string(),
        version,
        display_name: display_name.to_string(),
        description,
        downloads,
    })
}
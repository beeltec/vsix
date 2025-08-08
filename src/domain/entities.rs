use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Extension {
    pub id: String,
    pub name: String,
    pub publisher: String,
    pub version: String,
    pub display_name: String,
    pub description: Option<String>,
    pub downloads: u64,
}

impl Extension {
    pub fn unique_identifier(&self) -> String {
        format!("{}.{}", self.publisher, self.name)
    }
    
    pub fn download_url(&self, target_platform: Option<&str>) -> String {
        let base_url = format!(
            "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{}/vsextensions/{}/{}/vspackage",
            self.publisher, self.name, self.version
        );
        
        match target_platform {
            Some(platform) => format!("{}?targetPlatform={}", base_url, platform),
            None => base_url,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub extensions: Vec<Extension>,
    pub total_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_unique_identifier() {
        let ext = Extension {
            id: "test-id".to_string(),
            name: "python".to_string(),
            publisher: "ms-python".to_string(),
            version: "2024.17.2024100401".to_string(),
            display_name: "Python".to_string(),
            description: Some("Python language support".to_string()),
            downloads: 1000,
        };
        
        assert_eq!(ext.unique_identifier(), "ms-python.python");
    }
    
    #[test]
    fn test_extension_download_url_with_platform() {
        let ext = Extension {
            id: "test-id".to_string(),
            name: "python".to_string(),
            publisher: "ms-python".to_string(),
            version: "2024.17.2024100401".to_string(),
            display_name: "Python".to_string(),
            description: None,
            downloads: 1000,
        };
        
        let url = ext.download_url(Some("win32-x64"));
        assert_eq!(
            url,
            "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/ms-python/vsextensions/python/2024.17.2024100401/vspackage?targetPlatform=win32-x64"
        );
    }
    
    #[test]
    fn test_extension_download_url_without_platform() {
        let ext = Extension {
            id: "test-id".to_string(),
            name: "python".to_string(),
            publisher: "ms-python".to_string(),
            version: "2024.17.2024100401".to_string(),
            display_name: "Python".to_string(),
            description: None,
            downloads: 1000,
        };
        
        let url = ext.download_url(None);
        assert_eq!(
            url,
            "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/ms-python/vsextensions/python/2024.17.2024100401/vspackage"
        );
    }
}
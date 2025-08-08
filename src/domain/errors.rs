use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Extension not found: {0}")]
    ExtensionNotFound(String),
    
    #[error("Invalid extension format: {0}")]
    InvalidExtensionFormat(String),
    
    #[error("Installation failed: {0}")]
    InstallationFailed(String),
    
    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}
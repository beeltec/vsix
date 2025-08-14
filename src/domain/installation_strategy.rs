use crate::domain::DomainError;
use async_trait::async_trait;
use std::path::PathBuf;

/// Represents the available IDE types for extension installation
#[derive(Debug, Clone, PartialEq)]
pub enum IdeType {
    VsCode,
    Cursor,
}

impl IdeType {
    /// Returns the CLI command name for the IDE
    pub fn command_name(&self) -> &'static str {
        match self {
            IdeType::VsCode => "code",
            IdeType::Cursor => "cursor",
        }
    }

    /// Returns the display name for the IDE
    pub fn display_name(&self) -> &'static str {
        match self {
            IdeType::VsCode => "VSCode",
            IdeType::Cursor => "Cursor",
        }
    }
}

/// Represents the installation method available for an IDE
#[derive(Debug, Clone, PartialEq)]
pub enum InstallationMethod {
    /// Install using CLI command (e.g., `code --install-extension`)
    CliCommand { command_path: PathBuf },
    /// Install by extracting to file system directory
    FileSystem { extensions_dir: PathBuf },
}

/// Value object representing an installation strategy for a specific IDE
#[derive(Debug, Clone)]
pub struct InstallationStrategy {
    pub ide_type: IdeType,
    pub method: InstallationMethod,
}

impl InstallationStrategy {
    pub fn new(ide_type: IdeType, method: InstallationMethod) -> Self {
        Self { ide_type, method }
    }
}

/// Service for detecting available installation methods
#[async_trait]
pub trait InstallationDetector: Send + Sync {
    /// Detects the available installation method for the specified IDE
    async fn detect_method(&self, ide_type: &IdeType) -> Result<InstallationMethod, DomainError>;
}

/// Service for executing installations using a specific strategy
#[async_trait]
pub trait InstallationExecutor: Send + Sync {
    /// Executes the installation using the provided strategy
    async fn execute(
        &self,
        strategy: &InstallationStrategy,
        extension_id: &str,
        vsix_data: &[u8],
    ) -> Result<(), DomainError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ide_type_command_names() {
        assert_eq!(IdeType::VsCode.command_name(), "code");
        assert_eq!(IdeType::Cursor.command_name(), "cursor");
    }

    #[test]
    fn test_ide_type_display_names() {
        assert_eq!(IdeType::VsCode.display_name(), "VSCode");
        assert_eq!(IdeType::Cursor.display_name(), "Cursor");
    }

    #[test]
    fn test_installation_strategy_creation() {
        let strategy = InstallationStrategy::new(
            IdeType::VsCode,
            InstallationMethod::CliCommand {
                command_path: PathBuf::from("/usr/local/bin/code"),
            },
        );

        assert_eq!(strategy.ide_type, IdeType::VsCode);
        match strategy.method {
            InstallationMethod::CliCommand { command_path } => {
                assert_eq!(command_path, PathBuf::from("/usr/local/bin/code"));
            }
            _ => panic!("Expected CliCommand"),
        }
    }
}

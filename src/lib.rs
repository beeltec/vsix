//! # vsix
//!
//! A command-line utility that downloads and installs .vsix extensions into Visual Studio Code and Cursor.
//!
//! ## Features
//!
//! - Search for extensions in the Visual Studio Code marketplace
//! - Install extensions to VSCode or Cursor
//! - Download extensions without installing
//! - Automatic CLI detection for `code` and `cursor` commands
//! - Cross-platform support (Windows, macOS, Linux)
//! - Architecture-aware installation (x86_64, ARM64)
//!
//! ## Architecture
//!
//! This crate follows Domain-Driven Design (DDD) principles with a clean architecture:
//!
//! - **Domain Layer**: Core business logic and domain model
//! - **Application Layer**: Use cases and service orchestration
//! - **Infrastructure Layer**: External service implementations
//! - **Presentation Layer**: User interface and CLI handling
//!
//! ## Example Usage
//!
//! ```no_run
//! use vsix::application::ApplicationService;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let service = ApplicationService::new();
//!     
//!     // Search for extensions
//!     let results = service.search_extensions("rust", None).await?;
//!     println!("Found {} extensions", results.total_count);
//!     
//!     // Install an extension
//!     service.install_extension("rust-lang.rust-analyzer", false, None).await?;
//!     
//!     Ok(())
//! }
//! ```

/// Application layer containing use cases and services
pub mod application;

/// Domain layer with core business logic, entities, and value objects
pub mod domain;

/// Infrastructure layer with external service implementations
pub mod infrastructure;

/// Presentation layer for user interface and CLI
pub mod presentation;

pub use application::*;
pub use domain::*;

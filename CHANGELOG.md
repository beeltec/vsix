# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.2] - 2025-08-14

### Added
- Comprehensive crate-level documentation with architecture overview
- Detailed API documentation for all public methods
- Usage examples in documentation for common scenarios
- Package metadata for better docs.rs presentation
- Maintenance badge configuration

### Changed
- Enhanced documentation for ApplicationService with complete examples
- Improved method documentation with parameters, returns, and error conditions
- Added documentation links and homepage to Cargo.toml

### Fixed
- Documentation formatting consistency with proper whitespace handling

## [1.0.1] - 2025-08-14

### Added
- Automatic CLI detection for VSCode and Cursor commands in PATH
- Improved installation strategy with CLI preference over filesystem extraction
- Comprehensive GitHub shields/badges in README
- Python script for robust Homebrew formula updates
- Verification step in release workflow

### Changed
- Refactored installation logic following Domain-Driven Design (DDD) principles
- Improved code organization using SOLID principles and Clean Code practices
- Enhanced error handling in release workflow
- Better separation of concerns with new installation strategy pattern

### Fixed
- Fixed Homebrew formula update in release workflow
- Fixed temporary file cleanup in GitHub Actions
- Fixed SHA256 checksum updates for all platforms
- Resolved untracked files issue in release workflow

### Technical Improvements
- Implemented InstallationDetector and InstallationExecutor traits
- Added SystemInstallationDetector for PATH command detection
- Created ImprovedInstallExtensionUseCase with better architecture
- Added comprehensive unit tests following TDD approach
- Updated to which crate v7.0 for command detection

## [1.0.0] - 2025-08-13

### Added
- Initial stable release of vsix CLI tool
- Search functionality for VSCode marketplace extensions with sorting and filtering
- Install capability for extensions to VSCode and Cursor editors
- Download command to save VSIX files without installing
- Automatic detection and use of VSCode CLI (`code` command) when available
- Automatic detection and use of Cursor CLI (`cursor` command) when available
- Sorting options for search results (`--sort` with name, downloads, publisher fields)
- Reverse sort order flag (`--reverse`)
- Result limit option (`--limit`, default: 20)
- Publisher column in search results table for better extension identification
- Output directory option for download command
- Automatic system architecture detection (x64, arm64, universal)
- Beautiful table display for search results with colored output
- Support for custom marketplace URLs
- Progress indicators during download and installation
- Error handling with detailed messages
- Clean architecture implementation with domain-driven design
- Full test coverage including unit and integration tests
- GitHub Actions CI/CD pipeline for multi-platform builds
- Security auditing with cargo-audit
- Comprehensive integration tests for marketplace API
- Test-Driven Development (TDD) approach for bug fixes

### Features
- **Search Extensions**: Query the VSCode marketplace with keywords
- **Install Extensions**: Download and install VSIX files directly
- **Download Extensions**: Save VSIX files without installing
- **Multi-Editor Support**: Compatible with both VSCode and Cursor
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Architecture Detection**: Automatically selects correct extension variant
- **Custom Marketplace**: Configure alternative marketplace endpoints
- **CLI Integration**: Uses native editor CLI tools when available

### Technical Details
- Built with Rust for performance and reliability
- Async/await support with Tokio runtime
- Clean separation of concerns using hexagonal architecture
- Error handling with custom error types
- Modular design with domain, application, and infrastructure layers
- Installation prefers CLI tools when available with fallback to manual extraction
- Added flate2 dependency for proper gzip handling
- Enhanced code organization following SOLID principles

### Fixed
- Fixed VSCode marketplace API endpoint URL
- Fixed HTTP 500 errors during extension download
- Fixed automatic gzip decompression for downloaded VSIX files
- Fixed table header alignment issues in search results
- Improved table display with fixed column widths and better alignment
- Enhanced search result presentation with dynamic content arrangement

[1.0.2]: https://github.com/beeltec/vsix/releases/tag/v1.0.2
[1.0.1]: https://github.com/beeltec/vsix/releases/tag/v1.0.1
[1.0.0]: https://github.com/beeltec/vsix/releases/tag/v1.0.0

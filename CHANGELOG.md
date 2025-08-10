# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-08-10

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

[1.0.0]: https://github.com/beeltec/vsix/releases/tag/v1.0.0

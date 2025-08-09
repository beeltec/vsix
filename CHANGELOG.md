# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-08-09

### Added
- Initial stable release of vsix-install CLI tool
- Search functionality for VSCode marketplace extensions
- Install capability for extensions to VSCode and Cursor editors
- Automatic system architecture detection (x64, arm64, universal)
- Beautiful table display for search results with colored output
- Support for custom marketplace URLs
- Progress indicators during download and installation
- Error handling with detailed messages
- Clean architecture implementation with domain-driven design
- Full test coverage including unit and integration tests
- GitHub Actions CI/CD pipeline for multi-platform builds
- Security auditing with cargo-audit

### Features
- **Search Extensions**: Query the VSCode marketplace with keywords
- **Install Extensions**: Download and install VSIX files directly
- **Multi-Editor Support**: Compatible with both VSCode and Cursor
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Architecture Detection**: Automatically selects correct extension variant
- **Custom Marketplace**: Configure alternative marketplace endpoints

### Technical Details
- Built with Rust for performance and reliability
- Async/await support with Tokio runtime
- Clean separation of concerns using hexagonal architecture
- Error handling with custom error types
- Modular design with domain, application, and infrastructure layers

[1.0.0]: https://github.com/yourusername/vsix-install/releases/tag/v1.0.0
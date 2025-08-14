# vsix

[![Crates.io Version](https://img.shields.io/crates/v/vsix)](https://crates.io/crates/vsix)
[![Crates.io Downloads](https://img.shields.io/crates/d/vsix)](https://crates.io/crates/vsix)
[![License](https://img.shields.io/crates/l/vsix)](https://github.com/beeltec/vsix/blob/main/LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/beeltec/vsix/release.yml?branch=main&label=release)](https://github.com/beeltec/vsix/actions/workflows/release.yml)
[![CI Status](https://img.shields.io/github/actions/workflow/status/beeltec/vsix/ci.yml?branch=main&label=ci)](https://github.com/beeltec/vsix/actions/workflows/ci.yml)
[![GitHub Stars](https://img.shields.io/github/stars/beeltec/vsix?style=social)](https://github.com/beeltec/vsix)
[![Rust Version](https://img.shields.io/badge/rust-1.89%2B-blue)](https://www.rust-lang.org)
[![docs.rs](https://img.shields.io/docsrs/vsix)](https://docs.rs/vsix)
[![Dependencies](https://img.shields.io/librariesio/release/cargo/vsix)](https://libraries.io/cargo/vsix)

A command-line utility that downloads and installs .vsix extensions into Visual Studio Code and Cursor

## Features

- Search for extensions in the Visual Studio Code marketplace
- Install extensions to Visual Studio Code or Cursor
- Automatic system architecture detection
- Beautiful table display for search results
- Support for custom marketplace URLs

## Platform Support

[![macOS](https://img.shields.io/badge/macOS-x86__64%20%7C%20ARM64-black?logo=apple)](https://github.com/beeltec/vsix/releases)
[![Linux](https://img.shields.io/badge/Linux-x86__64%20%7C%20ARM64-FCC624?logo=linux&logoColor=black)](https://github.com/beeltec/vsix/releases)
[![Windows](https://img.shields.io/badge/Windows-x86__64%20%7C%20ARM64-0078D4?logo=windows)](https://github.com/beeltec/vsix/releases)

## Installation

### Homebrew (macOS and Linux)

```bash
# Coming soon - after first release
brew tap beeltec/vsix
brew install vsix
```

### Cargo

```bash
cargo install vsix
```

### Build from source

```bash
# Clone the repository
git clone https://github.com/beeltec/vsix.git
cd vsix

# Build the project
cargo build --release

# The binary will be available at ./target/release/vsix
```

### Download prebuilt binaries

Prebuilt binaries are available from the [releases page](https://github.com/beeltec/vsix/releases) for:
- macOS (Intel and Apple Silicon)
- Linux (x86_64)
- Windows (x86_64)

## Usage

### Search for extensions
```bash
vsix search python

# Sort by name
vsix search rust --sort name

# Limit results
vsix search python --limit 10

# Reverse sort order
vsix search vscode --sort downloads --reverse
```

### Install an extension
```bash
# Install to VSCode (default)
vsix install ms-python.python

# Install to Cursor
vsix install ms-python.python --cursor
```

### Use custom marketplace
```bash
vsix --marketplace https://custom.marketplace.com search rust
```

## Development

### Prerequisites

- Rust 1.88 or higher
- Cargo

### Running tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_architecture_detection
```

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run directly with cargo
cargo run -- search python
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Fix warnings
cargo fix
```

## Project Structure

```
src/
├── domain/           # Core business logic
│   ├── entities.rs   # Extension entity
│   ├── value_objects.rs # Architecture detection
│   ├── repositories.rs  # Repository traits
│   └── errors.rs     # Domain errors
├── application/      # Use cases and services
│   ├── use_cases.rs  # Search and install use cases
│   └── services.rs   # Application service
├── infrastructure/   # External implementations
│   ├── marketplace_client.rs # VSCode marketplace API
│   └── file_system.rs # File system operations
├── presentation/     # User interface
│   ├── cli.rs        # CLI argument parsing
│   └── display.rs    # Output formatting
└── main.rs          # Application entry point
```

## Contributing

[![Issues](https://img.shields.io/github/issues/beeltec/vsix)](https://github.com/beeltec/vsix/issues)
[![Pull Requests](https://img.shields.io/github/issues-pr/beeltec/vsix)](https://github.com/beeltec/vsix/pulls)
[![Contributors](https://img.shields.io/github/contributors/beeltec/vsix)](https://github.com/beeltec/vsix/graphs/contributors)
[![Last Commit](https://img.shields.io/github/last-commit/beeltec/vsix)](https://github.com/beeltec/vsix/commits/main)

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines

- Follow Rust best practices and idioms
- Write tests for new functionality
- Ensure all tests pass before submitting PR
- Follow the existing code structure (DDD architecture)
- Update documentation when needed

## Transparency

This application was developed with assistance from [Claude Code](https://claude.ai/code), an AI coding assistant. Claude Code helped with code implementation, testing, documentation, and CI/CD pipeline setup.

## License

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

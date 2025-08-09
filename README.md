# vsix-install

A command-line tool to download and install VSCode extensions from the marketplace.

## Features

- Search for extensions in the VSCode marketplace
- Install extensions to VSCode or Cursor
- Automatic system architecture detection
- Beautiful table display for search results
- Support for custom marketplace URLs

## Installation

### Build from source

```bash
# Clone the repository
git clone https://github.com/yourusername/vsix-install.git
cd vsix-install

# Build the project
cargo build --release

# The binary will be available at ./target/release/vsix-install
```

## Usage

### Search for extensions
```bash
vsix-install search python
```

### Install an extension
```bash
# Install to VSCode (default)
vsix-install install ms-python.python

# Install to Cursor
vsix-install install ms-python.python --cursor
```

### Use custom marketplace
```bash
vsix-install --marketplace https://custom.marketplace.com search rust
```

## Development

### Prerequisites

- Rust 1.70 or higher
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

## Transparency

This application was developed with assistance from [Claude Code](https://claude.ai/code), an AI coding assistant. Claude Code helped with code implementation, testing, documentation, and CI/CD pipeline setup.

## License

MIT
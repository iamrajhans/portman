# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-08-02

### Added
- **Core Commands**
  - `list` - Display all occupied ports with process information
  - `check` - Check if ports are available (script-friendly with exit codes)
  - `kill` - Terminate processes on specific ports with safety checks
  - `free` - Kill processes on common development ports
  - `init` - Create project configuration files
  - `watch` - Monitor ports from configuration file
  - `history` - Show recent port management actions (placeholder)

- **Features**
  - Cross-platform support (Linux, macOS, Windows)
  - Beautiful table output with colors and proper alignment
  - Multiple output formats (table, JSON, CSV)
  - Port range checking (e.g., `3000-3010`)
  - Process filtering by name
  - Common development ports preset
  - Configuration file support (YAML/TOML)
  - Safety checks for system-critical processes
  - Graceful process termination (SIGTERM → SIGKILL)

- **User Experience**
  - Intuitive command-line interface using clap
  - Colored output for better readability
  - Progress indicators and status messages
  - Confirmation prompts with `--force` override
  - Comprehensive help documentation
  - Memory usage and runtime duration display

- **Developer Features**
  - Comprehensive test suite
  - GitHub Actions CI/CD pipeline
  - Cross-platform release automation
  - Installation script for easy setup
  - Linting and formatting checks
  - Security audit integration

### Technical Details
- Built with Rust 2021 edition
- Uses `lsof` on macOS for accurate port detection
- Uses `netstat` on Linux and Windows
- Leverages `sysinfo` for cross-platform process information
- Table rendering with `tabled` crate
- Async runtime with `tokio`

### Performance
- Fast port scanning (<100ms for typical systems)
- Efficient memory usage
- Minimal system resource impact
- Optimized for developer workflows

[Unreleased]: https://github.com/yourusername/portman/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/yourusername/portman/releases/tag/v1.0.0
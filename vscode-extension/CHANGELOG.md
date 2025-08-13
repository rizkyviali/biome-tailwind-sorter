# Changelog

All notable changes to the "Biome Tailwind Sorter" extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-01-XX

### Added
- Initial release of Biome Tailwind Sorter VS Code extension
- Automatic Tailwind CSS class sorting with cursor preservation
- Command palette integration with "Sort Tailwind Classes" command
- Keyboard shortcut support (Ctrl+Shift+T / Cmd+Shift+T)
- Format-on-save functionality (configurable)
- Multi-language support (HTML, JS/TS/JSX/TSX, Vue, Astro)
- Smart binary detection (node_modules, target/, PATH)
- Comprehensive configuration options
- Error handling with user-friendly messages
- Support for both single-line and multi-line class attributes

### Features
- **High Performance**: Rust-powered binary for instant sorting
- **Cursor Preservation**: Maintains exact cursor position during formatting
- **Language Support**: HTML, JavaScript, TypeScript, JSX, TSX, Vue, Astro
- **Configurable**: Enable/disable, format-on-save, custom binary path
- **Smart Detection**: Automatically finds the biome-tailwind-sorter binary
- **Official Ordering**: Follows Tailwind CSS official class ordering

### Configuration
- `biome-tailwind-sorter.enable`: Enable/disable extension
- `biome-tailwind-sorter.formatOnSave`: Auto-format on save
- `biome-tailwind-sorter.binaryPath`: Custom binary path
- `biome-tailwind-sorter.languages`: Supported file types

### Requirements
- VS Code 1.74.0 or higher
- biome-tailwind-sorter binary (npm package)

---

## Version History

### Development Milestones

- **v0.1.0**: Initial marketplace release
  - Core functionality complete
  - Documentation and examples
  - Multi-platform support
  - Performance optimizations
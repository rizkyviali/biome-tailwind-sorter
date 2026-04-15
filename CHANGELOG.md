# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.11] - 2026-04-16

### Added
- `.gitignore` and `.ignore` file support — directory traversal now automatically respects ignore rules via the `ignore` crate

### Fixed
- CI integration test exit code handling with `bash -e`
- Clippy warnings: `unnecessary_sort_by`, `uninlined_format_args`, `field_reassign_with_default`, `unnecessary_to_owned`
- Rust formatting issues across multiple files

## [0.2.9] - 2026-04-06

### Added
- `.gitignore` file exclusions for `target/` build artifacts

### Fixed
- Multiline class string reconstruction with better indentation
- Improved Tailwind class ordering with better prefix matching
- Success message now only shows when content actually changes

## [0.2.8] - 2025-08-20

### Added
- Tailwind configuration file detection and parsing (`tailwind.config.js`, `.mjs`, `.ts`, `.json`)
- Stdin support for piping content through the formatter
- `--doctor` command for installation and configuration diagnostics
- `--debug` flag for detailed troubleshooting output
- `--max-size` option for configurable file size limits
- Recursive directory traversal with intelligent ignore patterns
- Performance testing and comprehensive error handling

### Changed
- Improved formatter performance with early exits and reduced memory allocations
- Enhanced CLI interface with better error messages and progress indicators
- File processing now supports configurable size limits (default 10MB, was 50MB)
- Directory traversal now recursively processes subdirectories while ignoring common build/cache folders

### Fixed
- Binary path resolution in package.json now uses relative `./target/release/` prefix
- Removed unused struct fields and dead code for cleaner codebase
- Memory usage optimizations in class extraction and sorting algorithms

## [0.2.7] - 2025-08-14

### Added
- Support for `.astro` files - Astro components can now be formatted with Tailwind class sorting

### Fixed
- Resolved all compilation warnings and errors

## [0.2.5] - 2025-08-04

### Added
- High-performance Rust-based formatter for Tailwind CSS classes
- Cursor position preservation for seamless editor integration
- CLI tool with `--write`, `--check`, and `--preserve-cursor` options
- Support for multiple file types: HTML, JSX, TSX, and Vue
- Multi-line class attribute support
- Smart Tailwind class detection
- VS Code integration via Run on Save extension
- Comprehensive test coverage with 19 passing tests

### Features
- Automatic sorting according to Tailwind's official class ordering
- Zero runtime dependencies
- Memory efficient processing
- Concurrent file processing support
- Custom Tailwind config compatibility

### Supported File Extensions
- `.js` - JavaScript files with JSX
- `.jsx` - React JSX files  
- `.ts` - TypeScript files
- `.tsx` - React TypeScript files
- `.html` - HTML files
- `.vue` - Vue.js single file components

## [0.2.0] - 2025-08-02

### Added
- Initial release of biome-tailwind-sorter
- Rust-based CLI implementation
- Basic Tailwind class sorting functionality

## [0.1.0] - 2025-08-01

### Added
- Initial commit and project setup

---

## How to Update This Changelog

When making changes to the project:

1. Add new changes under the `[Unreleased]` section
2. Use the following categories:
   - `Added` for new features
   - `Changed` for changes in existing functionality  
   - `Deprecated` for soon-to-be removed features
   - `Removed` for now removed features
   - `Fixed` for any bug fixes
   - `Security` for vulnerability fixes

3. When releasing a new version:
   - Move unreleased changes to a new version section
   - Update the version number and date
   - Create a new empty `[Unreleased]` section
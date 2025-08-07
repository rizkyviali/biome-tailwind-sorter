# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Support for `.astro` files - Astro components can now be formatted with Tailwind class sorting

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
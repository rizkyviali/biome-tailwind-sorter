# Biome Tailwind Sorter - VS Code Extension

[![Visual Studio Marketplace Version](https://img.shields.io/visual-studio-marketplace/v/rizkyviali.biome-tailwind-sorter-vscode)](https://marketplace.visualstudio.com/items?itemName=rizkyviali.biome-tailwind-sorter-vscode)
[![Visual Studio Marketplace Downloads](https://img.shields.io/visual-studio-marketplace/d/rizkyviali.biome-tailwind-sorter-vscode)](https://marketplace.visualstudio.com/items?itemName=rizkyviali.biome-tailwind-sorter-vscode)

A high-performance VS Code extension for automatically sorting Tailwind CSS classes with advanced cursor position preservation. Powered by a blazing-fast Rust binary for optimal performance.

## ✨ Features

- 🚀 **High Performance** - Rust-powered sorting for instant results
- 🎯 **Cursor Preservation** - Maintains exact cursor position during formatting
- ⚡ **Format on Save** - Automatic class sorting when files are saved
- 🎨 **Multi-language Support** - Works with HTML, JSX, TSX, Vue, and Astro
- ⚙️ **Configurable** - Extensive customization options
- 🔧 **Smart Binary Detection** - Automatically finds the sorter binary
- 🎹 **Keyboard Shortcuts** - Quick access with Ctrl+Shift+T (Cmd+Shift+T on Mac)

## 🚀 Quick Start

1. **Install the extension** from the VS Code marketplace
2. **Install the binary** via npm: `npm install --save-dev biome-tailwind-sorter`
3. **Start sorting** with `Ctrl+Shift+T` or enable format-on-save

## 📦 Installation

### Method 1: VS Code Marketplace
1. Open VS Code
2. Go to Extensions (`Ctrl+Shift+X`)
3. Search for "Biome Tailwind Sorter"
4. Click Install

### Method 2: Command Line
```bash
code --install-extension rizkyviali.biome-tailwind-sorter-vscode
```

### Binary Installation
The extension requires the `biome-tailwind-sorter` binary:

```bash
# Install locally (recommended)
npm install --save-dev biome-tailwind-sorter

# Or install globally
npm install -g biome-tailwind-sorter
```

## 🎯 Usage

### Manual Sorting
- **Command Palette**: `Ctrl+Shift+P` → "Sort Tailwind Classes"
- **Keyboard Shortcut**: `Ctrl+Shift+T` (Windows/Linux) or `Cmd+Shift+T` (Mac)
- **Right-click menu**: Available in supported file types

### Automatic Sorting
Enable format-on-save in your VS Code settings:

```json
{
  "biome-tailwind-sorter.formatOnSave": true
}
```

## ⚙️ Configuration

### Extension Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `biome-tailwind-sorter.enable` | boolean | `true` | Enable/disable the extension |
| `biome-tailwind-sorter.formatOnSave` | boolean | `false` | Sort classes automatically on file save |
| `biome-tailwind-sorter.binaryPath` | string | `""` | Custom path to the binary (auto-detected if empty) |
| `biome-tailwind-sorter.languages` | array | `["html", "javascript", ...]` | Languages to enable sorting for |

### Example Configuration

```json
{
  "biome-tailwind-sorter.enable": true,
  "biome-tailwind-sorter.formatOnSave": true,
  "biome-tailwind-sorter.binaryPath": "./node_modules/.bin/biome-tailwind-sorter",
  "biome-tailwind-sorter.languages": [
    "html",
    "javascript", 
    "javascriptreact",
    "typescript",
    "typescriptreact",
    "vue",
    "astro"
  ]
}
```

## 📝 Supported File Types

- **HTML** (`.html`) - `class` attributes
- **JavaScript/JSX** (`.js`, `.jsx`) - `className` attributes
- **TypeScript/TSX** (`.ts`, `.tsx`) - `className` attributes
- **Vue** (`.vue`) - `class` attributes
- **Astro** (`.astro`) - `class` and `className` attributes

## 🎨 Before & After

### Before (unsorted):
```jsx
<div className="text-white bg-red-500 p-4 hover:bg-red-600 rounded-lg font-semibold">
  Button
</div>
```

### After (sorted):
```jsx
<div className="p-4 font-semibold text-white bg-red-500 rounded-lg hover:bg-red-600">
  Button
</div>
```

## 🏗️ Class Ordering

The extension follows Tailwind's official class ordering:

1. **Layout** - `container`, `box-border`, `block`, `flex`, `grid`
2. **Position** - `static`, `fixed`, `absolute`, `relative`
3. **Spacing** - `p-*`, `m-*`, `space-*`
4. **Sizing** - `w-*`, `h-*`, `min-w-*`, `max-w-*`
5. **Typography** - `font-*`, `text-*`, `tracking-*`
6. **Backgrounds** - `bg-*`, `from-*`, `via-*`, `to-*`
7. **Borders** - `border-*`, `rounded-*`, `ring-*`
8. **Effects** - `shadow-*`, `opacity-*`
9. **Transitions** - `transition-*`, `duration-*`
10. **Transforms** - `transform`, `scale-*`, `rotate-*`

Responsive and state modifiers (`sm:`, `hover:`, etc.) are preserved and sorted appropriately.

## 🔧 Commands

| Command | Description | Keyboard Shortcut |
|---------|-------------|-------------------|
| `biome-tailwind-sorter.sortClasses` | Sort Tailwind classes with cursor preservation | `Ctrl+Shift+T` / `Cmd+Shift+T` |
| `biome-tailwind-sorter.sortClassesInFile` | Sort all classes in the current file | None |

## 🚨 Troubleshooting

### Binary Not Found
**Error**: `Failed to spawn biome-tailwind-sorter`

**Solutions**:
1. Install the binary: `npm install biome-tailwind-sorter`
2. Set custom path in settings: `"biome-tailwind-sorter.binaryPath": "/path/to/binary"`
3. Add to PATH or install globally

### Permission Denied
**Error**: Permission denied when executing binary

**Solutions**:
1. Make binary executable: `chmod +x ./node_modules/.bin/biome-tailwind-sorter`
2. Use absolute path in settings

### Format Not Working
**Issues**: Classes not being sorted

**Check**:
1. File type is supported (see supported file types)
2. Extension is enabled in settings
3. File contains valid Tailwind classes
4. Binary is accessible and working

## 🚀 Performance

- **Rust-powered**: 10-50x faster than JavaScript alternatives
- **Cursor preservation**: Maintains exact editing position
- **Memory efficient**: Handles large files without performance issues
- **Instant feedback**: No noticeable delay on most files

## 🤝 Contributing

Found a bug or want to contribute? 

- **Report issues**: [GitHub Issues](https://github.com/rizkyviali/biome-tailwind-sorter/issues)
- **Source code**: [GitHub Repository](https://github.com/rizkyviali/biome-tailwind-sorter)
- **Main project**: [Biome Tailwind Sorter CLI](https://github.com/rizkyviali/biome-tailwind-sorter)

## 📄 License

MIT - See [LICENSE](https://github.com/rizkyviali/biome-tailwind-sorter/blob/master/LICENSE) for details.

## 🙏 Acknowledgments

- [Tailwind CSS](https://tailwindcss.com/) for the official class ordering specification
- [Biome](https://biomejs.dev/) for inspiration on fast tooling
- [VS Code](https://code.visualstudio.com/) for the excellent extension API

---

**Enjoy lightning-fast Tailwind CSS class sorting!** ⚡
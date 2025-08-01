# biome-tailwind-sorter

A Biome plugin and formatter for automatically sorting Tailwind CSS classes according to the official recommended order.

## Features

- ✅ **Automatic formatting** - Sorts Tailwind classes on file save (format-on-save)  
- ✅ **CLI tool** - Standalone formatter for any project
- ✅ **Biome integration** - Works as both linter rule and formatter plugin
- ✅ **Multi-language support** - HTML, JSX, TSX, and Vue files
- ✅ **Multi-line classes** - Preserves formatting for multi-line class definitions
- ✅ **Smart detection** - Only processes files that contain Tailwind classes
- ✅ **TypeScript** - Written in TypeScript with full type support

## Installation

```bash
npm install --save-dev biome-tailwind-sorter
```

## Usage

### Method 1: CLI Tool (Recommended)

Use the standalone CLI tool for immediate formatting:

```bash
# Format specific files
npx biome-tailwind-sorter --write src/components/Button.tsx

# Format all supported files in a directory  
npx biome-tailwind-sorter --write src/

# Check if files need formatting (useful in CI)
npx biome-tailwind-sorter --check src/
```

### Method 2: VS Code Integration

For automatic formatting on save, add this to your VS Code settings:

```json
{
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll": true
  },
  "[typescript]": {
    "editor.defaultFormatter": "biomejs.biome"
  },
  "[typescriptreact]": {
    "editor.defaultFormatter": "biomejs.biome"
  }
}
```

Then add to your `biome.json`:

```json
{
  "plugins": ["biome-tailwind-sorter"],
  "linter": {
    "rules": {
      "tailwind/useSortedClasses": "error"
    }
  }
}
```

### Method 3: Package.json Script

Add a format script to your `package.json`:

```json
{
  "scripts": {
    "format:tailwind": "biome-tailwind-sorter --write src/"
  }
}
```

## Examples

### Before (unsorted):
```jsx
<div
  className="
    text-white
    bg-red-500
    p-4
    hover:bg-red-600
    rounded-lg
    font-semibold
  "
>
  Button
</div>
```

### After (sorted):
```jsx
<div
  className="
    p-4
    font-semibold
    text-white
    bg-red-500
    rounded-lg
    hover:bg-red-600
  "
>
  Button
</div>
```

### Single line example:

**Before:** `<div className="text-white bg-red-500 p-4" />`

**After:** `<div className="p-4 bg-red-500 text-white" />`

## Supported File Types

- **HTML** - `class` attributes
- **JSX/TSX** - `className` attributes  
- **Vue** - `class` attributes

## Class Ordering

The plugin follows Tailwind's official class ordering:

1. **Layout** - `container`, `box-border`, `block`, `inline-block`, `flex`, `grid`, etc.
2. **Position** - `static`, `fixed`, `absolute`, `relative`, `sticky`
3. **Top/Right/Bottom/Left** - `inset-*`, `top-*`, `right-*`, etc.
4. **Visibility** - `visible`, `invisible`, `collapse`
5. **Z-Index** - `z-*`
6. **Flex & Grid** - `flex-row`, `justify-center`, `items-center`, etc.
7. **Spacing** - `p-*`, `m-*`, `space-*`, etc.
8. **Sizing** - `w-*`, `h-*`, `min-w-*`, `max-w-*`, etc.
9. **Typography** - `font-*`, `text-*`, `tracking-*`, etc.
10. **Backgrounds** - `bg-*`, `from-*`, `via-*`, `to-*`
11. **Borders** - `border-*`, `rounded-*`, `ring-*`
12. **Effects** - `shadow-*`, `opacity-*`
13. **Filters** - `filter`, `blur-*`, `brightness-*`, etc.
14. **Transitions** - `transition-*`, `duration-*`, `ease-*`
15. **Transforms** - `transform`, `scale-*`, `rotate-*`, etc.
16. **Interactivity** - `cursor-*`, `pointer-events-*`, etc.

Responsive modifiers (`sm:`, `md:`, `lg:`, etc.) and state modifiers (`hover:`, `focus:`, etc.) are preserved and sorted appropriately.

## Rule Configuration

You can configure the rule severity in your `biome.json`:

```json
{
  "linter": {
    "rules": {
      "tailwind/useSortedClasses": "error"    // Error (default)
      // or
      "tailwind/useSortedClasses": "warn"     // Warning
      // or  
      "tailwind/useSortedClasses": "off"      // Disabled
    }
  }
}
```

## Advanced Usage

### Programmatic API

You can also use the plugin's utilities programmatically:

```typescript
import { sortTailwindClasses, areClassesSorted } from 'biome-tailwind-sorter';

const classes = ['text-white', 'bg-red-500', 'p-4'];
const sorted = sortTailwindClasses(classes);
console.log(sorted); // ['p-4', 'bg-red-500', 'text-white']

const isAlreadySorted = areClassesSorted(['p-4', 'bg-red-500', 'text-white']);
console.log(isAlreadySorted); // true
```

### Custom Tailwind Config Support

The plugin automatically detects standard Tailwind classes. For custom utilities defined in your `tailwind.config.js`, the plugin will leave them in their original position to avoid breaking functionality.

## CLI Options

```bash
npx biome-tailwind-sorter [options] <files...>

Options:
  --write, -w      Write sorted classes back to files
  --check, -c      Check if files need sorting (exit code 1 if changes needed)
  --verbose, -v    Verbose output
  --help, -h       Show help message
```

## Comparison with Prettier Plugin

This plugin is designed to work natively with Biome and doesn't require Prettier or `prettier-plugin-tailwindcss`. Key differences:

- ✅ **Native Biome integration** - No additional dependencies
- ✅ **Fast performance** - Leverages Biome's speed  
- ✅ **Standalone CLI** - Works in any project, even without Biome
- ✅ **Format-on-save support** - True automatic formatting
- ✅ **Multi-line preservation** - Maintains your code formatting
- ✅ **TypeScript native** - Better type safety and IDE support

## Development

### Building the plugin:

```bash
npm run build
```

### Running tests:

```bash
npm test
```

### Development mode:

```bash
npm run dev
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Tailwind CSS](https://tailwindcss.com/) for the official class ordering specification
- [Biome](https://biomejs.dev/) for the excellent toolchain and plugin architecture
- [Prettier Tailwind Plugin](https://github.com/tailwindlabs/prettier-plugin-tailwindcss) for inspiration
# Extension Icon Placeholder

The VS Code extension needs a 128x128 PNG icon. For production release, create an icon with:

- **Size**: 128x128 pixels
- **Format**: PNG
- **Background**: Transparent or solid color
- **Theme**: Related to Tailwind CSS or sorting/organizing
- **Colors**: Consider using Tailwind's brand colors or VS Code theme colors

## Suggested Design Elements:
- CSS class brackets `{}`
- Sorting arrows ↕️
- Tailwind logo elements
- Grid/organization symbols
- Lightning bolt (for performance)

## Icon Requirements:
- Must be named `icon.png`
- Place in the root of the vscode-extension directory
- Should look good on both light and dark backgrounds
- Follows VS Code design guidelines

For now, update package.json to remove the icon reference until a proper icon is created.
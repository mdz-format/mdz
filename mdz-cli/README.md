# mdz-cli

[English](README.md) | [中文版本](README_CN.md)

Command-line tool for creating and managing MDZ (Markdown Zipped) documents.

## Overview

`mdz-cli` is a comprehensive command-line interface for working with MDZ files. It provides all the essential operations needed to create, extract, validate, and preview MDZ documents.

## Installation

### From Source

```bash
git clone https://github.com/mdz-format/mdz.git
cd mdz
cargo install --path mdz-cli
```

### Local Build

```bash
cargo build --release
# Binary at: ./target/release/mdz-cli
```

## Quick Start

### Create an MDZ file

```bash
# Create from a directory structure
mdz create document.mdz --from ./my-document

# With verbose output
mdz create document.mdz --from ./my-document --verbose

# Custom compression level
mdz create document.mdz --from ./my-document --compression 9
```

### Extract an MDZ file

```bash
# Extract to a directory
mdz extract document.mdz --to ./extracted

# Force overwrite existing files
mdz extract document.mdz --to ./extracted --force

# With verbose output
mdz extract document.mdz --to ./extracted --verbose
```

### Validate an MDZ file

```bash
# Basic validation
mdz validate document.mdz

# Detailed validation report
mdz validate document.mdz --detailed

# Strict mode (warnings as errors)
mdz validate document.mdz --strict
```

### Preview an MDZ file

```bash
# Generate HTML preview
mdz preview document.mdz

# Open in browser automatically
mdz preview document.mdz --browser

# Save to specific file
mdz preview document.mdz --output preview.html

# Use custom CSS
mdz preview document.mdz --css custom.css

# Disable CSS
mdz preview document.mdz --no-css
```

## Commands

### `mdz create`

Create an MDZ file from a source directory.

```
mdz create <OUTPUT> --from <SOURCE>

Arguments:
  <OUTPUT>              Output MDZ file path

Options:
  -f, --from <SOURCE>   Source directory containing markdown and resources
  -c, --compression <LEVEL>  Compression level (0-9) [default: 6]
  -v, --verbose         Show detailed output
  -h, --help            Print help
```

**Source Directory Structure:**
```
my-document/
├── main.md           # Required: Main markdown content
├── img/              # Optional: Image resources
│   ├── photo.jpg
│   └── diagram.png
└── css/              # Optional: Stylesheets
    └── style.css     # Auto-loaded main stylesheet
```

**Examples:**

```bash
# Basic creation
mdz create report.mdz --from ./report-source

# High compression for distribution
mdz create report.mdz --from ./report-source --compression 9

# With detailed progress
mdz create report.mdz --from ./report-source --verbose
```

### `mdz extract`

Extract an MDZ file to a directory.

```
mdz extract <INPUT> --to <OUTPUT>

Arguments:
  <INPUT>               MDZ file to extract

Options:
  -t, --to <OUTPUT>     Output directory
  -f, --force           Overwrite existing files
  -v, --verbose         Show detailed output
  -h, --help            Print help
```

**Examples:**

```bash
# Extract to new directory
mdz extract document.mdz --to ./extracted

# Overwrite existing files
mdz extract document.mdz --to ./extracted --force

# Show extraction progress
mdz extract document.mdz --to ./extracted --verbose
```

### `mdz validate`

Validate MDZ file format and structure.

```
mdz validate <INPUT>

Arguments:
  <INPUT>               MDZ file to validate

Options:
  -d, --detailed        Show detailed validation report
  -s, --strict          Treat warnings as errors
  -h, --help            Print help
```

**Validation Checks:**
- ✅ ZIP file integrity
- ✅ Required `main.md` file presence
- ✅ Directory structure compliance
- ✅ File naming conventions
- ✅ Image format support
- ✅ CSS structure validation

**Examples:**

```bash
# Quick validation
mdz validate document.mdz

# Detailed report with file listings
mdz validate document.mdz --detailed

# Strict validation (fail on warnings)
mdz validate document.mdz --strict
```

**Sample Output:**

```
🔍 Validating MDZ file...

📋 Structure Check:
  ✓ main.md present
  ✓ img/ directory
  ✓ css/ directory
  ✓ css/style.css

📁 File Summary:
  Image files: 3
  CSS files: 1

  🖼️ Images:
    img/hero.jpg
    img/diagram.png
    img/icon.svg

  🎨 CSS Files:
    css/style.css

📊 Overall Status:
  ✓ Valid MDZ format

✓ Validation passed
```

### `mdz preview`

Generate HTML preview of MDZ documents.

```
mdz preview <INPUT>

Arguments:
  <INPUT>               MDZ file to preview

Options:
  -b, --browser         Open in browser automatically
  -o, --output <FILE>   Output HTML file path
  -c, --css <FILE>      Custom CSS file to use
  -n, --no-css          Disable CSS styling
  -h, --help            Print help
```

**Features:**
- 🎨 Full CSS styling support
- 🖼️ Embedded base64 images
- 📱 Responsive design
- 🌐 Cross-platform browser opening
- 💾 Temporary or persistent HTML output

**Examples:**

```bash
# Generate and open in browser
mdz preview document.mdz --browser

# Save to specific file
mdz preview document.mdz --output report.html

# Use custom styling
mdz preview document.mdz --css themes/dark.css --browser

# Plain HTML without styling
mdz preview document.mdz --no-css --output plain.html
```

## Global Options

All commands support these global options:

- `-h, --help` - Show help information
- `-V, --version` - Show version information
- `--verbose` - Enable verbose output (where applicable)

## File Format

### Supported Input Structure

When creating MDZ files, your source directory should follow this structure:

```
source-directory/
├── main.md           # Required: Main Markdown content
├── img/              # Optional: Image directory
│   ├── *.jpg, *.jpeg # JPEG images
│   ├── *.png         # PNG images  
│   ├── *.gif         # GIF animations
│   ├── *.svg         # SVG graphics
│   └── *.webp        # WebP images
└── css/              # Optional: Stylesheet directory
    ├── style.css     # Main stylesheet (auto-loaded)
    └── *.css         # Additional stylesheets
```

### Markdown Content Guidelines

Your `main.md` should use standard Markdown syntax:

```markdown
# Document Title

## Introduction

This is a sample MDZ document with embedded resources.

![Sample Image](img/sample.png)

## Features

- **Bold text**
- *Italic text*
- [Links](https://example.com)
- `code snippets`

### Code Blocks

```rust
fn main() {
    println!("Hello, MDZ!");
}
```

### Tables

| Feature | Status | Priority |
|---------|--------|----------|
| Parser  | ✅     | High     |
| CLI     | ✅     | High     |
| Web     | 🚧     | Medium   |
```

### Image Guidelines

For best results with images:

- **Formats**: Use JPG for photos, PNG for graphics with transparency, SVG for scalable graphics
- **Size**: Keep individual images under 2MB for better performance
- **Naming**: Use descriptive names like `hero-image.jpg`, not `img1.jpg`
- **Organization**: Use subdirectories for different image types:
  ```
  img/
  ├── photos/
  ├── diagrams/
  └── icons/
  ```

### CSS Guidelines

For styling:

- **Main stylesheet**: Place primary styles in `css/style.css` (auto-loaded)
- **Responsive**: Design for different screen sizes
- **Print-friendly**: Consider print media queries
- **Variables**: Use CSS custom properties for theming

```css
/* css/style.css */
:root {
  --primary-color: #3498db;
  --text-color: #333;
  --background: #fff;
}

body {
  font-family: system-ui, sans-serif;
  color: var(--text-color);
  background: var(--background);
}

@media (max-width: 768px) {
  body {
    padding: 16px;
    font-size: 16px;
  }
}
```

## Workflows

### Basic Document Creation

1. **Prepare content**:
   ```bash
   mkdir my-document
   cd my-document
   ```

2. **Create main.md**:
   ```bash
   echo "# My Document" > main.md
   echo "This is my MDZ document." >> main.md
   ```

3. **Add resources**:
   ```bash
   mkdir img css
   cp ~/photos/header.jpg img/
   echo "body { font-family: Arial; }" > css/style.css
   ```

4. **Create MDZ**:
   ```bash
   cd ..
   mdz create my-document.mdz --from my-document
   ```

5. **Validate and preview**:
   ```bash
   mdz validate my-document.mdz
   mdz preview my-document.mdz --browser
   ```

### Batch Processing

Process multiple documents:

```bash
#!/bin/bash
for dir in documents/*/; do
  name=$(basename "$dir")
  echo "Processing $name..."
  mdz create "output/${name}.mdz" --from "$dir"
  mdz validate "output/${name}.mdz"
done
```

### Converting Existing Documents

Convert from other formats:

```bash
# From a single markdown file with images
mkdir temp-doc
cp document.md temp-doc/main.md
mkdir temp-doc/img
cp images/* temp-doc/img/
mdz create document.mdz --from temp-doc
rm -rf temp-doc
```

## Integration

### CI/CD Pipeline

Example GitHub Actions workflow:

```yaml
name: Build MDZ Documentation

on: [push, pull_request]

jobs:
  build-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Install mdz-cli
        run: cargo install --path mdz-cli
        
      - name: Build documentation
        run: |
          mdz create docs.mdz --from ./documentation
          mdz validate docs.mdz --strict
          mdz preview docs.mdz --output docs.html
          
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: documentation
          path: |
            docs.mdz
            docs.html
```

### Shell Integration

Add to your shell profile:

```bash
# ~/.bashrc or ~/.zshrc

# Quick MDZ preview function
mdzview() {
  mdz preview "$1" --browser
}

# Quick MDZ creation from current directory
mdzpack() {
  local name=${1:-$(basename "$(pwd)")}
  mdz create "${name}.mdz" --from . --verbose
}

# Validate all MDZ files in directory
mdzcheck() {
  find . -name "*.mdz" -exec mdz validate {} \;
}
```

## Troubleshooting

### Common Issues

**"main.md not found"**
```bash
# Check source directory structure
ls -la source-directory/
# Ensure main.md exists and is not Main.md or main.MD
```

**"Invalid ZIP file"**
```bash
# Check file integrity
unzip -t document.mdz
# Re-create if corrupted
```

**"Failed to open browser"**
```bash
# Manual preview
mdz preview document.mdz --output preview.html
open preview.html  # macOS
xdg-open preview.html  # Linux
```

**Large file issues**
```bash
# Check file sizes
mdz validate document.mdz --detailed
# Optimize images before packing
```

### Performance Tips

- **Optimize images**: Compress images before adding to MDZ
- **Use appropriate compression**: Higher compression for distribution, lower for development
- **Batch operations**: Process multiple files in parallel
- **Clean directories**: Remove .DS_Store and other system files before packing

### Getting Help

```bash
# Command help
mdz help
mdz create --help
mdz validate --help

# Version information
mdz --version
```

## Examples Repository

For more examples and templates, see:
- [Basic Template](../test-document/) - Simple MDZ document structure
- Advanced examples (coming soon)
- Theme gallery (coming soon)

## License

MIT License - see LICENSE file for details.
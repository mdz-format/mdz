# MDZ - Markdown Zipped Document Format

[中文版本](README_CN.md) | English

MDZ (Markdown Zipped) is a self-contained document packaging format that solves the resource management problem of Markdown documents. It packages Markdown content, images, and styles into a single file while maintaining full compatibility with standard Markdown syntax.

## Overview

- **Self-contained**: All resources in a single file
- **Markdown compatible**: Fully compatible with standard Markdown syntax  
- **Minimal complexity**: Only three core components (main.md, img/, css/)
- **Automation-first**: Automatic resource loading with minimal manual configuration
- **Open standard**: Based on open ZIP and Markdown standards

## Project Structure

```
MDZ/
├── mdz-core/                 # Core parsing and rendering 
├── mdz-cli/                 # Command-line tool
│   ├── src/                 # CLI source code
│   ├── Cargo.toml          # CLI dependencies
│   └── README.md           # CLI documentation
├── MDZ_SPEC.md             # Format specification v1.0
├── Cargo.toml              # Workspace configuration
└── README.md               # This file
```

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/mdz-format/mdz.git
cd mdz

# Install from source
cargo install --path mdz-cli

# Or build for local use
cargo build --release
# Binary at: ./target/release/mdz-cli
```

### Basic Usage

```bash
# Create from a directory containing main.md, img/, and css/
mdz create document.mdz --from ./my-document

# Extract MDZ file
mdz extract document.mdz --to ./extracted

# Validate MDZ format
mdz validate document.mdz --detailed

# Preview in browser
mdz preview document.mdz --browser

# Preview with HTML output
mdz preview document.mdz --output document.html
```

### MDZ File Structure

```
example.mdz (ZIP file)
├── main.md        # Main Markdown content (required)
├── img/           # Image resources (optional)
│   ├── photo.jpg
│   └── diagram.png
└── css/           # Style resources (optional)
    └── style.css  # Auto-loaded main stylesheet
```

## Features

### Core Library (mdz-core)

- ✅ ZIP file parsing and validation
- ✅ Markdown parsing with pulldown-cmark
- ✅ HTML rendering with embedded resources
- ✅ Automatic CSS loading
- ✅ Image resource handling (base64 embedding)
- ✅ Comprehensive format validation
- ✅ Error handling and reporting

### CLI Tool (mdz-cli)

- ✅ Create MDZ files from directories
- ✅ Extract MDZ files to directories  
- ✅ Validate MDZ format compliance
- ✅ Generate HTML previews with embedded resources
- ✅ Cross-platform browser opening
- ✅ Colorized output and progress reporting
- ✅ Verbose mode and detailed error messages

## Supported Features

### Markdown Syntax
- Headers, paragraphs, emphasis
- Lists (ordered, unordered, task lists)
- Code blocks with syntax highlighting
- Tables and blockquotes
- Links and images
- HTML inline elements

### Image Formats
- JPEG, PNG, GIF, SVG, WebP
- Automatic base64 embedding in HTML output
- Subdirectory organization support

### CSS Support
- Automatic `css/style.css` loading
- Custom CSS override options
- Responsive design support
- Print-friendly styles

## Development

### Build

```bash
# Build all packages
cargo build

# Build in release mode
cargo build --release

# Build specific package
cargo build -p mdz-core
cargo build -p mdz-cli
```

### Test with Example

```bash
# Create test MDZ file
./target/release/mdz-cli create test.mdz --from test-document --verbose

# Validate it
./target/release/mdz-cli validate test.mdz --detailed

# Preview in browser
./target/release/mdz-cli preview test.mdz --browser
```

## Documentation

- [**Format Specification**](MDZ_SPEC.md) - Complete MDZ format specification v1.0
- [**Core Library API**](mdz-core/README.md) - mdz-core library documentation
- [**CLI Tool Guide**](mdz-cli/README.md) - mdz-cli command-line tool guide

## Performance

- Supports files up to 100MB
- Fast parsing and rendering (< 2 seconds for 100MB files)
- Memory efficient with streaming processing
- Optimized for development and production use

## Security

- ZIP bomb protection
- Path traversal attack prevention
- File size limits
- XSS protection in HTML rendering

## Compatibility

- **Rust**: 1.70+
- **Platforms**: macOS, Linux, Windows
- **Format**: MDZ Specification v1.0
- **Standards**: ZIP, Markdown, HTML5, CSS3

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Status

**Current Version**: 0.1.0  
**Status**: Production Ready ✅  
**Last Updated**: 2025-07-21
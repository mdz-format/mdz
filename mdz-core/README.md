# mdz-core

[English](README.md) | [中文版本](README_CN.md)

Core parsing and rendering library for MDZ (Markdown Zipped) documents.

## Overview

`mdz-core` provides the fundamental functionality for working with MDZ files:
- Parsing and validating MDZ ZIP archives
- Extracting Markdown content and embedded resources
- Rendering MDZ documents to HTML with automatic resource embedding
- Format validation and error handling

## Features

- **ZIP Archive Processing**: Robust ZIP file parsing with proper error handling
- **Markdown Rendering**: Full-featured Markdown rendering using `pulldown-cmark`
- **Resource Embedding**: Automatic base64 embedding of images in HTML output
- **CSS Auto-loading**: Automatic detection and application of `css/style.css`
- **Format Validation**: Comprehensive validation of MDZ format compliance
- **Error Handling**: Detailed error reporting with context

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mdz-core = "0.1.0"
```

## Quick Start

### Basic Usage

```rust
use mdz_core::{MdzParser, MdzRenderer};

// Parse an MDZ file
let document = MdzParser::parse_file("document.mdz")?;

// Render to HTML
let renderer = MdzRenderer::new();
let html = renderer.render_html(&document)?;

println!("{}", html);
```

### From Bytes

```rust
use mdz_core::MdzParser;

let mdz_data: Vec<u8> = std::fs::read("document.mdz")?;
let document = MdzParser::parse_bytes(&mdz_data)?;
```

### Validation

```rust
use mdz_core::MdzValidator;
use std::fs::File;

let file = File::open("document.mdz")?;
let result = MdzValidator::validate(file)?;

if result.is_valid() {
    println!("✅ Valid MDZ format");
} else {
    println!("❌ Validation errors: {:?}", result.errors);
}
```

### Custom Rendering Options

```rust
use mdz_core::{MdzRenderer, RenderOptions};

let options = RenderOptions {
    include_css: true,
    base64_images: true,
    custom_css: Some("body { background: #f0f0f0; }".to_string()),
    html_title: Some("My Document".to_string()),
};

let renderer = MdzRenderer::with_options(options);
let html = renderer.render_html(&document)?;
```

## API Reference

### Core Types

#### `MdzDocument`

Represents a parsed MDZ document:

```rust
pub struct MdzDocument {
    pub content: String,                           // Markdown content
    pub images: HashMap<String, Vec<u8>>,          // Image resources
    pub css: Option<String>,                       // CSS content
    pub metadata: DocumentMetadata,                // Document metadata
}
```

#### `DocumentMetadata`

Document metadata extracted during parsing:

```rust
pub struct DocumentMetadata {
    pub title: Option<String>,                     // Extracted from first heading
    pub created_at: Option<DateTime<Utc>>,         // Creation timestamp
    pub modified_at: Option<DateTime<Utc>>,        // Modification timestamp
    pub version: String,                           // Format version
}
```

#### `ValidationResult`

Validation result with detailed information:

```rust
pub struct ValidationResult {
    pub has_main_md: bool,                         // main.md presence
    pub has_img_dir: bool,                         // img/ directory presence
    pub has_css_dir: bool,                         // css/ directory presence
    pub has_main_css: bool,                        // css/style.css presence
    pub image_files: Vec<String>,                  // List of image files
    pub css_files: Vec<String>,                    // List of CSS files
    pub errors: Vec<String>,                       // Validation errors
    pub warnings: Vec<String>,                     // Validation warnings
}
```

### Parser API

#### `MdzParser`

Main parser for MDZ files:

```rust
impl MdzParser {
    /// Parse from any Read + Seek source
    pub fn parse<R: Read + Seek>(reader: R) -> Result<MdzDocument>;
    
    /// Parse from byte slice
    pub fn parse_bytes(data: &[u8]) -> Result<MdzDocument>;
    
    /// Parse from file path
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<MdzDocument>;
}
```

### Renderer API

#### `MdzRenderer`

HTML renderer with customization options:

```rust
impl MdzRenderer {
    /// Create renderer with default options
    pub fn new() -> Self;
    
    /// Create renderer with custom options
    pub fn with_options(options: RenderOptions) -> Self;
    
    /// Render document to HTML
    pub fn render_html(&self, document: &MdzDocument) -> Result<String>;
    
    /// Render to plain text (strip formatting)
    pub fn render_text(&self, document: &MdzDocument) -> Result<String>;
}
```

#### `RenderOptions`

Rendering customization options:

```rust
pub struct RenderOptions {
    pub include_css: bool,                         // Include CSS in output
    pub base64_images: bool,                       // Embed images as base64
    pub custom_css: Option<String>,                // Override CSS
    pub html_title: Option<String>,                // HTML document title
}
```

### Validator API

#### `MdzValidator`

Format validator for MDZ files:

```rust
impl MdzValidator {
    /// Validate MDZ file format
    pub fn validate<R: Read + Seek>(reader: R) -> Result<ValidationResult>;
}
```

## Error Handling

All operations return `Result<T, MdzError>` where `MdzError` provides detailed error information:

```rust
pub enum MdzError {
    Io(std::io::Error),                           // I/O errors
    Zip(zip::result::ZipError),                   // ZIP format errors
    Json(serde_json::Error),                      // JSON parsing errors
    InvalidFormat(String),                        // Invalid MDZ format
    MissingFile(String),                          // Required file missing
    ResourceNotFound(String),                     // Resource not found
    Validation(String),                           // Validation error
    Parse(String),                                // Parsing error
    Render(String),                               // Rendering error
}
```

## Examples

### Complete Example

```rust
use mdz_core::{MdzParser, MdzRenderer, MdzValidator, RenderOptions};
use std::fs;

fn process_mdz_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Validate the file
    let file = fs::File::open(path)?;
    let validation = MdzValidator::validate(file)?;
    
    if !validation.is_valid() {
        println!("Validation errors:");
        for error in &validation.errors {
            println!("  ❌ {}", error);
        }
        return Ok(());
    }
    
    println!("✅ Validation passed");
    
    // 2. Parse the document
    let document = MdzParser::parse_file(path)?;
    
    println!("📄 Document: {}", 
        document.metadata.title.as_deref().unwrap_or("Untitled"));
    println!("🖼️  Images: {}", document.images.len());
    println!("🎨 Has CSS: {}", document.css.is_some());
    
    // 3. Render to HTML
    let options = RenderOptions {
        include_css: true,
        base64_images: true,
        custom_css: None,
        html_title: document.metadata.title.clone(),
    };
    
    let renderer = MdzRenderer::with_options(options);
    let html = renderer.render_html(&document)?;
    
    // 4. Save rendered HTML
    let output_path = path.replace(".mdz", ".html");
    fs::write(&output_path, html)?;
    
    println!("💾 Saved HTML to: {}", output_path);
    
    Ok(())
}
```

### Working with Images

```rust
use mdz_core::{MdzDocument, MdzParser};

fn extract_images(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let document = MdzParser::parse_file(path)?;
    
    for (image_path, image_data) in &document.images {
        println!("Image: {} ({} bytes)", image_path, image_data.len());
        
        // Save image to disk
        let output_path = format!("extracted_{}", 
            image_path.replace("/", "_"));
        std::fs::write(&output_path, image_data)?;
    }
    
    Ok(())
}
```

### Custom CSS Processing

```rust
use mdz_core::{MdzRenderer, RenderOptions};

fn render_with_theme(document: &MdzDocument, theme: &str) -> Result<String, Box<dyn std::error::Error>> {
    let custom_css = match theme {
        "dark" => Some(include_str!("themes/dark.css").to_string()),
        "minimal" => Some(include_str!("themes/minimal.css").to_string()),
        _ => None,
    };
    
    let options = RenderOptions {
        include_css: true,
        base64_images: true,
        custom_css,
        html_title: None,
    };
    
    let renderer = MdzRenderer::with_options(options);
    Ok(renderer.render_html(document)?)
}
```

## Dependencies

- `zip` - ZIP archive handling
- `pulldown-cmark` - Markdown parsing and rendering
- `serde` + `serde_json` - Serialization support
- `chrono` - Date/time handling
- `thiserror` - Error handling
- `base64` - Base64 encoding for images
- `regex` - Text processing
- `mime_guess` - MIME type detection

## Format Support

Supports MDZ format specification v1.0:

- **Required**: `main.md` file
- **Optional**: `img/` directory with images (jpg, jpeg, png, gif, svg, webp)
- **Optional**: `css/` directory with `style.css`

## Performance

- Optimized for files up to 100MB
- Memory-efficient streaming for large archives
- Lazy loading of resources
- Automatic compression detection

## Thread Safety

All public APIs are thread-safe and can be used concurrently.

## License

MIT License - see LICENSE file for details.
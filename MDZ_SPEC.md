# MDZ Format Specification v1.0

[中文版本](MDZ_SPEC_ZH.md) | English

## 1. Overview

MDZ (Markdown Zipped) is a self-contained document packaging format designed to solve the resource management problems of Markdown documents with images and styles. The MDZ format is based on ZIP compression while maintaining maximum compatibility with traditional Markdown.

### 1.1 Design Principles

- **Markdown Compatible**: Core content is fully compatible with standard Markdown syntax
- **Minimal Complexity**: Contains only three necessary core components
- **Self-contained**: Single file contains all dependent resources
- **Automation-first**: Reduce manual configuration, renderers automatically handle resource loading
- **Open Standard**: Based on open ZIP and Markdown standards

### 1.2 File Extension

- **Recommended**: `.mdz`
- **MIME Type**: `application/mdz+zip`

## 2. Core Structure

MDZ file is a ZIP archive containing the following **three core parts**:

```
example.mdz (ZIP archive)
├── main.md        # Main Markdown content (required)
├── img/           # Image resource directory (optional)
└── css/           # Style resource directory (optional)
```

### 2.1 File Structure Specification

#### 2.1.1 main.md (Required)

- **Purpose**: Contains the main Markdown content of the document
- **Format**: Standard Markdown syntax
- **Encoding**: UTF-8
- **Naming**: Must be `main.md` (case-sensitive)

#### 2.1.2 img/ (Optional)

- **Purpose**: Store all images referenced in the document
- **Supported Formats**: jpg, jpeg, png, gif, svg, webp
- **Reference Method**: `![alt text](img/filename.ext)`
- **Subdirectories**: Subdirectory structure is supported

#### 2.1.3 css/ (Optional)

- **Purpose**: Store custom style files
- **Supported Formats**: css
- **Auto-loading**: Renderer automatically finds and loads `css/style.css`
- **Default Styles**: If no CSS file is provided, use system default Markdown styles

## 3. Detailed Specification

### 3.1 main.md Specification

#### 3.1.1 Content Requirements

```markdown
# Document Title

This is a standard Markdown document.

![Example Image](img/example.png)

## Section Title

Supports all standard Markdown syntax:
- Lists
- **Bold**
- *Italic*
- `code`
- [Links](https://example.com)
```

#### 3.1.2 Image References

```markdown
# Relative path reference (recommended)
![Image Description](img/photo.jpg)
![With subdirectory](img/screenshots/screen1.png)

# Supported image formats
![JPEG Image](img/image.jpeg)
![PNG Image](img/image.png)
![GIF Animation](img/animation.gif)
![SVG Vector](img/icon.svg)
![WebP Image](img/modern.webp)
```

### 3.2 img/ Directory Specification

#### 3.2.1 Directory Structure

```
img/
├── photo1.jpg
└── photo2.png
```

#### 3.2.2 File Naming

- **Character Limitation**: Use ASCII characters, numbers, hyphens, underscores
- **Avoid Spaces**: Use hyphens or underscores instead of spaces
- **Lowercase Recommended**: Recommend using lowercase filenames
- **Meaningful**: Filenames should be descriptive

**Examples**:
```
✅ Recommended:
- hero-image.jpg
- user_avatar.png
- button-icon.svg

❌ Avoid:
- image 1.jpg (contains spaces and non-ASCII)
- IMG_001.JPG (all uppercase)
- a.png (meaningless naming)
```

### 3.3 css/ Directory Specification

#### 3.3.1 Auto-loading Mechanism

- **Core File**: `css/style.css` is the default style file
- **Auto-loading**: MDZ renderer automatically finds and loads this file
- **No Reference Needed**: No need to manually reference CSS files in main.md
- **Priority**: If style.css exists, automatically apply it; otherwise use system default styles

#### 3.3.2 Directory Structure (v1.0 Basic Version)

```
css/
└── style.css       # Auto-loaded main style file
```

#### 3.3.3 Style File Example

```css
/* css/style.css */
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
    line-height: 1.6;
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

h1, h2, h3 {
    color: #333;
}

img {
    max-width: 100%;
    height: auto;
}

code {
    background-color: #f5f5f5;
    padding: 2px 4px;
    border-radius: 3px;
}
```

## 4. Compatibility Specification

### 4.1 Markdown Compatibility

The main.md file of MDZ format must be a completely valid Markdown document that can be correctly parsed by any standard Markdown parser.

#### 4.1.1 Supported Markdown Features

- **Basic Syntax**: Headers, paragraphs, emphasis, lists, links, images
- **Extended Syntax**: Tables, code blocks, task lists
- **Inline HTML**: Supports inline HTML tags

#### 4.1.2 Path Resolution Rules

```markdown
# Relative path (internal to MDZ format)
![Internal Image](img/photo.jpg)

# Absolute URL (external resources)
![External Image](https://example.com/image.jpg)

# Relative path (traditional Markdown compatibility)
![Same Directory](./photo.jpg)  # Resolves to img/photo.jpg
![Parent Directory](../images/photo.jpg)  # Not recommended, but supported
```

**CSS Loading Notes**:
- CSS style files do not need to be referenced in Markdown
- Renderer automatically finds `css/style.css` and applies it
- Maintains purity of Markdown content

### 4.2 ZIP Format Compatibility

#### 4.2.1 Compression Requirements

- **Compression Method**: Standard ZIP compression (deflate)
- **Compression Level**: Recommend using medium compression (level 6)
- **File Encoding**: UTF-8 filename encoding

#### 4.2.2 Extraction Compatibility

MDZ files can be extracted by any standard ZIP tool:

```bash
# Extract MDZ file
unzip document.mdz -d extracted/

# View contents
cd extracted/
cat main.md
ls img/
ls css/
```

## 5. Creation and Usage

### 5.1 Manual MDZ File Creation

#### 5.1.1 Creation Steps

```bash
# 1. Create working directory
mkdir my-document
cd my-document

# 2. Create main.md file
echo "# My Document" > main.md
echo "![Example](img/example.png)" >> main.md

# 3. Create resource directories
mkdir img css

# 4. Add images and styles
cp /path/to/image.png img/example.png
echo "body { font-family: Arial; }" > css/style.css

# 5. Package as MDZ file
zip -r ../my-document.mdz main.md img/ css/
```

#### 5.1.2 MDZ File Validation

```bash
# Validate ZIP structure
unzip -l document.mdz

# Should display similar to:
# Archive:  document.mdz
#   Length      Date    Time    Name
# ---------  ---------- -----   ----
#      123  2025-01-01 10:00   main.md
#        0  2025-01-01 10:00   img/
#     5432  2025-01-01 10:00   img/example.png
#        0  2025-01-01 10:00   css/
#       89  2025-01-01 10:00   css/style.css
```

### 5.2 Content Guidelines

#### 5.2.1 main.md Writing Guidelines

```markdown
# Document Title

## Introduction
This is the introduction section of the document.

## Main Content
![Important Chart](img/chart.png)

Detailed explanation text...

### Subsection
- Point 1
- Point 2

## Conclusion
Summary content.

---
*Created: 2025-01-01*
```

#### 5.2.2 Image Optimization Recommendations

- **Resolution**: Choose appropriate resolution based on usage
- **Format Selection**: Use JPEG for photos, PNG for charts, SVG for icons
- **File Size**: Recommend individual images not exceed 2MB
- **Total Size**: Entire MDZ file recommended not to exceed 50MB

## 6. Implementation Suggestions

### 6.1 Parser Implementation

#### 6.1.1 Basic Parsing Process

```
1. Validate ZIP file format
2. Check if main.md exists
3. Parse main.md content
4. Identify img/ directory, parse image reference relationships
5. Automatically find css/style.css file
6. If style.css exists, automatically load styles
7. Render final output (apply custom styles or default styles)
```

#### 6.1.2 Error Handling

- **Missing main.md**: Throw format error
- **Corrupted ZIP**: Prompt file corruption
- **Missing Resources**: Show placeholders or warnings
- **Encoding Issues**: Attempt automatic encoding detection

### 6.2 Tool Development Suggestions

#### 6.2.1 Command-line Tools

```bash
# Create MDZ file
mdz create my-document.mdz --from ./source-folder

# Extract MDZ file
mdz extract document.mdz --to ./output-folder

# Validate MDZ file
mdz validate document.mdz

# Preview MDZ file
mdz preview document.mdz --browser
```

#### 6.2.2 Editor Integration

- **VS Code Plugin**: Provide MDZ file preview and editing
- **Online Editor**: Web-based MDZ editor
- **Desktop Application**: Dedicated MDZ editor

## 7. Version Control

### 7.1 Specification Version

Current specification version: **v1.0**

### 7.2 Backward Compatibility

MDZ format promises:
- **Major version upgrades**: May include incompatible changes
- **Minor version upgrades**: Maintain backward compatibility
- **Patch versions**: Only fix errors and clarifications

### 7.3 Extension Plans

Future possible extensions (maintaining backward compatibility):
- **CSS Extensions**: Support multiple CSS files, theme switching, print styles
- Support JavaScript files (js/ directory)
- Support font files (fonts/ directory)
- Support multilingual documents (i18n/ directory)
- Support metadata files (metadata.json)

## 8. Security Considerations

### 8.1 Content Security

- **Script Execution**: Do not execute JavaScript code by default
- **External Resources**: Handle external URL references carefully
- **File Size**: Limit extracted file size
- **ZIP Bombs**: Protect against ZIP bomb attacks

### 8.2 Privacy Protection

- **Metadata**: Clean sensitive metadata from ZIP files
- **Local Priority**: Encourage local processing, reduce cloud dependencies
- **Transparency**: All content is inspectable text and resources

## 9. Best Practices

### 9.1 Document Organization

```markdown
# Recommended main.md structure
# Document Title

## Overview
Brief introduction to document purpose

## Table of Contents
- [Chapter 1](#chapter-1)
- [Chapter 2](#chapter-2)
- [Appendix](#appendix)

## Chapter 1
![Concept Diagram](img/concept.png)

Content...

## Chapter 2
Detailed content...

## Appendix
Supplementary materials...
```

### 9.2 Resource Management

```
# Recommended directory structure
img/
├── diagrams/     # Charts and diagrams
├── screenshots/  # Screenshots
├── photos/       # Photos
└── icons/        # Icons

css/
└── style.css     # Auto-loaded main style file

# Future versions may support:
# ├── print.css   # Print styles (v2.0 planned)
# └── themes/     # Theme styles (v2.0 planned)
```

### 9.3 Performance Optimization

- **Image Compression**: Use appropriate compression ratios
- **CSS Minimization**: Remove unused styles
- **File Naming**: Use descriptive file names
- **Directory Structure**: Organize resource files reasonably

---

## Conclusion

The MDZ format aims to become a simple, practical, compatible Markdown document packaging standard. Through the minimalist architecture of main.md + img/ + css/, it maintains complete compatibility with traditional Markdown while solving the core problem of resource management.
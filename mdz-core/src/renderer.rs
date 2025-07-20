use crate::{MdzDocument, Result};
use pulldown_cmark::{Parser, Options, html};
use std::collections::HashMap;
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine;

/// MDZ document renderer
pub struct MdzRenderer {
    options: RenderOptions,
}

/// Rendering options
#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub include_css: bool,
    pub base64_images: bool,
    pub custom_css: Option<String>,
    pub html_title: Option<String>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            include_css: true,
            base64_images: true,
            custom_css: None,
            html_title: None,
        }
    }
}

impl MdzRenderer {
    /// Create new renderer with default options
    pub fn new() -> Self {
        Self {
            options: RenderOptions::default(),
        }
    }

    /// Create renderer with custom options
    pub fn with_options(options: RenderOptions) -> Self {
        Self { options }
    }

    /// Render MDZ document to HTML
    pub fn render_html(&self, document: &MdzDocument) -> Result<String> {
        let processed_markdown = self.process_image_links(&document.content, &document.images)?;
        
        // Configure markdown parser
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(&processed_markdown, options);
        
        // Convert to HTML
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // Wrap in complete HTML document
        let full_html = self.create_html_document(&html_output, document)?;

        Ok(full_html)
    }

    /// Process image links in markdown
    fn process_image_links(&self, content: &str, images: &HashMap<String, Vec<u8>>) -> Result<String> {
        let mut processed = content.to_string();

        if self.options.base64_images {
            // Convert image references to base64 data URLs
            for (path, data) in images {
                let mime_type = self.detect_mime_type(path);
                let base64_data = base64_engine.encode(data);
                let data_url = format!("data:{};base64,{}", mime_type, base64_data);
                
                // Replace all occurrences of the image path
                processed = processed.replace(&format!("img/{}", path.strip_prefix("img/").unwrap_or(path)), &data_url);
                processed = processed.replace(path, &data_url);
            }
        }

        Ok(processed)
    }

    /// Detect MIME type from file extension
    fn detect_mime_type(&self, path: &str) -> &'static str {
        match path.split('.').last().unwrap_or("").to_lowercase().as_str() {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "webp" => "image/webp",
            _ => "application/octet-stream",
        }
    }

    /// Create complete HTML document
    fn create_html_document(&self, body: &str, document: &MdzDocument) -> Result<String> {
        let title = self.options.html_title.as_ref()
            .or(document.metadata.title.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("MDZ Document");

        let css = if self.options.include_css {
            self.get_css_content(document)
        } else {
            String::new()
        };

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
{}
    </style>
</head>
<body>
{}
</body>
</html>"#,
            title,
            css,
            body
        );

        Ok(html)
    }

    /// Get CSS content (custom, document, or default)
    fn get_css_content(&self, document: &MdzDocument) -> String {
        if let Some(custom_css) = &self.options.custom_css {
            custom_css.clone()
        } else if let Some(doc_css) = &document.css {
            doc_css.clone()
        } else {
            self.default_css().to_string()
        }
    }

    /// Default CSS styles
    fn default_css(&self) -> &'static str {
        r#"
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
    line-height: 1.6;
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    color: #333;
}

h1, h2, h3, h4, h5, h6 {
    color: #2c3e50;
    margin-top: 1.5em;
    margin-bottom: 0.5em;
}

h1 {
    border-bottom: 2px solid #3498db;
    padding-bottom: 0.3em;
}

h2 {
    border-bottom: 1px solid #bdc3c7;
    padding-bottom: 0.2em;
}

img {
    max-width: 100%;
    height: auto;
    display: block;
    margin: 1em auto;
    border-radius: 4px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

code {
    background-color: #f8f9fa;
    padding: 2px 6px;
    border-radius: 3px;
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
    font-size: 0.9em;
}

pre {
    background-color: #f8f9fa;
    padding: 16px;
    border-radius: 6px;
    overflow-x: auto;
    border-left: 4px solid #3498db;
}

pre code {
    background: none;
    padding: 0;
}

blockquote {
    border-left: 4px solid #3498db;
    margin: 1em 0;
    padding-left: 1em;
    color: #666;
    font-style: italic;
}

table {
    border-collapse: collapse;
    width: 100%;
    margin: 1em 0;
}

th, td {
    border: 1px solid #ddd;
    padding: 8px 12px;
    text-align: left;
}

th {
    background-color: #f8f9fa;
    font-weight: 600;
}

a {
    color: #3498db;
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}

ul, ol {
    padding-left: 1.5em;
}

li {
    margin: 0.25em 0;
}

hr {
    border: none;
    border-top: 1px solid #ddd;
    margin: 2em 0;
}
        "#
    }

    /// Render to plain text (strip HTML)
    pub fn render_text(&self, document: &MdzDocument) -> Result<String> {
        // Simple implementation - remove markdown formatting
        let mut text = document.content.clone();
        
        // Remove headers
        text = regex::Regex::new(r"^#+\s*").unwrap().replace_all(&text, "").to_string();
        
        // Remove emphasis
        text = regex::Regex::new(r"\*\*([^*]+)\*\*").unwrap().replace_all(&text, "$1").to_string();
        text = regex::Regex::new(r"\*([^*]+)\*").unwrap().replace_all(&text, "$1").to_string();
        
        // Remove links
        text = regex::Regex::new(r"\[([^\]]+)\]\([^)]+\)").unwrap().replace_all(&text, "$1").to_string();
        
        // Remove images
        text = regex::Regex::new(r"!\[[^\]]*\]\([^)]+\)").unwrap().replace_all(&text, "").to_string();
        
        Ok(text)
    }
}

impl Default for MdzRenderer {
    fn default() -> Self {
        Self::new()
    }
}
pub mod parser;
pub mod renderer;
pub mod validator;
pub mod error;

pub use parser::MdzParser;
pub use renderer::{MdzRenderer, RenderOptions};
pub use validator::{MdzValidator, ValidationResult};
pub use error::{MdzError, Result};

/// MDZ document structure
#[derive(Debug, Clone)]
pub struct MdzDocument {
    pub content: String,
    pub images: std::collections::HashMap<String, Vec<u8>>,
    pub css: Option<String>,
    pub metadata: DocumentMetadata,
}

/// Document metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: String,
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        Self {
            title: None,
            created_at: None,
            modified_at: None,
            version: "1.0".to_string(),
        }
    }
}

impl MdzDocument {
    /// Create a new MDZ document
    pub fn new(content: String) -> Self {
        Self {
            content,
            images: std::collections::HashMap::new(),
            css: None,
            metadata: DocumentMetadata::default(),
        }
    }

    /// Add an image to the document
    pub fn add_image(&mut self, path: String, data: Vec<u8>) {
        self.images.insert(path, data);
    }

    /// Set CSS content
    pub fn set_css(&mut self, css: String) {
        self.css = Some(css);
    }
}

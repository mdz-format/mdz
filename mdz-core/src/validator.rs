use crate::{MdzError, Result};
use std::io::Read;
use zip::ZipArchive;

/// MDZ format validator
pub struct MdzValidator;

impl MdzValidator {
    /// Validate MDZ file format
    pub fn validate<R: Read + std::io::Seek>(reader: R) -> Result<ValidationResult> {
        let mut archive = ZipArchive::new(reader)?;
        let mut result = ValidationResult::new();

        // Check for main.md
        if archive.by_name("main.md").is_ok() {
            result.has_main_md = true;
        } else {
            result.errors.push("Missing required file: main.md".to_string());
        }

        // Check directory structure
        let mut has_img_dir = false;
        let mut has_css_dir = false;
        let mut image_files = Vec::new();
        let mut css_files = Vec::new();

        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            let name = file.name();

            if name.starts_with("img/") {
                has_img_dir = true;
                if !name.ends_with('/') {
                    image_files.push(name.to_string());
                }
            } else if name.starts_with("css/") {
                has_css_dir = true;
                if !name.ends_with('/') {
                    css_files.push(name.to_string());
                }
            }

            // Validate file paths
            if let Err(e) = Self::validate_file_path(name) {
                result.warnings.push(format!("Invalid file path '{}': {}", name, e));
            }
        }

        result.has_img_dir = has_img_dir;
        result.has_css_dir = has_css_dir;
        result.image_files = image_files;
        result.css_files = css_files;

        // Validate image formats
        for img_file in &result.image_files {
            if !Self::is_valid_image_format(img_file) {
                result.warnings.push(format!("Unsupported image format: {}", img_file));
            }
        }

        // Check for style.css
        if result.css_files.contains(&"css/style.css".to_string()) {
            result.has_main_css = true;
        }

        Ok(result)
    }

    /// Validate file path according to MDZ specification
    fn validate_file_path(path: &str) -> Result<()> {
        if path.contains("..") {
            return Err(MdzError::Validation("Path traversal not allowed".to_string()));
        }

        if path.contains(' ') {
            return Err(MdzError::Validation("Spaces in file paths not recommended".to_string()));
        }

        if path.chars().any(|c| !c.is_ascii()) {
            return Err(MdzError::Validation("Non-ASCII characters not recommended".to_string()));
        }

        Ok(())
    }

    /// Check if file has valid image format
    fn is_valid_image_format(path: &str) -> bool {
        let valid_extensions = ["jpg", "jpeg", "png", "gif", "svg", "webp"];
        if let Some(ext) = path.split('.').last() {
            valid_extensions.contains(&ext.to_lowercase().as_str())
        } else {
            false
        }
    }
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub has_main_md: bool,
    pub has_img_dir: bool,
    pub has_css_dir: bool,
    pub has_main_css: bool,
    pub image_files: Vec<String>,
    pub css_files: Vec<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    fn new() -> Self {
        Self {
            has_main_md: false,
            has_img_dir: false,
            has_css_dir: false,
            has_main_css: false,
            image_files: Vec::new(),
            css_files: Vec::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Check if validation passed
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty() && self.has_main_md
    }
}
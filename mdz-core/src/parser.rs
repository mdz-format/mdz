use crate::{MdzDocument, MdzError, Result, DocumentMetadata};
use std::collections::HashMap;
use std::io::{Read, Cursor};
use zip::ZipArchive;

/// MDZ file parser
pub struct MdzParser;

impl MdzParser {
    /// Parse MDZ file from reader
    pub fn parse<R: Read + std::io::Seek>(reader: R) -> Result<MdzDocument> {
        let mut archive = ZipArchive::new(reader)?;
        
        // Read main.md
        let content = Self::read_main_md(&mut archive)?;
        
        // Parse metadata from content (extract title from first heading)
        let metadata = Self::extract_metadata(&content);
        
        // Create document
        let mut document = MdzDocument {
            content,
            images: HashMap::new(),
            css: None,
            metadata,
        };

        // Load images
        document.images = Self::load_images(&mut archive)?;

        // Load CSS
        document.css = Self::load_css(&mut archive)?;

        Ok(document)
    }

    /// Read main.md content
    fn read_main_md<R: Read + std::io::Seek>(archive: &mut ZipArchive<R>) -> Result<String> {
        let mut file = archive.by_name("main.md")
            .map_err(|_| MdzError::MissingFile("main.md".to_string()))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        Ok(content)
    }

    /// Extract metadata from markdown content
    fn extract_metadata(content: &str) -> DocumentMetadata {
        let mut metadata = DocumentMetadata::default();
        
        // Extract title from first heading
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("# ") {
                metadata.title = Some(line[2..].trim().to_string());
                break;
            }
        }

        metadata.modified_at = Some(chrono::Utc::now());
        metadata
    }

    /// Load all images from the archive
    fn load_images<R: Read + std::io::Seek>(archive: &mut ZipArchive<R>) -> Result<HashMap<String, Vec<u8>>> {
        let mut images = HashMap::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_name = file.name().to_string();

            if file_name.starts_with("img/") && !file_name.ends_with('/') {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                images.insert(file_name, buffer);
            }
        }

        Ok(images)
    }

    /// Load CSS content
    fn load_css<R: Read + std::io::Seek>(archive: &mut ZipArchive<R>) -> Result<Option<String>> {
        match archive.by_name("css/style.css") {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                Ok(Some(content))
            }
            Err(_) => Ok(None),
        }
    }

    /// Parse MDZ file from bytes
    pub fn parse_bytes(data: &[u8]) -> Result<MdzDocument> {
        let cursor = Cursor::new(data);
        Self::parse(cursor)
    }

    /// Parse MDZ file from file path
    pub fn parse_file<P: AsRef<std::path::Path>>(path: P) -> Result<MdzDocument> {
        let file = std::fs::File::open(path)?;
        Self::parse(file)
    }
}
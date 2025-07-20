use anyhow::{Result, Context};
use clap::Args;
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;
use zip::{ZipWriter, write::FileOptions};

#[derive(Args)]
pub struct CreateCommand {
    /// Output MDZ file path
    pub output: PathBuf,
    
    /// Source directory containing markdown and resources
    #[arg(long, short)]
    pub from: PathBuf,
    
    /// Compression level (0-9)
    #[arg(long, default_value = "6")]
    pub compression: u8,
    
    /// Verbose output
    #[arg(long, short)]
    pub verbose: bool,
}

impl CreateCommand {
    pub async fn execute(self) -> Result<()> {
        println!("{} Creating MDZ file...", "●".cyan());
        
        if !self.from.exists() {
            anyhow::bail!("Source directory does not exist: {}", self.from.display());
        }

        if !self.from.is_dir() {
            anyhow::bail!("Source path is not a directory: {}", self.from.display());
        }

        // Check for main.md
        let main_md_path = self.from.join("main.md");
        if !main_md_path.exists() {
            anyhow::bail!("Required file 'main.md' not found in source directory");
        }

        // Create output directory if needed
        if let Some(parent) = self.output.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create output directory")?;
        }

        // Create ZIP file
        let file = fs::File::create(&self.output)
            .context("Failed to create output file")?;
        
        let mut zip = ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .compression_level(Some(self.compression as i32));

        // Add main.md
        self.add_file_to_zip(&mut zip, &main_md_path, "main.md", &options)?;
        if self.verbose {
            println!("  {} Added main.md", "✓".green());
        }

        // Add img/ directory if exists
        let img_dir = self.from.join("img");
        if img_dir.exists() && img_dir.is_dir() {
            self.add_directory_to_zip(&mut zip, &img_dir, "img", &options)?;
            if self.verbose {
                println!("  {} Added img/ directory", "✓".green());
            }
        }

        // Add css/ directory if exists
        let css_dir = self.from.join("css");
        if css_dir.exists() && css_dir.is_dir() {
            self.add_directory_to_zip(&mut zip, &css_dir, "css", &options)?;
            if self.verbose {
                println!("  {} Added css/ directory", "✓".green());
            }
        }

        zip.finish().context("Failed to finalize ZIP file")?;

        println!("{} MDZ file created: {}", "✓".green(), self.output.display());
        
        // Show file size
        let size = fs::metadata(&self.output)?.len();
        println!("  Size: {} bytes", size);

        Ok(())
    }

    fn add_file_to_zip<W: Write + std::io::Seek>(
        &self,
        zip: &mut ZipWriter<W>,
        file_path: &Path,
        zip_path: &str,
        options: &FileOptions,
    ) -> Result<()> {
        let content = fs::read(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;
        
        zip.start_file(zip_path, *options)
            .context("Failed to start ZIP file entry")?;
        
        zip.write_all(&content)
            .context("Failed to write file content to ZIP")?;
        
        Ok(())
    }

    fn add_directory_to_zip<W: Write + std::io::Seek>(
        &self,
        zip: &mut ZipWriter<W>,
        dir_path: &Path,
        zip_prefix: &str,
        options: &FileOptions,
    ) -> Result<()> {
        for entry in fs::read_dir(dir_path)
            .with_context(|| format!("Failed to read directory: {}", dir_path.display()))?
        {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            
            if path.is_file() {
                let zip_path = format!("{}/{}", zip_prefix, name_str);
                self.add_file_to_zip(zip, &path, &zip_path, options)?;
                
                if self.verbose {
                    println!("    {} {}", "·".blue(), zip_path);
                }
            } else if path.is_dir() {
                // Recursively add subdirectories
                let sub_prefix = format!("{}/{}", zip_prefix, name_str);
                self.add_directory_to_zip(zip, &path, &sub_prefix, options)?;
            }
        }
        
        Ok(())
    }
}
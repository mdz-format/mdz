use anyhow::{Result, Context};
use clap::Args;
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::io::Read;
use zip::ZipArchive;

#[derive(Args)]
pub struct ExtractCommand {
    /// MDZ file to extract
    pub input: PathBuf,
    
    /// Output directory
    #[arg(long, short)]
    pub to: PathBuf,
    
    /// Overwrite existing files
    #[arg(long)]
    pub force: bool,
    
    /// Verbose output
    #[arg(long, short)]
    pub verbose: bool,
}

impl ExtractCommand {
    pub async fn execute(self) -> Result<()> {
        println!("{} Extracting MDZ file...", "●".cyan());
        
        if !self.input.exists() {
            anyhow::bail!("Input file does not exist: {}", self.input.display());
        }

        if self.to.exists() && !self.force {
            anyhow::bail!("Output directory already exists. Use --force to overwrite.");
        }

        // Create output directory
        fs::create_dir_all(&self.to)
            .context("Failed to create output directory")?;

        // Open ZIP file
        let file = fs::File::open(&self.input)
            .context("Failed to open input file")?;
        
        let mut archive = ZipArchive::new(file)
            .context("Failed to read ZIP archive")?;

        // Extract all files
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .context("Failed to read ZIP entry")?;
            
            let outpath = self.to.join(file.name());
            
            if file.is_dir() {
                if self.verbose {
                    println!("  {} Creating directory: {}", "·".blue(), file.name());
                }
                fs::create_dir_all(&outpath)
                    .with_context(|| format!("Failed to create directory: {}", outpath.display()))?;
            } else {
                if self.verbose {
                    println!("  {} Extracting: {}", "·".blue(), file.name());
                }
                
                // Create parent directory if needed
                if let Some(parent) = outpath.parent() {
                    fs::create_dir_all(parent)
                        .with_context(|| format!("Failed to create parent directory: {}", parent.display()))?;
                }
                
                // Extract file content
                let mut content = Vec::new();
                file.read_to_end(&mut content)
                    .context("Failed to read file content")?;
                
                fs::write(&outpath, content)
                    .with_context(|| format!("Failed to write file: {}", outpath.display()))?;
            }
        }

        println!("{} Extracted to: {}", "✓".green(), self.to.display());
        
        // Show extracted file count
        println!("  Files extracted: {}", archive.len());

        Ok(())
    }
}
use anyhow::{Result, Context};
use clap::Args;
use colored::*;
use mdz_core::{MdzParser, MdzRenderer, RenderOptions};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Args)]
pub struct PreviewCommand {
    /// MDZ file to preview
    pub input: PathBuf,
    
    /// Open in browser automatically
    #[arg(long, short)]
    pub browser: bool,
    
    /// Output HTML file path (optional)
    #[arg(long, short)]
    pub output: Option<PathBuf>,
    
    /// Custom CSS file to use instead of document CSS
    #[arg(long)]
    pub css: Option<PathBuf>,
    
    /// Disable CSS styling
    #[arg(long)]
    pub no_css: bool,
}

impl PreviewCommand {
    pub async fn execute(self) -> Result<()> {
        println!("{} Generating preview...", "●".cyan());
        
        if !self.input.exists() {
            anyhow::bail!("Input file does not exist: {}", self.input.display());
        }

        // Parse MDZ file
        let document = MdzParser::parse_file(&self.input)
            .context("Failed to parse MDZ file")?;

        // Prepare render options
        let mut options = RenderOptions::default();
        options.include_css = !self.no_css;
        
        if let Some(css_path) = &self.css {
            let custom_css = fs::read_to_string(css_path)
                .context("Failed to read custom CSS file")?;
            options.custom_css = Some(custom_css);
        }

        // Render to HTML
        let renderer = MdzRenderer::with_options(options);
        let html = renderer.render_html(&document)
            .context("Failed to render HTML")?;

        // Determine output path
        let output_path = if let Some(ref output) = self.output {
            output.clone()
        } else {
            // Create temporary file
            let temp_dir = std::env::temp_dir();
            let filename = self.input.file_stem()
                .unwrap_or_else(|| std::ffi::OsStr::new("preview"))
                .to_string_lossy();
            temp_dir.join(format!("{}.html", filename))
        };

        // Write HTML file
        fs::write(&output_path, html)
            .context("Failed to write HTML file")?;

        println!("{} Preview generated: {}", "✓".green(), output_path.display());

        // Open in browser if requested
        if self.browser {
            self.open_in_browser(&output_path)?;
        } else {
            println!("  {} Use --browser to open automatically", "💡".to_string());
        }

        Ok(())
    }

    fn open_in_browser(&self, path: &PathBuf) -> Result<()> {
        let path_str = path.to_string_lossy();
        
        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(&*path_str)
                .spawn()
                .context("Failed to open browser on macOS")?;
        }
        
        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(["/C", "start", "", &path_str])
                .spawn()
                .context("Failed to open browser on Windows")?;
        }
        
        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open")
                .arg(&*path_str)
                .spawn()
                .context("Failed to open browser on Linux")?;
        }

        println!("  {} Opened in browser", "🌐".to_string());
        Ok(())
    }
}
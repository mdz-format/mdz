use anyhow::{Result, Context};
use clap::Args;
use colored::*;
use mdz_core::{MdzValidator, ValidationResult};
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct ValidateCommand {
    /// MDZ file to validate
    pub input: PathBuf,
    
    /// Show detailed validation report
    #[arg(long, short)]
    pub detailed: bool,
    
    /// Treat warnings as errors
    #[arg(long)]
    pub strict: bool,
}

impl ValidateCommand {
    pub async fn execute(self) -> Result<()> {
        println!("{} Validating MDZ file...", "●".cyan());
        
        if !self.input.exists() {
            anyhow::bail!("Input file does not exist: {}", self.input.display());
        }

        // Open and validate file
        let file = fs::File::open(&self.input)
            .context("Failed to open input file")?;

        let result = MdzValidator::validate(file)
            .context("Failed to validate MDZ file")?;

        self.print_validation_result(&result);

        // Determine exit status
        let has_errors = !result.errors.is_empty();
        let has_warnings = !result.warnings.is_empty();
        
        if has_errors || (self.strict && has_warnings) {
            anyhow::bail!("Validation failed");
        }

        println!("{} Validation passed", "✓".green());
        Ok(())
    }

    fn print_validation_result(&self, result: &ValidationResult) {
        // Print structure status
        println!("\n{} Structure Check:", "📋".to_string());
        self.print_check("main.md present", result.has_main_md);
        self.print_check("img/ directory", result.has_img_dir);
        self.print_check("css/ directory", result.has_css_dir);
        self.print_check("css/style.css", result.has_main_css);

        // Print file counts
        if self.detailed {
            println!("\n{} File Summary:", "📁".to_string());
            println!("  Image files: {}", result.image_files.len());
            println!("  CSS files: {}", result.css_files.len());

            if !result.image_files.is_empty() {
                println!("\n  {} Images:", "🖼️".to_string());
                for img in &result.image_files {
                    println!("    {}", img);
                }
            }

            if !result.css_files.is_empty() {
                println!("\n  {} CSS Files:", "🎨".to_string());
                for css in &result.css_files {
                    println!("    {}", css);
                }
            }
        }

        // Print errors
        if !result.errors.is_empty() {
            println!("\n{} Errors:", "❌".to_string());
            for error in &result.errors {
                println!("  {} {}", "✗".red(), error);
            }
        }

        // Print warnings
        if !result.warnings.is_empty() {
            println!("\n{} Warnings:", "⚠️".to_string());
            for warning in &result.warnings {
                println!("  {} {}", "!".yellow(), warning);
            }
        }

        // Overall status
        println!("\n{} Overall Status:", "📊".to_string());
        if result.is_valid() {
            println!("  {} Valid MDZ format", "✓".green());
        } else {
            println!("  {} Invalid MDZ format", "✗".red());
        }
    }

    fn print_check(&self, label: &str, status: bool) {
        let (symbol, color) = if status {
            ("✓", "green")
        } else {
            ("✗", "red")
        };
        
        let colored_symbol = match color {
            "green" => symbol.green(),
            "red" => symbol.red(),
            _ => symbol.normal(),
        };
        
        println!("  {} {}", colored_symbol, label);
    }
}
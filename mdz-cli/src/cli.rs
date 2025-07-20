use crate::commands::{CreateCommand, ExtractCommand, ValidateCommand, PreviewCommand};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mdz")]
#[command(about = "A tool for creating and managing MDZ (Markdown Zipped) documents")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create an MDZ file from a source directory
    Create(CreateCommand),
    /// Extract an MDZ file to a directory
    Extract(ExtractCommand),
    /// Validate an MDZ file format
    Validate(ValidateCommand),
    /// Preview an MDZ file in browser
    Preview(PreviewCommand),
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Commands::Create(cmd) => cmd.execute().await,
            Commands::Extract(cmd) => cmd.execute().await,
            Commands::Validate(cmd) => cmd.execute().await,
            Commands::Preview(cmd) => cmd.execute().await,
        }
    }
}
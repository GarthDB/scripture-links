//! Command-line interface handling

use clap::Parser;
use std::fs;
use crate::{parse_scripture_reference, generate_url, generate_url_with_query, process_text_for_scripture_references};

/// Custom error type for CLI operations
pub type CliError = Box<dyn std::error::Error>;

/// Command-line interface definition
#[derive(Parser)]
#[command(name = "scripture-links")]
#[command(about = "Generate links to scriptures on ChurchofJesusChrist.org")]
pub struct Cli {
    /// Scripture reference (e.g., "Isa. 6:5", "2 Ne. 10:14-15")
    #[arg(short, long, group = "input")]
    pub reference: Option<String>,

    /// Process text and convert scripture references to markdown links
    #[arg(short, long, group = "input")]
    pub text: Option<String>,

    /// Process file and convert scripture references to markdown links
    #[arg(short, long, group = "input")]
    pub file: Option<String>,

    /// Optional search query to append to URLs (e.g., "creation", "light")
    #[arg(short, long)]
    pub query: Option<String>,
}

impl Cli {
    /// Execute the CLI command
    pub fn execute(self) -> Result<(), CliError> {
        if let Some(reference) = self.reference {
            // Original single reference mode
            match parse_scripture_reference(&reference) {
                Ok(scripture) => {
                    let url = if let Some(query) = self.query.as_deref() {
                        generate_url_with_query(&scripture, Some(query))
                    } else {
                        generate_url(&scripture)
                    };
                    println!("{}", url);
                }
                Err(error) => {
                    eprintln!("Error: {}", error);
                    std::process::exit(1);
                }
            }
        } else if let Some(text) = self.text {
            // Text processing mode
            let processed_text = process_text_for_scripture_references(&text);
            println!("{}", processed_text);
        } else if let Some(file_path) = self.file {
            // File processing mode
            match fs::read_to_string(&file_path) {
                Ok(file_content) => {
                    let processed_text = process_text_for_scripture_references(&file_content);
                    println!("{}", processed_text);
                }
                Err(error) => {
                    eprintln!("Error reading file '{}': {}", file_path, error);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Error: Please provide either --reference, --text, or --file");
            std::process::exit(1);
        }
        
        Ok(())
    }
}

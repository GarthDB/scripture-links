//! Command-line interface handling

use crate::json_output::{
    BatchResponse, ErrorCategory, ErrorInfo, SingleReferenceResponse, TextProcessingResponse,
    ValidationResponse, create_error_response,
};
use crate::{generate_url, parse_scripture_reference, process_text_for_scripture_references};
use clap::Parser;
use std::fs;

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

    /// Output in JSON format (machine-readable)
    #[arg(long)]
    pub json: bool,

    /// Validate references without generating URLs
    #[arg(long)]
    pub validate_only: bool,

    /// Process multiple references (comma-separated)
    #[arg(long, group = "input")]
    pub batch: Option<String>,
}

impl Cli {
    /// Execute the CLI command
    ///
    /// # Errors
    /// Returns an error if file operations fail or if invalid arguments are provided
    pub fn execute(self) -> Result<(), CliError> {
        if let Some(ref reference) = self.reference {
            self.handle_single_reference(reference)
        } else if let Some(ref batch) = self.batch {
            self.handle_batch_references(batch)
        } else if let Some(ref text) = self.text {
            self.handle_text_processing(text)
        } else if let Some(ref file_path) = self.file {
            self.handle_file_processing(file_path)
        } else {
            Self::output_error("Please provide either --reference, --batch, --text, or --file");
            std::process::exit(1);
        }
    }

    fn handle_single_reference(&self, reference: &str) -> Result<(), CliError> {
        if self.validate_only {
            self.handle_validation(reference)?;
        } else {
            match parse_scripture_reference(reference) {
                Ok(scripture) => {
                    let url = generate_url(&scripture);

                    if self.json {
                        let response = SingleReferenceResponse {
                            success: true,
                            input: reference.to_string(),
                            parsed: Some(scripture),
                            url: Some(url),
                            error: None,
                        };
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        println!("{url}");
                    }
                }
                Err(error) => {
                    if self.json {
                        let response = create_error_response(reference, &error);
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        Self::output_error(&format!("Error: {error}"));
                        std::process::exit(1);
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_batch_references(&self, batch: &str) -> Result<(), CliError> {
        let references: Vec<&str> = batch.split(',').map(str::trim).collect();
        let mut results = Vec::new();
        let mut successful = 0;
        let mut failed = 0;

        for reference in &references {
            match parse_scripture_reference(reference) {
                Ok(scripture) => {
                    let url = if self.validate_only {
                        None
                    } else {
                        Some(generate_url(&scripture))
                    };
                    results.push(SingleReferenceResponse {
                        success: true,
                        input: (*reference).to_string(),
                        parsed: Some(scripture),
                        url,
                        error: None,
                    });
                    successful += 1;
                }
                Err(error) => {
                    results.push(create_error_response(reference, &error));
                    failed += 1;
                }
            }
        }

        if self.json {
            let response = BatchResponse {
                success: failed == 0,
                total_processed: references.len(),
                successful,
                failed,
                results,
            };
            println!("{}", serde_json::to_string_pretty(&response)?);
        } else {
            // Human-readable batch output
            for result in &results {
                if result.success {
                    if let Some(url) = &result.url {
                        println!("{}: {}", result.input, url);
                    } else {
                        println!("{}: Valid", result.input);
                    }
                } else if let Some(error) = &result.error {
                    println!("{}: Error - {}", result.input, error.message);
                }
            }
            println!("\nSummary: {}/{} successful", successful, references.len());
        }

        Ok(())
    }

    fn handle_validation(&self, reference: &str) -> Result<(), CliError> {
        match parse_scripture_reference(reference) {
            Ok(scripture) => {
                if self.json {
                    let response = ValidationResponse {
                        success: true,
                        input: reference.to_string(),
                        valid: true,
                        parsed: Some(scripture),
                        error: None,
                    };
                    println!("{}", serde_json::to_string_pretty(&response)?);
                } else {
                    println!("Valid: {reference}");
                }
            }
            Err(error) => {
                if self.json {
                    let error_info = {
                        let (code, category) = crate::json_output::categorize_error(&error);
                        ErrorInfo::new(&code, &error, category)
                    };
                    let response = ValidationResponse {
                        success: false,
                        input: reference.to_string(),
                        valid: false,
                        parsed: None,
                        error: Some(error_info),
                    };
                    println!("{}", serde_json::to_string_pretty(&response)?);
                } else {
                    println!("Invalid: {reference} - {error}");
                }
            }
        }
        Ok(())
    }

    fn handle_text_processing(&self, text: &str) -> Result<(), CliError> {
        let processed_text = process_text_for_scripture_references(text);

        if self.json {
            // Count references found (rough estimate)
            let references_found = processed_text.matches('[').count();

            let response = TextProcessingResponse {
                success: true,
                input_text: text.to_string(),
                output_text: processed_text,
                references_found,
                references: Vec::new(), // TODO: Could be enhanced to provide detailed reference info
            };
            println!("{}", serde_json::to_string_pretty(&response)?);
        } else {
            println!("{processed_text}");
        }

        Ok(())
    }

    fn handle_file_processing(&self, file_path: &str) -> Result<(), CliError> {
        match fs::read_to_string(file_path) {
            Ok(file_content) => self.handle_text_processing(&file_content),
            Err(error) => {
                if self.json {
                    let _error_info = ErrorInfo::new(
                        "FILE_READ_ERROR",
                        &format!("Error reading file '{file_path}': {error}"),
                        ErrorCategory::FileReadError,
                    );
                    let response = TextProcessingResponse {
                        success: false,
                        input_text: String::new(),
                        output_text: String::new(),
                        references_found: 0,
                        references: Vec::new(),
                    };
                    // Note: This is a bit of a hack - we should have a separate FileError response type
                    println!("{}", serde_json::to_string_pretty(&response)?);
                } else {
                    Self::output_error(&format!("Error reading file '{file_path}': {error}"));
                    std::process::exit(1);
                }
                Ok(())
            }
        }
    }

    #[allow(clippy::branches_sharing_code)]
    fn output_error(message: &str) {
        eprintln!("{message}");
    }
}

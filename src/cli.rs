//! Command-line interface handling

use crate::abbreviations::book_slug_to_display_name;
use crate::json_output::{
    BatchResponse, ErrorCategory, ErrorInfo, SingleReferenceResponse, TextProcessingResponse,
    ValidationResponse, create_error_response,
};
use crate::types::OutputFormat;
use crate::{generate_url, parse_scripture_reference, process_text_with_format};
use clap::Parser;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Custom error type for CLI operations
pub type CliError = Box<dyn std::error::Error>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, clap::ValueEnum)]
pub enum FormatArg {
    Markdown,
    Wikilink,
}

impl From<FormatArg> for OutputFormat {
    fn from(f: FormatArg) -> Self {
        match f {
            FormatArg::Markdown => Self::Markdown,
            FormatArg::Wikilink => Self::Wikilink,
        }
    }
}

/// Command-line interface definition
#[derive(Parser)]
#[command(name = "scripture-links")]
#[command(about = "Generate links to scriptures on ChurchofJesusChrist.org")]
#[command(version)]
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

    /// Write output back to the file (only with --file); only writes if content changed
    #[arg(short, long)]
    pub in_place: bool,

    /// Link format: markdown [text](url) or wikilink [[Book Chapter]]:Verse for Obsidian
    #[arg(long, value_enum, default_value = "markdown")]
    pub format: FormatArg,

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
        if self.in_place && self.file.is_none() {
            Self::output_error("--in-place can only be used with --file");
            std::process::exit(1);
        }
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
                    let output_format: OutputFormat = self.format.into();
                    if self.json {
                        let url = generate_url(&scripture);
                        #[allow(clippy::redundant_clone)]
                        let response = SingleReferenceResponse {
                            success: true,
                            input: reference.to_string(),
                            parsed: Some(scripture.clone()),
                            url: Some(url),
                            error: None,
                        };
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        match output_format {
                            OutputFormat::Wikilink => {
                                let display_name = book_slug_to_display_name(&scripture.book)
                                    .unwrap_or(scripture.book.as_str());
                                let verse_suffix = scripture.verse_end.map_or_else(
                                    || scripture.verse_start.to_string(),
                                    |end| format!("{}-{end}", scripture.verse_start),
                                );
                                println!(
                                    "[[{} {}]]:{verse_suffix}",
                                    display_name, scripture.chapter
                                );
                            }
                            OutputFormat::Markdown => {
                                let url = generate_url(&scripture);
                                println!("{url}");
                            }
                        }
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
        let output_format: OutputFormat = self.format.into();
        let processed_text = process_text_with_format(text, output_format, false);

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
        let file_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
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
                    println!("{}", serde_json::to_string_pretty(&response)?);
                } else {
                    Self::output_error(&format!("Error reading file '{file_path}': {error}"));
                    std::process::exit(1);
                }
                return Ok(());
            }
        };

        let output_format: OutputFormat = self.format.into();
        let processed = process_text_with_format(&file_content, output_format, false);

        if self.in_place {
            if processed != file_content {
                let path = Path::new(file_path);
                let parent = path.parent().unwrap_or_else(|| Path::new("."));
                let mut temp_file = tempfile::Builder::new()
                    .prefix(".scripture-links.")
                    .suffix(".tmp")
                    .tempfile_in(parent)?;
                temp_file.write_all(processed.as_bytes())?;
                temp_file.as_file().sync_all()?;
                temp_file.persist(path)?;
            }
        } else if self.json {
            let references_found = processed.matches('[').count();
            let response = TextProcessingResponse {
                success: true,
                input_text: file_content,
                output_text: processed,
                references_found,
                references: Vec::new(),
            };
            println!("{}", serde_json::to_string_pretty(&response)?);
        } else {
            print!("{processed}");
        }
        Ok(())
    }

    #[allow(clippy::branches_sharing_code)]
    fn output_error(message: &str) {
        eprintln!("{message}");
    }
}

//! # Scripture Links Library
//! 
//! A library for converting scripture references to URLs and processing text containing scripture references.

pub mod abbreviations;
pub mod cli;
pub mod parser;
pub mod scripture_data;
pub mod text_processor;
pub mod types;
pub mod url_generator;

// Re-export the main types and functions for easy use
pub use types::{ScriptureReference, StandardWork};
pub use parser::parse_scripture_reference;
pub use url_generator::generate_url;
pub use text_processor::process_text_for_scripture_references;

#[cfg(test)]
mod integration_tests;

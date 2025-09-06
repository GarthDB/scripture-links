//! # Scripture Links Library
//!
//! A library for converting scripture references to URLs and processing text containing scripture references.

pub mod abbreviations;
pub mod cli;
pub mod json_output;
pub mod parser;
pub mod scripture_data;
pub mod text_processor;
pub mod types;
pub mod url_generator;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

// Re-export the main types and functions for easy use
pub use parser::parse_scripture_reference;
pub use text_processor::process_text_for_scripture_references;
pub use types::{ScriptureReference, StandardWork};
pub use url_generator::generate_url;

#[cfg(test)]
mod integration_tests;

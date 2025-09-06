//! JSON output structures for machine-readable responses

use crate::types::ScriptureReference;
use serde::{Deserialize, Serialize};

/// Success response for single reference processing
#[derive(Debug, Serialize, Deserialize)]
pub struct SingleReferenceResponse {
    pub success: bool,
    pub input: String,
    pub parsed: Option<ScriptureReference>,
    pub url: Option<String>,
    pub error: Option<ErrorInfo>,
}

/// Success response for batch processing
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResponse {
    pub success: bool,
    pub total_processed: usize,
    pub successful: usize,
    pub failed: usize,
    pub results: Vec<SingleReferenceResponse>,
}

/// Response for text processing
#[derive(Debug, Serialize, Deserialize)]
pub struct TextProcessingResponse {
    pub success: bool,
    pub input_text: String,
    pub output_text: String,
    pub references_found: usize,
    pub references: Vec<FoundReference>,
}

/// Information about a found reference in text
#[derive(Debug, Serialize, Deserialize)]
pub struct FoundReference {
    pub original_text: String,
    pub parsed: Option<ScriptureReference>,
    pub url: Option<String>,
    pub position: Option<TextPosition>,
}

/// Position information for found references
#[derive(Debug, Serialize, Deserialize)]
pub struct TextPosition {
    pub start: usize,
    pub end: usize,
}

/// Validation-only response
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResponse {
    pub success: bool,
    pub input: String,
    pub valid: bool,
    pub parsed: Option<ScriptureReference>,
    pub error: Option<ErrorInfo>,
}

/// Structured error information
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
    pub category: ErrorCategory,
    pub suggestions: Option<Vec<String>>,
}

/// Error categories for programmatic handling
#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorCategory {
    InvalidFormat,
    UnknownBook,
    InvalidChapter,
    InvalidVerse,
    FileNotFound,
    FileReadError,
    ParseError,
}

impl ErrorInfo {
    #[must_use]
    pub fn new(code: &str, message: &str, category: ErrorCategory) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            category,
            suggestions: None,
        }
    }

    #[must_use]
    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions = Some(suggestions);
        self
    }
}

/// Helper function to create error responses
#[must_use]
pub fn create_error_response(input: &str, error_msg: &str) -> SingleReferenceResponse {
    let (code, category) = categorize_error(error_msg);
    let suggestions = extract_suggestions(error_msg);

    let error_info = ErrorInfo::new(&code, error_msg, category);
    let error_info = if let Some(suggestions) = suggestions {
        error_info.with_suggestions(suggestions)
    } else {
        error_info
    };

    SingleReferenceResponse {
        success: false,
        input: input.to_string(),
        parsed: None,
        url: None,
        error: Some(error_info),
    }
}

/// Extract suggestions from error messages
fn extract_suggestions(error_msg: &str) -> Option<Vec<String>> {
    if error_msg.contains("Did you mean:") {
        // Extract suggestions from "Did you mean: Philip, Rev.?" format
        error_msg
            .split("Did you mean: ")
            .nth(1)
            .map(|suggestions_part| {
                let suggestions_str = suggestions_part.trim_end_matches('?');
                suggestions_str
                    .split(", ")
                    .map(|s| s.trim().to_string())
                    .collect()
            })
    } else {
        None
    }
}

/// Categorize error messages for structured responses
#[must_use]
pub fn categorize_error(error_msg: &str) -> (String, ErrorCategory) {
    if error_msg.contains("Invalid scripture reference format") {
        ("INVALID_FORMAT".to_string(), ErrorCategory::InvalidFormat)
    } else if error_msg.contains("Unknown book abbreviation") {
        ("UNKNOWN_BOOK".to_string(), ErrorCategory::UnknownBook)
    } else if error_msg.contains("Chapter") && error_msg.contains("does not exist") {
        ("INVALID_CHAPTER".to_string(), ErrorCategory::InvalidChapter)
    } else if error_msg.contains("Verse") && error_msg.contains("does not exist") {
        ("INVALID_VERSE".to_string(), ErrorCategory::InvalidVerse)
    } else {
        ("PARSE_ERROR".to_string(), ErrorCategory::ParseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ScriptureReference, StandardWork};

    #[test]
    fn test_single_reference_response_serialization() {
        let scripture = ScriptureReference {
            book: "gen".to_string(),
            chapter: 1,
            verse_start: 1,
            verse_end: None,
            standard_work: StandardWork::OldTestament,
        };

        let response = SingleReferenceResponse {
            success: true,
            input: "Genesis 1:1".to_string(),
            parsed: Some(scripture),
            url: Some("https://example.com".to_string()),
            error: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"input\":\"Genesis 1:1\""));
    }

    #[test]
    fn test_error_categorization() {
        let (code, category) = categorize_error("Unknown book abbreviation: 'InvalidBook'");
        assert_eq!(code, "UNKNOWN_BOOK");
        assert!(matches!(category, ErrorCategory::UnknownBook));

        let (code, category) = categorize_error("Invalid scripture reference format");
        assert_eq!(code, "INVALID_FORMAT");
        assert!(matches!(category, ErrorCategory::InvalidFormat));
    }
}

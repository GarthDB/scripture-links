//! WASM bindings for the scripture links library

use wasm_bindgen::prelude::*;
use crate::{parse_scripture_reference, generate_url, process_text_for_scripture_references};

// Enable `console.log` for debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Result type for WASM functions
#[wasm_bindgen]
pub struct ScriptureLinkResult {
    success: bool,
    result: String,
    error: Option<String>,
}

#[wasm_bindgen]
impl ScriptureLinkResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn result(&self) -> String {
        self.result.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

/// Parse a single scripture reference and return the URL
#[wasm_bindgen]
pub fn parse_reference(reference: &str) -> ScriptureLinkResult {
    console_log!("Parsing reference: {}", reference);
    
    match parse_scripture_reference(reference) {
        Ok(scripture) => {
            let url = generate_url(&scripture);
            ScriptureLinkResult {
                success: true,
                result: url,
                error: None,
            }
        }
        Err(error) => {
            ScriptureLinkResult {
                success: false,
                result: String::new(),
                error: Some(error),
            }
        }
    }
}

/// Process text and convert scripture references to markdown links
#[wasm_bindgen]
pub fn process_text(text: &str) -> String {
    console_log!("Processing text: {}", text);
    process_text_for_scripture_references(text)
}

/// Get information about supported formats
#[wasm_bindgen]
pub fn get_supported_formats() -> String {
    r#"{
  "supported_works": [
    "Old Testament",
    "New Testament", 
    "Book of Mormon",
    "Doctrine and Covenants",
    "Pearl of Great Price"
  ],
  "formats": [
    "Official abbreviations (e.g., 'Gen.', 'Matt.', '1 Ne.')",
    "Full book names (e.g., 'Genesis', 'Matthew', '1 Nephi')",
    "Compact abbreviations (e.g., 'Gen.', 'Matt.')",
    "Case insensitive",
    "Optional spacing between book and chapter"
  ],
  "examples": [
    "Genesis 1:1",
    "Matt. 5:3-4", 
    "2 Ne. 10:14-15",
    "D&C 128:22-23",
    "Moses 1:39"
  ]
}"#.to_string()
}

/// Initialize the WASM module (called when module loads)
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Scripture Links WASM module initialized!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_parse_reference() {
        let result = parse_reference("Genesis 1:1");
        assert!(result.success());
        assert!(result.result().contains("https://www.churchofjesuschrist.org"));
        assert!(result.error().is_none());
    }

    #[test]
    fn test_wasm_parse_invalid_reference() {
        let result = parse_reference("InvalidBook 1:1");
        assert!(!result.success());
        assert!(result.result().is_empty());
        assert!(result.error().is_some());
    }

    #[test]
    fn test_wasm_process_text() {
        let result = process_text("See Genesis 1:1 and 2 Nephi 10:14");
        assert!(result.contains("[Genesis 1:1]("));
        assert!(result.contains("[2 Nephi 10:14]("));
    }
}

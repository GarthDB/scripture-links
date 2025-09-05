//! Text processing for finding and converting scripture references to markdown links

use regex::Regex;
use crate::abbreviations::create_abbreviation_map;
use crate::parser::parse_scripture_reference;
use crate::url_generator::generate_url;

/// Process text and convert scripture references to markdown links
/// 
/// # Examples
/// 
/// ```
/// use scripture_links::process_text_for_scripture_references;
/// 
/// let input = "See Genesis 1:1 for the creation story";
/// let result = process_text_for_scripture_references(input);
/// assert!(result.contains("[Genesis 1:1]("));
/// ```
pub fn process_text_for_scripture_references(text: &str) -> String {
    // Create a more comprehensive regex to find scripture references in text
    // This should match patterns like:
    // - "See Genesis 1:1 for more details"
    // - "According to 2 Nephi 10:14-15"
    // - "The scripture in D&C 128:22 says"
    // - "As stated in Matt. 5:3-4"
    
    let abbreviations = create_abbreviation_map();
    let mut all_book_patterns = Vec::new();
    
    // Create regex patterns for all known book abbreviations
    for book_abbrev in abbreviations.keys() {
        // Escape special regex characters in book names
        let escaped = regex::escape(book_abbrev);
        all_book_patterns.push(escaped);
    }
    
    // Sort by length (descending) to match longer book names first
    // This prevents "1 Ne." from matching before "1 Nephi"
    all_book_patterns.sort_by_key(|b| std::cmp::Reverse(b.len()));
    
    // Create the master regex pattern
    let book_pattern = all_book_patterns.join("|");
    let pattern = format!(r"\b({})\s*(\d+):(\d+)(?:-(\d+))?\b", book_pattern);
    let re = Regex::new(&pattern).unwrap();
    
    let mut result = text.to_string();
    
    // Find all matches and replace them from end to beginning to preserve indices
    let mut matches: Vec<_> = re.find_iter(text).collect();
    matches.reverse();
    
    for match_obj in matches {
        let matched_text = match_obj.as_str();
        
        // Try to parse this as a scripture reference
        if let Ok(scripture) = parse_scripture_reference(matched_text) {
            let url = generate_url(&scripture);
            let markdown_link = format!("[{}]({})", matched_text, url);
            
            // Replace the matched text with the markdown link
            result.replace_range(match_obj.range(), &markdown_link);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_text_single_reference() {
        let input = "See Genesis 1:1 for the creation story.";
        let result = process_text_for_scripture_references(input);
        assert!(result.contains("[Genesis 1:1](https://www.churchofjesuschrist.org/study/scriptures/ot/gen/1?lang=eng&id=p1#p1)"));
        assert!(result.contains("for the creation story."));
    }

    #[test]
    fn test_process_text_multiple_references() {
        let input = "Read Genesis 1:1 and 2 Nephi 10:14 and D&C 128:22-23 for insights.";
        let result = process_text_for_scripture_references(input);
        
        // Should contain all three references as markdown links
        assert!(result.contains("[Genesis 1:1]("));
        assert!(result.contains("[2 Nephi 10:14]("));
        assert!(result.contains("[D&C 128:22-23]("));
        assert!(result.contains("for insights."));
    }

    #[test]
    fn test_process_text_no_references() {
        let input = "This text has no scripture references in it.";
        let result = process_text_for_scripture_references(input);
        assert_eq!(result, input); // Should be unchanged
    }

    #[test]
    fn test_process_text_mixed_formats() {
        let input = "Compare Genesis 1:1 with Gen. 2:7 and see the difference.";
        let result = process_text_for_scripture_references(input);
        
        // Both should be converted to links
        assert!(result.contains("[Genesis 1:1]("));
        assert!(result.contains("[Gen. 2:7]("));
    }

    #[test]
    fn test_preserves_surrounding_text() {
        let input = "The verse in Isaiah 55:8-9 teaches us about God's ways being higher.";
        let result = process_text_for_scripture_references(input);
        
        assert!(result.starts_with("The verse in "));
        assert!(result.contains("[Isaiah 55:8-9]("));
        assert!(result.ends_with(" teaches us about God's ways being higher."));
    }
}

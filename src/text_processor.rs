//! Text processing for finding and converting scripture references to markdown links

use crate::abbreviations::create_abbreviation_map;
use crate::parser::parse_scripture_reference;
use crate::url_generator::generate_url;
use regex::Regex;

/// Process text and convert scripture references to markdown links
///
/// # Examples
///
/// ```
/// use scripture_links_lib::process_text_for_scripture_references;
///
/// let input = "See Genesis 1:1 for the creation story";
/// let result = process_text_for_scripture_references(input);
/// assert!(result.contains("[Genesis 1:1]("));
/// ```
/// Processes text to find and convert scripture references to markdown links
///
/// # Panics
/// Panics if the regex pattern is invalid (should never happen with hardcoded pattern)
#[must_use]
pub fn process_text_for_scripture_references(text: &str) -> String {
    process_text_with_options(text, false)
}

/// Process text with options for including Study Helps
///
/// # Arguments
/// * `text` - The text to process
/// * `include_study_helps` - Whether to also process Study Helps abbreviations (may cause false positives)
///
/// # Examples
///
/// ```
/// use scripture_links_lib::process_text_with_options;
///
/// let input = "See TG Faith and Genesis 1:1";
/// let result = process_text_with_options(input, true);
/// assert!(result.contains("[Genesis 1:1]("));
/// // Study Helps processing is more restrictive to avoid false positives
/// ```
///
/// # Panics
/// Panics if the internal regex pattern is invalid (should never happen with hardcoded patterns).
#[must_use]
pub fn process_text_with_options(text: &str, include_study_helps: bool) -> String {
    // Create a more comprehensive regex to find scripture references in text
    // This should match patterns like:
    // - "See Genesis 1:1 for more details"
    // - "According to 2 Nephi 10:14-15"
    // - "The scripture in D&C 128:22 says"
    // - "As stated in Matt. 5:3-4"

    let abbreviations = create_abbreviation_map();
    let mut scripture_patterns = Vec::new();
    let mut study_helps_patterns = Vec::new();

    // Separate scripture abbreviations from Study Helps
    for (book_abbrev, (_, standard_work)) in &abbreviations {
        let escaped = regex::escape(book_abbrev);
        if standard_work.is_study_help() {
            study_helps_patterns.push(escaped);
        } else {
            scripture_patterns.push(escaped);
        }
    }

    // Sort by length (descending) to match longer book names first
    // This prevents "1 Ne" from matching before "1 Nephi"
    scripture_patterns.sort_by_key(|b| std::cmp::Reverse(b.len()));
    study_helps_patterns.sort_by_key(|b| std::cmp::Reverse(b.len()));

    let mut result = text.to_string();

    // Process regular scripture references (chapter:verse pattern)
    if !scripture_patterns.is_empty() {
        let book_pattern = scripture_patterns.join("|");
        let pattern = format!(r"\b({book_pattern})\s*\.?\s*(\d+):(\d+)(?:-(\d+))?\b");
        let re = Regex::new(&pattern).unwrap();

        let matches: Vec<_> = re
            .find_iter(&result)
            .map(|m| (m.range(), m.as_str().to_string()))
            .collect();

        // Process matches in reverse order to preserve indices
        for (range, matched_text) in matches.into_iter().rev() {
            // Try to parse this as a scripture reference
            if let Ok(scripture) = parse_scripture_reference(&matched_text) {
                let url = generate_url(&scripture);
                let markdown_link = format!("[{matched_text}]({url})");

                // Replace the matched text with the markdown link
                result.replace_range(range, &markdown_link);
            }
        }
    }

    // Process Study Helps if enabled (more restrictive patterns to avoid false positives)
    if include_study_helps && !study_helps_patterns.is_empty() {
        // Use more restrictive patterns for Study Helps to reduce false positives
        // Look for patterns like "TG Faith", "BD Abraham", "IT Accountability"
        let study_pattern = study_helps_patterns.join("|");
        // Match abbreviation followed by a capitalized word (topic name)
        // Use non-greedy matching and stop at common word boundaries
        let pattern = format!(
            r"\b({study_pattern})\s+([A-Z][A-Za-z0-9\s,.-]*?)(?:\s+(?:and|or|for|in|on|at|to|with|by|the|a|an)\b|\s*[.!?;]|\s*$)"
        );
        let re = Regex::new(&pattern).unwrap();

        let matches: Vec<_> = re
            .captures_iter(&result)
            .map(|caps| {
                let full_match = caps.get(0).unwrap();
                let abbreviation = caps.get(1).unwrap().as_str();
                let topic = caps.get(2).unwrap().as_str().trim();
                (
                    full_match.range(),
                    abbreviation.to_string(),
                    topic.to_string(),
                )
            })
            .collect();

        // Process matches in reverse order to preserve indices
        for (range, abbreviation, topic) in matches.into_iter().rev() {
            // Look up the abbreviation
            if let Some((book_url, standard_work)) = abbreviations.get(abbreviation.as_str())
                && standard_work.is_study_help()
            {
                // Create a ScriptureReference for the Study Help
                let scripture = crate::types::ScriptureReference {
                    book: (*book_url).to_string(),
                    chapter: 1,     // Not used for Study Helps
                    verse_start: 1, // Not used for Study Helps
                    verse_end: None,
                    standard_work: standard_work.clone(),
                    topic: Some(topic.clone()),
                };

                let url = generate_url(&scripture);
                // Only include the abbreviation and topic in the link text, not the boundary words
                let link_text = format!("{abbreviation} {topic}");
                let markdown_link = format!("[{link_text}]({url})");

                // Replace the matched text with the markdown link
                result.replace_range(range, &markdown_link);
            }
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

    #[test]
    fn test_study_helps_disabled_by_default() {
        let input = "See TG Faith and Genesis 1:1 for more information.";
        let result = process_text_for_scripture_references(input);

        // Should process Genesis 1:1 but not TG Faith
        assert!(result.contains("[Genesis 1:1]("));
        assert!(!result.contains("[TG Faith]("));
        assert!(result.contains("See TG Faith and")); // TG Faith should remain unchanged
    }

    #[test]
    fn test_study_helps_enabled() {
        let input = "See TG Faith and BD Abraham for more information.";
        let result = process_text_with_options(input, true);

        // Should process both Study Helps
        assert!(result.contains("[TG Faith]("));
        assert!(result.contains("[BD Abraham]("));
        assert!(result.contains("https://www.churchofjesuschrist.org/study/scriptures/tg/faith"));
        assert!(result.contains("https://www.churchofjesuschrist.org/study/scriptures/bd/abraham"));
    }

    #[test]
    fn test_study_helps_complex_topics() {
        let input = "Check IT Accountability, Age of and GS Aaron, Brother of Moses.";
        let result = process_text_with_options(input, true);

        assert!(result.contains("[IT Accountability, Age of]("));
        assert!(result.contains("[GS Aaron, Brother of Moses]("));
        assert!(result.contains("triple-index/accountability-age-of"));
        assert!(result.contains("gs/aaron-brother-of-moses"));
    }

    #[test]
    fn test_study_helps_avoids_false_positives() {
        let input =
            "The IT department uses TG as an abbreviation. BD stands for business development.";
        let result = process_text_with_options(input, true);

        // These should NOT be converted because they don't follow the expected pattern
        // (no clear topic following the abbreviation)
        assert!(!result.contains("[IT department]("));
        assert!(!result.contains("[TG as]("));
        assert!(!result.contains("[BD stands]("));
        assert_eq!(result, input); // Should be unchanged
    }

    #[test]
    fn test_mixed_scripture_and_study_helps() {
        let input = "Read Genesis 1:1 and see TG Creation for more details.";
        let result = process_text_with_options(input, true);

        // Should process both types
        assert!(result.contains("[Genesis 1:1]("));
        assert!(result.contains("[TG Creation]("));
        assert!(result.contains("ot/gen/1"));
        assert!(result.contains("tg/creation"));
    }
}

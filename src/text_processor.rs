//! Text processing for finding and converting scripture references to markdown links

use crate::abbreviations::{book_slug_to_display_name, create_abbreviation_map};
use crate::parser::parse_scripture_reference;
use crate::types::OutputFormat;
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
    process_text_with_format(text, OutputFormat::Markdown, false)
}

#[must_use]
pub fn process_text_with_format(
    text: &str,
    format: OutputFormat,
    include_study_helps: bool,
) -> String {
    process_text_with_format_impl(text, format, include_study_helps)
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
    process_text_with_format(text, OutputFormat::Markdown, include_study_helps)
}

#[allow(clippy::too_many_lines)]
#[must_use]
fn process_text_with_format_impl(
    text: &str,
    format: OutputFormat,
    include_study_helps: bool,
) -> String {
    const fn range_overlaps(a: &std::ops::Range<usize>, b: &std::ops::Range<usize>) -> bool {
        a.start < b.end && a.end > b.start
    }

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

    // Ranges of existing markdown links [text](url) — do not convert text inside them
    let markdown_link_ranges: Vec<std::ops::Range<usize>> = {
        let link_re = Regex::new(r"\[[^\]]*\]\([^)]*\)").unwrap();
        link_re.find_iter(&result).map(|m| m.range()).collect()
    };

    // Process regular scripture references (chapter:verse pattern)
    if !scripture_patterns.is_empty() {
        struct ScriptureMatch {
            range: std::ops::Range<usize>,
            matched_text: String,
            reference_text: String,
        }

        let book_pattern = scripture_patterns.join("|");
        // Matches "Book Chapter:Verse[-Verse]" or a bare "Book Chapter[-Chapter]"
        // (chapter-only reference, e.g. "Isaiah 29" or "Isaiah 13-14"). The
        // chapter:verse alternative is tried first so "Isaiah 13:5" is never
        // split into bare chapter "13" plus a stray ":5".
        let pattern =
            format!(r"\b({book_pattern})\s*\.?\s*(\d+)(?::(\d+)(?:[-–—](\d+))?|[-–—]\d+)?\b");
        let re = Regex::new(&pattern).unwrap();
        // A single trailing ", N[-M]" comma-list item following a base match.
        let comma_item_re = Regex::new(r"^[ \t]*,[ \t]*(\d+)(?:[-–—]\d+)?").unwrap();
        let continuation_separator_re = Regex::new(r"^[ \t]*([;,])[ \t]*").unwrap();
        let continuation_item_re =
            Regex::new(r"^(\d+)(?::(\d+)(?:[-–—](\d+))?|[-–—]\d+)?\b").unwrap();
        // Detects a new book reference starting at a position, so the comma
        // guard below stops extending when the next item is really the start
        // of a different scripture reference (e.g. "1:1, 2 Nephi 3:4").
        let new_ref_re = Regex::new(&format!(r"^({book_pattern})\b")).unwrap();
        let extend_comma_list = |mut end: usize| {
            while let Some(item) = comma_item_re.captures(&result[end..]) {
                let full = item.get(0).unwrap();
                let number = item.get(1).unwrap();
                let number_pos = end + number.start();
                let after_number = number_pos + number.as_str().len();
                if result
                    .get(after_number..)
                    .is_some_and(|rest| rest.starts_with(':'))
                    || new_ref_re.is_match(&result[number_pos..])
                {
                    break;
                }
                end += full.end();
            }
            end
        };

        let matches: Vec<_> = re
            .captures_iter(&result)
            .flat_map(|captures| {
                let m = captures.get(0).unwrap();
                let book = captures.get(1).unwrap().as_str().to_string();

                // Extend a match over trailing comma-separated verse ranges,
                // but do not consume the chapter of a comma-separated
                // continuation such as ", 24:21-23".
                let end = extend_comma_list(m.end());

                let base_range = m.start()..end;
                let base_text = result[base_range.clone()].to_string();
                let mut found = vec![ScriptureMatch {
                    range: base_range,
                    matched_text: base_text.clone(),
                    reference_text: base_text,
                }];

                // Continue over adjacent semicolon-separated references that
                // omit the book name. A comma is accepted only when the item
                // contains a colon, leaving #85's comma verse-list behavior
                // unchanged for bare verse numbers.
                let mut cursor = end;
                while let Some(separator) = continuation_separator_re.captures(&result[cursor..]) {
                    let separator_match = separator.get(0).unwrap();
                    let separator_kind = separator.get(1).unwrap().as_str();
                    let item_start = cursor + separator_match.end();
                    let Some(item) = continuation_item_re.captures(&result[item_start..]) else {
                        break;
                    };
                    if separator_kind == "," && item.get(2).is_none() {
                        break;
                    }
                    if new_ref_re.is_match(&result[item_start..]) {
                        break;
                    }

                    let item_match = item.get(0).unwrap();
                    let item_range_start = item_start;
                    let item_end = extend_comma_list(item_start + item_match.end());

                    let item_text = result[item_range_start..item_end].to_string();
                    found.push(ScriptureMatch {
                        range: item_range_start..item_end,
                        matched_text: item_text.clone(),
                        reference_text: format!("{book} {item_text}"),
                    });
                    cursor = item_end;
                }

                found
            })
            .collect();

        // Process matches in reverse order to preserve indices
        for ScriptureMatch {
            range,
            matched_text,
            reference_text,
        } in matches.into_iter().rev()
        {
            // Skip if already inside [[wikilink]] (avoid double-converting)
            if range.start >= 2
                && result.get(range.start.saturating_sub(2)..range.start) == Some("[[")
            {
                continue;
            }
            // Skip if inside an existing markdown link [text](url)
            if markdown_link_ranges
                .iter()
                .any(|link_range| range_overlaps(&range, link_range))
            {
                continue;
            }
            // Try to parse this as a scripture reference
            if let Ok(scripture) = parse_scripture_reference(&reference_text) {
                let replacement = match format {
                    OutputFormat::Wikilink => {
                        let display_name = book_slug_to_display_name(&scripture.book)
                            .unwrap_or(scripture.book.as_str());
                        if scripture.verses.is_empty() {
                            format!("[[{display_name} {}]]", scripture.chapter)
                        } else {
                            format!(
                                "[[{display_name} {}]]:{}",
                                scripture.chapter,
                                scripture.verse_display()
                            )
                        }
                    }
                    OutputFormat::Markdown => {
                        let url = generate_url(&scripture);
                        // Normalize DC to D&C in link text when user wrote DC (no ampersand)
                        let trimmed = matched_text.trim();
                        let u = trimmed.to_uppercase();
                        let link_text = if reference_text == matched_text
                            && scripture.book == "dc"
                            && u.len() >= 2
                            && u.get(0..2) == Some("DC")
                            && (u.len() == 2 || u.chars().nth(2) != Some('&'))
                        {
                            let verse_part = if scripture.verses.is_empty() {
                                scripture.chapter.to_string()
                            } else {
                                format!("{}:{}", scripture.chapter, scripture.verse_display())
                            };
                            format!("D&C {verse_part}")
                        } else {
                            matched_text.clone()
                        };
                        format!("[{link_text}]({url})")
                    }
                };

                // Replace the matched text with the link
                result.replace_range(range, &replacement);
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
                    chapter: 1,              // Not used for Study Helps
                    verses: vec![(1, None)], // Not used for Study Helps
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
    fn test_process_text_comma_separated_verse_list() {
        // A trailing comma-list extends the match into a single reference/link.
        let input = "Read Isaiah 13:1-11, 19-22 today.";
        let result = process_text_for_scripture_references(input);
        assert!(result.contains(
            "[Isaiah 13:1-11, 19-22](https://www.churchofjesuschrist.org/study/scriptures/ot/isa/13?lang=eng&id=p1-11,p19-22#p1)"
        ));
        assert!(result.contains("today."));
    }

    #[test]
    fn test_process_text_comma_list_guard_stops_at_new_book() {
        // The comma extension must not swallow a following book reference.
        let input = "Read Genesis 1:1, 2 Nephi 3:4 today.";
        let result = process_text_for_scripture_references(input);
        assert!(result.contains("[Genesis 1:1]("));
        assert!(result.contains("[2 Nephi 3:4]("));
        assert!(!result.contains("1:1, 2 Nephi 3:4]("));
    }

    #[test]
    fn test_process_text_semicolon_list_infers_book_name() {
        let input = "Isaiah 22:22-23; 24:21-23; 25:6-8";
        let result = process_text_for_scripture_references(input);

        assert!(result.contains("[Isaiah 22:22-23](https://"));
        assert!(result.contains("[24:21-23](https://www.churchofjesuschrist.org/study/scriptures/ot/isa/24?lang=eng&id=p21-23#p21)"));
        assert!(result.contains("[25:6-8](https://www.churchofjesuschrist.org/study/scriptures/ot/isa/25?lang=eng&id=p6-8#p6)"));
        assert!(!result.contains("[Isaiah 24:21-23]("));
    }

    #[test]
    fn test_process_text_semicolon_list_stops_at_new_book() {
        let input = "Isaiah 22:22-23; Alma 5:1";
        let result = process_text_for_scripture_references(input);

        assert!(result.contains("[Isaiah 22:22-23]("));
        assert!(result.contains("[Alma 5:1]("));
        assert!(
            !result.contains(
                "[Alma 5:1](https://www.churchofjesuschrist.org/study/scriptures/ot/isa/"
            )
        );
    }

    #[test]
    fn test_process_text_semicolon_list_does_not_cross_sentence_boundary() {
        let input = "Isaiah 22:22-23. See also 24:21-23.";
        let result = process_text_for_scripture_references(input);

        assert!(result.contains("[Isaiah 22:22-23]("));
        assert!(result.contains("See also 24:21-23."));
    }

    #[test]
    fn test_process_text_semicolon_list_supports_chapter_only_and_ranges() {
        let input = "Isaiah 22:22-23; 24; 25–26";
        let result = process_text_for_scripture_references(input);

        assert!(result.contains(
            "[24](https://www.churchofjesuschrist.org/study/scriptures/ot/isa/24?lang=eng)"
        ));
        assert!(result.contains(
            "[25–26](https://www.churchofjesuschrist.org/study/scriptures/ot/isa/25?lang=eng)"
        ));
    }

    #[test]
    fn test_process_text_comma_colon_list_infers_book_name() {
        let input = "Isaiah 22:22-23, 24:21-23";
        let result = process_text_for_scripture_references(input);

        assert!(result.contains("[Isaiah 22:22-23]("));
        assert!(result.contains("[24:21-23](https://www.churchofjesuschrist.org/study/scriptures/ot/isa/24?lang=eng&id=p21-23#p21)"));
    }

    #[test]
    fn test_process_text_semicolon_list_does_not_cross_newline() {
        let input = "Isaiah 22:22-23;\n24:21-23";
        let result = process_text_for_scripture_references(input);

        assert!(result.contains("[Isaiah 22:22-23]("));
        assert!(result.contains("24:21-23"));
        assert!(!result.contains("[24:21-23]("));
    }

    #[test]
    fn test_process_text_semicolon_list_wikilink_expands_book_name() {
        let input = "Isaiah 22:22-23; 24:21-23";
        let result = process_text_with_format(input, OutputFormat::Wikilink, false);

        assert!(result.contains("[[Isaiah 22]]:22-23"));
        assert!(result.contains("[[Isaiah 24]]:21-23"));
    }

    #[test]
    fn test_dc_normalized_to_d_and_c_in_output() {
        // DC (no ampersand) should be recognized and output as D&C
        let input = "Read DC 121:41 today.";
        let result = process_text_for_scripture_references(input);
        assert!(result.contains("[D&C 121:41](https://"));
        assert!(!result.contains("[DC 121:41]("));

        let input_range = "See DC 88:1-2 for context.";
        let result_range = process_text_for_scripture_references(input_range);
        assert!(result_range.contains("[D&C 88:1-2](https://"));

        // Chapter-only DC reference (no verse) also gets normalized
        let input_chapter = "Read DC 88 today.";
        let result_chapter = process_text_for_scripture_references(input_chapter);
        assert!(result_chapter.contains("[D&C 88](https://"));
        assert!(!result_chapter.contains("[DC 88]("));
    }

    #[test]
    fn test_d_and_c_unchanged_in_output() {
        // D&C should remain D&C in link text
        let input = "Read D&C 121:41 today.";
        let result = process_text_for_scripture_references(input);
        assert!(result.contains("[D&C 121:41](https://"));
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
    fn test_preserves_surrounding_text_en_dash() {
        let input = "The verse in Isaiah 14:4–20 teaches us about pride.";
        let result = process_text_for_scripture_references(input);

        assert!(result.starts_with("The verse in "));
        assert!(result.contains("[Isaiah 14:4–20]("));
        assert!(!result.contains(
            "](https://www.churchofjesuschrist.org/study/scriptures/ot/isa/14?lang=eng&id=p4#p4)–20"
        ));
        assert!(result.ends_with(" teaches us about pride."));
    }

    #[test]
    fn test_preserves_surrounding_text_em_dash() {
        let input = "See Isa. 6:5—10 for the vision.";
        let result = process_text_for_scripture_references(input);

        assert!(result.contains("[Isa. 6:5—10]("));
    }

    #[test]
    fn test_chapter_only_reference() {
        let input = "Read Isaiah 29 today.";
        let result = process_text_for_scripture_references(input);

        assert!(result.contains(
            "[Isaiah 29](https://www.churchofjesuschrist.org/study/scriptures/ot/isa/29?lang=eng)"
        ));
        // No verse-anchor id/fragment for a chapter-only reference
        assert!(!result.contains("id=p"));
        assert!(!result.contains('#'));
    }

    #[test]
    fn test_chapter_range_reference() {
        let input = "Read Isaiah 13–14 today.";
        let result = process_text_for_scripture_references(input);

        // Full matched text (including the en dash) is preserved as link text,
        // but the URL links to the first chapter only, with no verse anchor.
        assert!(result.contains(
            "[Isaiah 13–14](https://www.churchofjesuschrist.org/study/scriptures/ot/isa/13?lang=eng)"
        ));
    }

    #[test]
    fn test_chapter_verse_still_works_alongside_chapter_only() {
        // A chapter-only match must not steal from a real chapter:verse reference
        let input = "Isaiah 13:5";
        let result = process_text_for_scripture_references(input);

        assert!(result.contains(
            "[Isaiah 13:5](https://www.churchofjesuschrist.org/study/scriptures/ot/isa/13?lang=eng&id=p5#p5)"
        ));
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

    #[test]
    fn test_study_helps_boundary_words() {
        let input = "See TG Faith and TG Hope for understanding.";
        let result = process_text_with_options(input, true);

        // Should stop at "and" boundary word
        assert!(result.contains("[TG Faith]("));
        assert!(result.contains("[TG Hope]("));
        assert!(!result.contains("[TG Faith and TG Hope]("));
    }

    #[test]
    fn test_study_helps_punctuation_boundaries() {
        let input = "Check TG Faith. Also see BD Abraham; and GS Moses!";
        let result = process_text_with_options(input, true);

        // Should stop at punctuation
        assert!(result.contains("[TG Faith]("));
        assert!(result.contains("[BD Abraham]("));
        assert!(result.contains("[GS Moses]("));
        assert!(result.contains("tg/faith"));
        assert!(result.contains("bd/abraham"));
        assert!(result.contains("gs/moses"));
    }

    #[test]
    fn test_study_helps_end_of_string() {
        let input = "See TG Faith";
        let result = process_text_with_options(input, true);

        assert!(result.contains("[TG Faith]("));
        assert!(result.contains("tg/faith"));
    }

    #[test]
    fn test_study_helps_multiple_same_type() {
        // The regex stops at boundary words like "and", so this needs to be structured differently
        let input = "Read TG Faith. Also TG Hope. And TG Charity.";
        let result = process_text_with_options(input, true);

        assert!(result.contains("[TG Faith]("));
        assert!(result.contains("[TG Hope]("));
        assert!(result.contains("[TG Charity]("));
    }

    #[test]
    fn test_study_helps_case_sensitivity() {
        let input = "See tg faith and TG HOPE.";
        let result = process_text_with_options(input, true);

        // Should match TG but not tg (case sensitive abbreviations)
        assert!(!result.contains("[tg faith]("));
        assert!(result.contains("[TG HOPE]("));
    }

    #[test]
    fn test_study_helps_with_numbers_in_topic() {
        // Topics must start with capital letter per regex pattern
        let input = "Check GS Nephi and BD Abraham.";
        let result = process_text_with_options(input, true);

        assert!(result.contains("[GS Nephi]("));
        assert!(result.contains("[BD Abraham]("));
        assert!(result.contains("gs/nephi"));
        assert!(result.contains("bd/abraham"));
    }

    #[test]
    fn test_study_helps_special_characters_in_topic() {
        let input = "See BD Aaron and GS Jesus Christ.";
        let result = process_text_with_options(input, true);

        assert!(result.contains("[BD Aaron]("));
        assert!(result.contains("[GS Jesus Christ]("));
        // Special characters should be handled in slug conversion
        assert!(result.contains("bd/aaron"));
        assert!(result.contains("gs/jesus-christ"));
    }

    #[test]
    fn test_study_helps_no_false_matches_with_lowercase() {
        let input = "The bd file and tg settings are important.";
        let result = process_text_with_options(input, true);

        // Lowercase abbreviations should not match
        assert_eq!(result, input);
        assert!(!result.contains("[bd file]("));
        assert!(!result.contains("[tg settings]("));
    }

    #[test]
    fn test_wikilink_format_single_reference() {
        let input = "Read Alma 13:6 today.";
        let result = process_text_with_format(input, OutputFormat::Wikilink, false);
        assert!(result.contains("[[Alma 13]]:6"));
        assert!(!result.contains("churchofjesuschrist.org"));
    }

    #[test]
    fn test_wikilink_format_verse_range() {
        let input = "See Moroni 7:45-48.";
        let result = process_text_with_format(input, OutputFormat::Wikilink, false);
        assert!(result.contains("[[Moroni 7]]:45-48"));
    }

    #[test]
    fn test_wikilink_format_chapter_only() {
        // Chapter-only reference (no verse) has no `:suffix` in wikilink output
        let input = "Read Isaiah 29 today.";
        let result = process_text_with_format(input, OutputFormat::Wikilink, false);
        assert!(result.contains("[[Isaiah 29]]"));
        assert!(!result.contains("[[Isaiah 29]]:"));
    }

    #[test]
    fn test_wikilink_format_full_book_names() {
        let input = "Genesis 1:1 and 2 Nephi 10:14.";
        let result = process_text_with_format(input, OutputFormat::Wikilink, false);
        assert!(result.contains("[[Genesis 1]]:1"));
        assert!(result.contains("[[2 Nephi 10]]:14"));
    }

    #[test]
    fn test_wikilink_format_dc() {
        let input = "Read D&C 121:41 today.";
        let result = process_text_with_format(input, OutputFormat::Wikilink, false);
        assert!(result.contains("[[D&C 121]]:41"));
    }

    #[test]
    fn test_wikilink_format_preserves_existing_wikilinks() {
        // Existing [[wikilinks]] don't match the book+chapter:verse pattern
        let input = "See [[Alma 13]]:6 and Alma 13:7.";
        let result = process_text_with_format(input, OutputFormat::Wikilink, false);
        assert!(result.contains("[[Alma 13]]:6")); // preserved
        assert!(result.contains("[[Alma 13]]:7")); // converted
    }

    #[test]
    fn test_existing_markdown_link_not_double_wrapped() {
        // Text already inside [text](url) must not be wrapped again
        let input = "[Matthew 6:27](https://www.churchofjesuschrist.org/study/scriptures/nt/matt/6?lang=eng&id=p27#p27)";
        let result = process_text_for_scripture_references(input);
        assert_eq!(
            result, input,
            "existing markdown link must be left unchanged"
        );
        assert!(
            !result.contains("]]("),
            "must not produce [[...](...)](...) double-wrapping"
        );
    }

    #[test]
    fn test_mixed_linked_and_unlinked_only_converts_unlinked() {
        let input = "See [Matthew 6:27](https://example.com/matt/6#p27) and also Mark 12:41.";
        let result = process_text_for_scripture_references(input);
        // First ref must stay as-is (one link), second must become a link
        assert!(result.contains("[Matthew 6:27](https://example.com/matt/6#p27)"));
        assert!(result.contains("[Mark 12:41](https://www.churchofjesuschrist.org"));
        assert!(
            !result.contains("]]("),
            "must not double-wrap the existing link"
        );
    }

    #[test]
    fn test_process_text_idempotent() {
        // Running the tool on its own output should not change it
        let input = "Read Genesis 1:1 and Alma 13:6.";
        let once = process_text_for_scripture_references(input);
        let twice = process_text_for_scripture_references(&once);
        assert_eq!(once, twice, "second pass must not alter output");
    }

    #[test]
    fn test_full_book_names_nt_epistles() {
        // Full names for NT epistles must be recognized and linked
        let input = "Read 1 Corinthians 13:2 and Galatians 6:15 and Hebrews 11:1.";
        let result = process_text_for_scripture_references(input);
        assert!(result.contains("[1 Corinthians 13:2]("));
        assert!(result.contains("1-cor/13"));
        assert!(result.contains("[Galatians 6:15]("));
        assert!(result.contains("gal/6"));
        assert!(result.contains("[Hebrews 11:1]("));
        assert!(result.contains("heb/11"));
    }

    #[test]
    fn test_one_john_matches_as_first_john_not_gospel() {
        // "1 John 4:18" must link to 1 John (1-jn), not Gospel of John (john)
        let input = "1 John 4:18";
        let result = process_text_for_scripture_references(input);
        assert!(result.contains("1-jn/4"));
        assert!(!result.contains("[John 4:18]("));
    }

    #[test]
    fn test_second_peter_full_name() {
        let input = "See 2 Peter 3:9.";
        let result = process_text_for_scripture_references(input);
        assert!(result.contains("[2 Peter 3:9]("));
        assert!(result.contains("2-pet/3"));
    }

    #[test]
    fn test_mixed_full_names_and_abbreviations() {
        let input = "Compare 1 Cor. 13:1 with 1 Corinthians 13:2 and Heb. 11:1 with Hebrews 11:1.";
        let result = process_text_for_scripture_references(input);
        assert!(result.contains("[1 Cor. 13:1]("));
        assert!(result.contains("[1 Corinthians 13:2]("));
        assert!(result.contains("[Heb. 11:1]("));
        assert!(result.contains("[Hebrews 11:1]("));
    }
}

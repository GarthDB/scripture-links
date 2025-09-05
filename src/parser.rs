//! Scripture reference parsing functionality

use regex::Regex;
use crate::abbreviations::create_abbreviation_map;
use crate::scripture_data;
use crate::types::{ScriptureReference, StandardWork};

/// Parse a scripture reference string into a structured format
/// 
/// # Examples
/// 
/// ```
/// use scripture_links::parse_scripture_reference;
/// 
/// let result = parse_scripture_reference("Genesis 1:1").unwrap();
/// assert_eq!(result.book, "gen");
/// assert_eq!(result.chapter, 1);
/// assert_eq!(result.verse_start, 1);
/// ```
pub fn parse_scripture_reference(reference: &str) -> Result<ScriptureReference, String> {
    let abbreviations = create_abbreviation_map();
    
    // Regex to match scripture references with optional verse ranges
    // Examples: "Isa. 6:5", "Isa.6:5", "2 Ne. 10:14-15", "2Ne.10:14-15", "D&C 128:22-23"
    // This regex captures everything before the chapter:verse pattern as the book
    // The \s* makes the space between book and chapter optional
    let re = Regex::new(r"^(.+?)\s*(\d+):(\d+)(?:-(\d+))?$").unwrap();
    
    if let Some(captures) = re.captures(reference.trim()) {
        let book_abbrev = captures.get(1).unwrap().as_str().trim();
        let chapter: u32 = captures.get(2).unwrap().as_str().parse()
            .map_err(|_| format!("Invalid chapter number in reference: {}", reference))?;
        let verse_start: u32 = captures.get(3).unwrap().as_str().parse()
            .map_err(|_| format!("Invalid verse number in reference: {}", reference))?;
        let verse_end: Option<u32> = captures.get(4)
            .and_then(|m| m.as_str().parse().ok());

        // Case-insensitive lookup
        let lookup_result = abbreviations.iter()
            .find(|(key, _)| key.to_lowercase() == book_abbrev.to_lowercase())
            .map(|(_, value)| value);

        if let Some((book_url, standard_work)) = lookup_result {
            // Validate chapter range
            scripture_data::validate_chapter_range(book_url, chapter)?;
            
            // Validate verse range
            scripture_data::validate_verse_range(book_url, chapter, verse_start, verse_end)?;
            
            Ok(ScriptureReference {
                book: book_url.to_string(),
                chapter,
                verse_start,
                verse_end,
                standard_work: match standard_work {
                    StandardWork::OldTestament => StandardWork::OldTestament,
                    StandardWork::NewTestament => StandardWork::NewTestament,
                    StandardWork::BookOfMormon => StandardWork::BookOfMormon,
                    StandardWork::DoctrineAndCovenants => StandardWork::DoctrineAndCovenants,
                    StandardWork::PearlOfGreatPrice => StandardWork::PearlOfGreatPrice,
                },
            })
        } else {
            // Find similar abbreviations for suggestions (case-insensitive)
            let similar: Vec<&str> = abbreviations.keys()
                .filter(|&key| key.to_lowercase().contains(&book_abbrev.to_lowercase()) || 
                              book_abbrev.to_lowercase().contains(&key.to_lowercase()))
                .take(3)
                .copied()
                .collect();
            
            if similar.is_empty() {
                Err(format!("Unknown book abbreviation: '{}'. Please check the spelling.", book_abbrev))
            } else {
                Err(format!("Unknown book abbreviation: '{}'. Did you mean: {}?", 
                           book_abbrev, similar.join(", ")))
            }
        }
    } else {
        Err(format!("Invalid scripture reference format: '{}'. Expected format: 'Book Chapter:Verse' or 'Book Chapter:Verse-Verse'", reference))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_isaiah() {
        let result = parse_scripture_reference("Isa. 6:5").unwrap();
        assert_eq!(result.book, "isa");
        assert_eq!(result.chapter, 6);
        assert_eq!(result.verse_start, 5);
        assert_eq!(result.verse_end, None);
    }

    #[test]
    fn test_parse_jeremiah() {
        let result = parse_scripture_reference("Jer. 23:5").unwrap();
        assert_eq!(result.book, "jer");
        assert_eq!(result.chapter, 23);
        assert_eq!(result.verse_start, 5);
    }

    #[test]
    fn test_parse_verse_range() {
        let result = parse_scripture_reference("2 Ne. 10:14-15").unwrap();
        assert_eq!(result.book, "2-ne");
        assert_eq!(result.chapter, 10);
        assert_eq!(result.verse_start, 14);
        assert_eq!(result.verse_end, Some(15));
    }

    #[test]
    fn test_parse_dc() {
        let result = parse_scripture_reference("D&C 128:22-23").unwrap();
        assert_eq!(result.book, "dc");
        assert_eq!(result.chapter, 128);
        assert_eq!(result.verse_start, 22);
        assert_eq!(result.verse_end, Some(23));
    }

    #[test]
    fn test_parse_full_names() {
        // Test Book of Mormon full names
        let result = parse_scripture_reference("2 Nephi 10:14").unwrap();
        assert_eq!(result.book, "2-ne");
        assert_eq!(result.chapter, 10);
        assert_eq!(result.verse_start, 14);

        // Test Old Testament full names
        let result = parse_scripture_reference("Genesis 1:1").unwrap();
        assert_eq!(result.book, "gen");
        assert_eq!(result.chapter, 1);
        assert_eq!(result.verse_start, 1);

        // Test New Testament full names
        let result = parse_scripture_reference("Matthew 5:3").unwrap();
        assert_eq!(result.book, "matt");
        assert_eq!(result.chapter, 5);
        assert_eq!(result.verse_start, 3);
    }

    #[test]
    fn test_doctrine_and_covenants_formats() {
        // Test all three D&C formats
        let result1 = parse_scripture_reference("D&C 1:1").unwrap();
        assert_eq!(result1.book, "dc");
        assert_eq!(result1.chapter, 1);
        assert_eq!(result1.verse_start, 1);

        let result2 = parse_scripture_reference("Doctrine and Covenants 1:1").unwrap();
        assert_eq!(result2.book, "dc");
        assert_eq!(result2.chapter, 1);
        assert_eq!(result2.verse_start, 1);

        let result3 = parse_scripture_reference("Doctrine & Covenants 1:1").unwrap();
        assert_eq!(result3.book, "dc");
        assert_eq!(result3.chapter, 1);
        assert_eq!(result3.verse_start, 1);
    }

    #[test]
    fn test_case_insensitive() {
        // Test case-insensitive parsing
        let result1 = parse_scripture_reference("genesis 1:1").unwrap();
        assert_eq!(result1.book, "gen");
        
        let result2 = parse_scripture_reference("GENESIS 1:1").unwrap();
        assert_eq!(result2.book, "gen");
        
        let result3 = parse_scripture_reference("d&c 1:1").unwrap();
        assert_eq!(result3.book, "dc");
        
        let result4 = parse_scripture_reference("2 nephi 10:14").unwrap();
        assert_eq!(result4.book, "2-ne");
        assert_eq!(result4.chapter, 10);
        assert_eq!(result4.verse_start, 14);
    }

    #[test]
    fn test_optional_space() {
        // Test that space between book and chapter is optional
        
        // With space (traditional format)
        let result1 = parse_scripture_reference("Genesis 1:1").unwrap();
        assert_eq!(result1.book, "gen");
        assert_eq!(result1.chapter, 1);
        
        // Without space (compact format)
        let result2 = parse_scripture_reference("Genesis1:1").unwrap();
        assert_eq!(result2.book, "gen");
        assert_eq!(result2.chapter, 1);
        
        // Book of Mormon with space
        let result3 = parse_scripture_reference("2 Ne. 10:14").unwrap();
        assert_eq!(result3.book, "2-ne");
        assert_eq!(result3.chapter, 10);
        
        // Book of Mormon without space (compact abbreviation)
        let result4 = parse_scripture_reference("2Ne.10:14").unwrap();
        assert_eq!(result4.book, "2-ne");
        assert_eq!(result4.chapter, 10);
        
        // D&C without space
        let result5 = parse_scripture_reference("D&C128:22-23").unwrap();
        assert_eq!(result5.book, "dc");
        assert_eq!(result5.chapter, 128);
        assert_eq!(result5.verse_start, 22);
        assert_eq!(result5.verse_end, Some(23));
    }
}

use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

mod scripture_data;

#[derive(Parser)]
#[command(name = "scripture-links")]
#[command(about = "Generate links to scriptures on ChurchofJesusChrist.org")]
struct Cli {
    /// Scripture reference (e.g., "Isa. 6:5", "2 Ne. 10:14-15")
    #[arg(short, long, group = "input")]
    reference: Option<String>,
    
    /// Process text and convert scripture references to markdown links
    #[arg(short, long, group = "input")]
    text: Option<String>,
    
    /// Process file and convert scripture references to markdown links  
    #[arg(short, long, group = "input")]
    file: Option<String>,
}

#[derive(Debug)]
struct ScriptureReference {
    book: String,
    chapter: u32,
    verse_start: u32,
    verse_end: Option<u32>,
    standard_work: StandardWork,
}

#[derive(Debug)]
enum StandardWork {
    OldTestament,
    NewTestament,
    BookOfMormon,
    DoctrineAndCovenants,
    PearlOfGreatPrice,
}

impl StandardWork {
    fn to_url_path(&self) -> &'static str {
        match self {
            StandardWork::OldTestament => "ot",
            StandardWork::NewTestament => "nt",
            StandardWork::BookOfMormon => "bofm",
            StandardWork::DoctrineAndCovenants => "dc-testament",
            StandardWork::PearlOfGreatPrice => "pgp",
        }
    }
}

fn create_abbreviation_map() -> HashMap<&'static str, (&'static str, StandardWork)> {
    let mut map = HashMap::new();
    
    // Old Testament
    map.insert("Gen.", ("gen", StandardWork::OldTestament));
    map.insert("Ex.", ("ex", StandardWork::OldTestament));
    map.insert("Lev.", ("lev", StandardWork::OldTestament));
    map.insert("Num.", ("num", StandardWork::OldTestament));
    map.insert("Deut.", ("deut", StandardWork::OldTestament));
    map.insert("Josh.", ("josh", StandardWork::OldTestament));
    map.insert("Judg.", ("judg", StandardWork::OldTestament));
    map.insert("Ruth", ("ruth", StandardWork::OldTestament));
    map.insert("1 Sam.", ("1-sam", StandardWork::OldTestament));
    map.insert("2 Sam.", ("2-sam", StandardWork::OldTestament));
    map.insert("1 Kgs.", ("1-kgs", StandardWork::OldTestament));
    map.insert("2 Kgs.", ("2-kgs", StandardWork::OldTestament));
    map.insert("1 Chr.", ("1-chr", StandardWork::OldTestament));
    map.insert("2 Chr.", ("2-chr", StandardWork::OldTestament));
    map.insert("Ezra", ("ezra", StandardWork::OldTestament));
    map.insert("Neh.", ("neh", StandardWork::OldTestament));
    map.insert("Esth.", ("esth", StandardWork::OldTestament));
    map.insert("Job", ("job", StandardWork::OldTestament));
    map.insert("Ps.", ("ps", StandardWork::OldTestament));
    map.insert("Prov.", ("prov", StandardWork::OldTestament));
    map.insert("Eccl.", ("eccl", StandardWork::OldTestament));
    map.insert("Song", ("song", StandardWork::OldTestament));
    map.insert("Isa.", ("isa", StandardWork::OldTestament));
    map.insert("Jer.", ("jer", StandardWork::OldTestament));
    map.insert("Lam.", ("lam", StandardWork::OldTestament));
    map.insert("Ezek.", ("ezek", StandardWork::OldTestament));
    map.insert("Dan.", ("dan", StandardWork::OldTestament));
    map.insert("Hosea", ("hosea", StandardWork::OldTestament));
    map.insert("Joel", ("joel", StandardWork::OldTestament));
    map.insert("Amos", ("amos", StandardWork::OldTestament));
    map.insert("Obad.", ("obad", StandardWork::OldTestament));
    map.insert("Jonah", ("jonah", StandardWork::OldTestament));
    map.insert("Micah", ("micah", StandardWork::OldTestament));
    map.insert("Nahum", ("nahum", StandardWork::OldTestament));
    map.insert("Hab.", ("hab", StandardWork::OldTestament));
    map.insert("Zeph.", ("zeph", StandardWork::OldTestament));
    map.insert("Hag.", ("hag", StandardWork::OldTestament));
    map.insert("Zech.", ("zech", StandardWork::OldTestament));
    map.insert("Mal.", ("mal", StandardWork::OldTestament));

    // New Testament
    map.insert("Matt.", ("matt", StandardWork::NewTestament));
    map.insert("Mark", ("mark", StandardWork::NewTestament));
    map.insert("Luke", ("luke", StandardWork::NewTestament));
    map.insert("John", ("john", StandardWork::NewTestament));
    map.insert("Acts", ("acts", StandardWork::NewTestament));
    map.insert("Rom.", ("rom", StandardWork::NewTestament));
    map.insert("1 Cor.", ("1-cor", StandardWork::NewTestament));
    map.insert("2 Cor.", ("2-cor", StandardWork::NewTestament));
    map.insert("Gal.", ("gal", StandardWork::NewTestament));
    map.insert("Eph.", ("eph", StandardWork::NewTestament));
    map.insert("Philip.", ("philip", StandardWork::NewTestament));
    map.insert("Col.", ("col", StandardWork::NewTestament));
    map.insert("1 Thes.", ("1-thes", StandardWork::NewTestament));
    map.insert("2 Thes.", ("2-thes", StandardWork::NewTestament));
    map.insert("1 Tim.", ("1-tim", StandardWork::NewTestament));
    map.insert("2 Tim.", ("2-tim", StandardWork::NewTestament));
    map.insert("Titus", ("titus", StandardWork::NewTestament));
    map.insert("Philem.", ("philem", StandardWork::NewTestament));
    map.insert("Heb.", ("heb", StandardWork::NewTestament));
    map.insert("James", ("james", StandardWork::NewTestament));
    map.insert("1 Pet.", ("1-pet", StandardWork::NewTestament));
    map.insert("2 Pet.", ("2-pet", StandardWork::NewTestament));
    map.insert("1 Jn.", ("1-jn", StandardWork::NewTestament));
    map.insert("2 Jn.", ("2-jn", StandardWork::NewTestament));
    map.insert("3 Jn.", ("3-jn", StandardWork::NewTestament));
    map.insert("Jude", ("jude", StandardWork::NewTestament));
    map.insert("Rev.", ("rev", StandardWork::NewTestament));

    // Book of Mormon - Official abbreviations
    map.insert("1 Ne.", ("1-ne", StandardWork::BookOfMormon));
    map.insert("2 Ne.", ("2-ne", StandardWork::BookOfMormon));
    map.insert("Jacob", ("jacob", StandardWork::BookOfMormon));
    map.insert("Enos", ("enos", StandardWork::BookOfMormon));
    map.insert("Jarom", ("jarom", StandardWork::BookOfMormon));
    map.insert("Omni", ("omni", StandardWork::BookOfMormon));
    map.insert("W of M", ("w-of-m", StandardWork::BookOfMormon));
    map.insert("Mosiah", ("mosiah", StandardWork::BookOfMormon));
    map.insert("Alma", ("alma", StandardWork::BookOfMormon));
    map.insert("Hel.", ("hel", StandardWork::BookOfMormon));
    map.insert("3 Ne.", ("3-ne", StandardWork::BookOfMormon));
    map.insert("4 Ne.", ("4-ne", StandardWork::BookOfMormon));
    map.insert("Morm.", ("morm", StandardWork::BookOfMormon));
    map.insert("Ether", ("ether", StandardWork::BookOfMormon));
    map.insert("Moro.", ("moro", StandardWork::BookOfMormon));

    // Book of Mormon - Full names (alternative spellings)
    map.insert("1 Nephi", ("1-ne", StandardWork::BookOfMormon));
    map.insert("2 Nephi", ("2-ne", StandardWork::BookOfMormon));
    map.insert("Words of Mormon", ("w-of-m", StandardWork::BookOfMormon));
    map.insert("Helaman", ("hel", StandardWork::BookOfMormon));
    map.insert("3 Nephi", ("3-ne", StandardWork::BookOfMormon));
    map.insert("4 Nephi", ("4-ne", StandardWork::BookOfMormon));
    map.insert("Mormon", ("morm", StandardWork::BookOfMormon));
    map.insert("Moroni", ("moro", StandardWork::BookOfMormon));

    // Book of Mormon - Compact abbreviations (no spaces)
    map.insert("1Ne.", ("1-ne", StandardWork::BookOfMormon));
    map.insert("2Ne.", ("2-ne", StandardWork::BookOfMormon));
    map.insert("3Ne.", ("3-ne", StandardWork::BookOfMormon));
    map.insert("4Ne.", ("4-ne", StandardWork::BookOfMormon));
    map.insert("Hel.", ("hel", StandardWork::BookOfMormon));
    map.insert("Morm.", ("morm", StandardWork::BookOfMormon));
    map.insert("Moro.", ("moro", StandardWork::BookOfMormon));

    // Common full name alternatives for frequently used books
    // Old Testament
    map.insert("Genesis", ("gen", StandardWork::OldTestament));
    map.insert("Exodus", ("ex", StandardWork::OldTestament));
    map.insert("Isaiah", ("isa", StandardWork::OldTestament));
    map.insert("Jeremiah", ("jer", StandardWork::OldTestament));
    map.insert("Psalms", ("ps", StandardWork::OldTestament));
    
    // New Testament  
    map.insert("Matthew", ("matt", StandardWork::NewTestament));
    map.insert("Mark", ("mark", StandardWork::NewTestament));
    map.insert("Luke", ("luke", StandardWork::NewTestament));
    map.insert("John", ("john", StandardWork::NewTestament));
    map.insert("Acts", ("acts", StandardWork::NewTestament));
    map.insert("Romans", ("rom", StandardWork::NewTestament));
    map.insert("Revelation", ("rev", StandardWork::NewTestament));

    // Compact abbreviations (no spaces after periods)
    // Old Testament
    map.insert("Gen.", ("gen", StandardWork::OldTestament));
    map.insert("Ex.", ("ex", StandardWork::OldTestament)); 
    map.insert("Isa.", ("isa", StandardWork::OldTestament));
    map.insert("Jer.", ("jer", StandardWork::OldTestament));
    
    // New Testament
    map.insert("Matt.", ("matt", StandardWork::NewTestament));
    map.insert("Rev.", ("rev", StandardWork::NewTestament));

    // Doctrine and Covenants
    map.insert("D&C", ("dc", StandardWork::DoctrineAndCovenants));
    map.insert("OD", ("od", StandardWork::DoctrineAndCovenants));
    map.insert("Doctrine and Covenants", ("dc", StandardWork::DoctrineAndCovenants));
    map.insert("Doctrine & Covenants", ("dc", StandardWork::DoctrineAndCovenants));

    // Pearl of Great Price
    map.insert("Moses", ("moses", StandardWork::PearlOfGreatPrice));
    map.insert("Abr.", ("abr", StandardWork::PearlOfGreatPrice));
    map.insert("JS—M", ("js-m", StandardWork::PearlOfGreatPrice));
    map.insert("JS—H", ("js-h", StandardWork::PearlOfGreatPrice));
    map.insert("A of F", ("a-of-f", StandardWork::PearlOfGreatPrice));

    map
}

fn parse_scripture_reference(reference: &str) -> Result<ScriptureReference, String> {
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
            .map(|m| m.as_str().parse().ok())
            .flatten();

        // Case-insensitive lookup
        let lookup_result = abbreviations.iter()
            .find(|(key, _)| key.to_lowercase() == book_abbrev.to_lowercase())
            .map(|(_, value)| value);

        if let Some((book_url, standard_work)) = lookup_result {
            // Validate chapter range
            if let Err(error) = scripture_data::validate_chapter_range(book_url, chapter) {
                return Err(error);
            }
            
            // Validate verse range
            if let Err(error) = scripture_data::validate_verse_range(book_url, chapter, verse_start, verse_end) {
                return Err(error);
            }
            
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
            // Find similar abbreviations for suggestions
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

fn generate_url(scripture: &ScriptureReference) -> String {
    let base_url = "https://www.churchofjesuschrist.org/study/scriptures";
    let standard_work_path = scripture.standard_work.to_url_path();
    
    // Special handling for Doctrine and Covenants
    let book_path = if matches!(scripture.standard_work, StandardWork::DoctrineAndCovenants) {
        "dc".to_string()
    } else {
        scripture.book.clone()
    };
    
    let id_param = if let Some(verse_end) = scripture.verse_end {
        format!("p{}-{}", scripture.verse_start, verse_end)
    } else {
        format!("p{}", scripture.verse_start)
    };
    
    let fragment = format!("p{}", scripture.verse_start);
    
    format!("{}/{}/{}/{}?lang=eng&id={}#{}", 
            base_url, standard_work_path, book_path, scripture.chapter, id_param, fragment)
}

fn process_text_for_scripture_references(text: &str) -> String {
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
    all_book_patterns.sort_by(|a, b| b.len().cmp(&a.len()));
    
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

fn main() {
    let cli = Cli::parse();
    
    if let Some(reference) = cli.reference {
        // Original single reference mode
        match parse_scripture_reference(&reference) {
            Ok(scripture) => {
                let url = generate_url(&scripture);
                println!("{}", url);
            }
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        }
    } else if let Some(text) = cli.text {
        // Text processing mode
        let processed_text = process_text_for_scripture_references(&text);
        println!("{}", processed_text);
    } else if let Some(file_path) = cli.file {
        // File processing mode
        match fs::read_to_string(&file_path) {
            Ok(file_content) => {
                let processed_text = process_text_for_scripture_references(&file_content);
                println!("{}", processed_text);
            }
            Err(error) => {
                eprintln!("Error reading file '{}': {}", file_path, error);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Error: Please provide either --reference, --text, or --file");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

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
    fn test_generate_url_single_verse() {
        let scripture = ScriptureReference {
            book: "isa".to_string(),
            chapter: 6,
            verse_start: 5,
            verse_end: None,
            standard_work: StandardWork::OldTestament,
        };
        let url = generate_url(&scripture);
        assert_eq!(url, "https://www.churchofjesuschrist.org/study/scriptures/ot/isa/6?lang=eng&id=p5#p5");
    }

    #[test]
    fn test_generate_url_verse_range() {
        let scripture = ScriptureReference {
            book: "2-ne".to_string(),
            chapter: 10,
            verse_start: 14,
            verse_end: Some(15),
            standard_work: StandardWork::BookOfMormon,
        };
        let url = generate_url(&scripture);
        assert_eq!(url, "https://www.churchofjesuschrist.org/study/scriptures/bofm/2-ne/10?lang=eng&id=p14-15#p14");
    }

    // Property-based tests
    proptest! {
        #[test]
        fn test_url_generation_always_contains_required_parts(
            chapter in 1u32..150,
            verse_start in 1u32..200,
            verse_end in proptest::option::of(1u32..200)
        ) {
            let scripture = ScriptureReference {
                book: "gen".to_string(),
                chapter,
                verse_start,
                verse_end,
                standard_work: StandardWork::OldTestament,
            };
            let url = generate_url(&scripture);
            
            // All generated URLs should contain these required components
            prop_assert!(url.contains("https://www.churchofjesuschrist.org/study/scriptures"));
            prop_assert!(url.contains("lang=eng"));
            let expected_path = format!("gen/{}", chapter);
            prop_assert!(url.contains(&expected_path));
            let expected_id = format!("id=p{}", verse_start);
            prop_assert!(url.contains(&expected_id));
            let expected_fragment = format!("#p{}", verse_start);
            prop_assert!(url.contains(&expected_fragment));
        }

        #[test]
        fn test_text_processing_preserves_non_scripture_text(
            prefix in r"[ ]{0,10}",
            suffix in r"[ ]{0,10}"
        ) {
            let text = format!("{}Genesis 1:1{}", prefix, suffix);
            let result = process_text_for_scripture_references(&text);
            
            // Should preserve prefix and suffix text (spaces)
            if !prefix.is_empty() {
                prop_assert!(result.starts_with(&prefix) || result.contains(&prefix));
            }
            if !suffix.is_empty() {
                prop_assert!(result.ends_with(&suffix) || result.contains(&suffix));
            }
            
            // Should contain markdown link (only when word boundaries allow it)
            prop_assert!(result.contains("[Genesis 1:1]("));
        }
    }
}
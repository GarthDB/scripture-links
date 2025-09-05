#!/usr/bin/env python3
"""
Generate Rust data structure from LDS Scriptures JSON

This script processes the beandog/lds-scriptures JSON data to create a complete
Rust data structure for scripture validation in our CLI tool.
"""

import json
from collections import defaultdict
from typing import Dict, List

def map_volume_to_standard_work(volume_title: str) -> str:
    """Map volume titles to our standard work identifiers"""
    mapping = {
        "Old Testament": "ot",
        "New Testament": "nt",
        "Book of Mormon": "bofm",
        "Doctrine and Covenants": "dc-testament",
        "Pearl of Great Price": "pgp"
    }
    return mapping.get(volume_title, "unknown")

def book_title_to_url_name(book_title: str, volume_title: str) -> str:
    """Convert book titles to URL-safe names"""
    # Map known book titles to their URL equivalents
    mapping = {
        # Old Testament
        "Genesis": "gen",
        "Exodus": "ex",
        "Leviticus": "lev",
        "Numbers": "num",
        "Deuteronomy": "deut",
        "Joshua": "josh",
        "Judges": "judg",
        "Ruth": "ruth",
        "1 Samuel": "1-sam",
        "2 Samuel": "2-sam",
        "1 Kings": "1-kgs",
        "2 Kings": "2-kgs",
        "1 Chronicles": "1-chr",
        "2 Chronicles": "2-chr",
        "Ezra": "ezra",
        "Nehemiah": "neh",
        "Esther": "esth",
        "Job": "job",
        "Psalms": "ps",
        "Proverbs": "prov",
        "Ecclesiastes": "eccl",
        "Song of Solomon": "song",
        "Isaiah": "isa",
        "Jeremiah": "jer",
        "Lamentations": "lam",
        "Ezekiel": "ezek",
        "Daniel": "dan",
        "Hosea": "hosea",
        "Joel": "joel",
        "Amos": "amos",
        "Obadiah": "obad",
        "Jonah": "jonah",
        "Micah": "micah",
        "Nahum": "nahum",
        "Habakkuk": "hab",
        "Zephaniah": "zeph",
        "Haggai": "hag",
        "Zechariah": "zech",
        "Malachi": "mal",
        
        # New Testament
        "Matthew": "matt",
        "Mark": "mark",
        "Luke": "luke",
        "John": "john",
        "Acts": "acts",
        "Romans": "rom",
        "1 Corinthians": "1-cor",
        "2 Corinthians": "2-cor",
        "Galatians": "gal",
        "Ephesians": "eph",
        "Philippians": "philip",
        "Colossians": "col",
        "1 Thessalonians": "1-thes",
        "2 Thessalonians": "2-thes",
        "1 Timothy": "1-tim",
        "2 Timothy": "2-tim",
        "Titus": "titus",
        "Philemon": "philem",
        "Hebrews": "heb",
        "James": "james",
        "1 Peter": "1-pet",
        "2 Peter": "2-pet",
        "1 John": "1-jn",
        "2 John": "2-jn",
        "3 John": "3-jn",
        "Jude": "jude",
        "Revelation": "rev",
        
        # Book of Mormon
        "1 Nephi": "1-ne",
        "2 Nephi": "2-ne",
        "Jacob": "jacob",
        "Enos": "enos",
        "Jarom": "jarom",
        "Omni": "omni",
        "Words of Mormon": "w-of-m",
        "Mosiah": "mosiah",
        "Alma": "alma",
        "Helaman": "hel",
        "3 Nephi": "3-ne",
        "4 Nephi": "4-ne",
        "Mormon": "morm",
        "Ether": "ether",
        "Moroni": "moro",
        
        # Doctrine and Covenants
        "Doctrine and Covenants": "dc",
        "Official Declaration 1": "od-1",
        "Official Declaration 2": "od-2",
        
        # Pearl of Great Price
        "Moses": "moses",
        "Abraham": "abr",
        "Joseph Smith—Matthew": "js-m",
        "Joseph Smith—History": "js-h",
        "Articles of Faith": "a-of-f"
    }
    
    return mapping.get(book_title, book_title.lower().replace(" ", "-"))

def process_json_data(json_file_path: str) -> Dict:
    """Process the JSON data and organize by book and chapter"""
    scripture_structure = defaultdict(lambda: defaultdict(int))
    book_info = {}
    
    print(f"Processing {json_file_path}...")
    
    with open(json_file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    print(f"Loaded {len(data)} verses")
    
    for verse in data:
        volume_title = verse['volume_title']
        book_title = verse['book_title']
        chapter_number = verse['chapter_number']
        verse_number = verse['verse_number']
        
        standard_work = map_volume_to_standard_work(volume_title)
        url_name = book_title_to_url_name(book_title, volume_title)
        
        # Track the maximum verse number for each chapter
        current_max = scripture_structure[url_name][chapter_number]
        if verse_number > current_max:
            scripture_structure[url_name][chapter_number] = verse_number
        
        # Store book info
        if url_name not in book_info:
            book_info[url_name] = {
                'name': book_title,
                'url_name': url_name,
                'standard_work': standard_work,
                'volume_title': volume_title
            }
    
    print(f"Processed {len(book_info)} books")
    return scripture_structure, book_info

def generate_rust_code(scripture_structure: Dict, book_info: Dict) -> str:
    """Generate the Rust code for the scripture data module"""
    
    rust_code = '''use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct BookInfo {
    pub name: &'static str,
    pub url_name: &'static str,
    pub standard_work: &'static str,
    pub chapters: Vec<u32>, // verse counts per chapter
}

// Complete scripture data generated from beandog/lds-scriptures repository
static SCRIPTURE_DATA: Lazy<HashMap<&'static str, BookInfo>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
'''
    
    # Sort books by standard work for organization
    books_by_standard_work = defaultdict(list)
    for url_name, info in book_info.items():
        books_by_standard_work[info['standard_work']].append((url_name, info))
    
    # Generate entries for each book
    for standard_work in ['ot', 'nt', 'bofm', 'dc-testament', 'pgp']:
        if standard_work in books_by_standard_work:
            rust_code += f'    // {standard_work.upper()}\n'
            
            for url_name, info in sorted(books_by_standard_work[standard_work]):
                if url_name in scripture_structure:
                    chapters = scripture_structure[url_name]
                    # Create list of verse counts per chapter
                    max_chapter = max(chapters.keys()) if chapters else 0
                    verse_counts = []
                    
                    for chapter_num in range(1, max_chapter + 1):
                        verse_count = chapters.get(chapter_num, 0)
                        verse_counts.append(str(verse_count))
                    
                    # Format the verse counts for Rust
                    if len(verse_counts) <= 10:
                        chapters_str = f"vec![{', '.join(verse_counts)}]"
                    else:
                        # Break long lists into multiple lines
                        lines = []
                        for i in range(0, len(verse_counts), 10):
                            chunk = verse_counts[i:i+10]
                            lines.append(', '.join(chunk))
                        
                        chapters_str = "vec![\n"
                        for i, line in enumerate(lines):
                            comment = f" // Chapters {i*10 + 1}-{min((i+1)*10, len(verse_counts))}"
                            chapters_str += f"        {line}{',' if i < len(lines) - 1 else ''}{comment}\n"
                        chapters_str += "    ]"
                    
                    rust_code += f'''    map.insert("{url_name}", BookInfo {{
        name: "{info['name']}",
        url_name: "{url_name}",
        standard_work: "{info['standard_work']}",
        chapters: {chapters_str},
    }});
    
'''
    
    rust_code += '''    map
});

pub fn get_book_info(book_key: &str) -> Option<&BookInfo> {
    SCRIPTURE_DATA.get(book_key)
}

pub fn validate_chapter_range(book_key: &str, chapter: u32) -> Result<(), String> {
    match get_book_info(book_key) {
        Some(book_info) => {
            let total_chapters = book_info.chapters.len() as u32;
            if chapter == 0 {
                Err(format!("Chapter number must be greater than 0"))
            } else if chapter > total_chapters {
                Err(format!(
                    "Chapter {} does not exist in {}. {} has {} chapters (1-{})",
                    chapter, book_info.name, book_info.name, total_chapters, total_chapters
                ))
            } else {
                Ok(())
            }
        }
        None => {
            // If we don't have data for this book, we can't validate
            // This allows the system to work with books we haven't included yet
            Ok(())
        }
    }
}

pub fn validate_verse_range(book_key: &str, chapter: u32, verse_start: u32, verse_end: Option<u32>) -> Result<(), String> {
    match get_book_info(book_key) {
        Some(book_info) => {
            // First validate the chapter exists
            validate_chapter_range(book_key, chapter)?;
            
            let chapter_index = (chapter - 1) as usize;
            let total_verses = book_info.chapters[chapter_index];
            
            // Validate start verse
            if verse_start == 0 {
                return Err(format!("Verse number must be greater than 0"));
            }
            if verse_start > total_verses {
                return Err(format!(
                    "Verse {} does not exist in {} {}. Chapter {} has {} verses (1-{})",
                    verse_start, book_info.name, chapter, chapter, total_verses, total_verses
                ));
            }
            
            // Validate end verse if provided
            if let Some(end_verse) = verse_end {
                if end_verse < verse_start {
                    return Err(format!(
                        "End verse ({}) cannot be less than start verse ({})",
                        end_verse, verse_start
                    ));
                }
                if end_verse > total_verses {
                    return Err(format!(
                        "Verse {} does not exist in {} {}. Chapter {} has {} verses (1-{})",
                        end_verse, book_info.name, chapter, chapter, total_verses, total_verses
                    ));
                }
            }
            
            Ok(())
        }
        None => {
            // If we don't have data for this book, we can't validate
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_data() {
        let genesis = get_book_info("gen").unwrap();
        assert_eq!(genesis.name, "Genesis");
        assert_eq!(genesis.chapters.len(), 50);
        assert_eq!(genesis.chapters[0], 31); // Genesis 1 has 31 verses
    }

    #[test]
    fn test_valid_chapter_range() {
        assert!(validate_chapter_range("gen", 1).is_ok());
        assert!(validate_chapter_range("gen", 50).is_ok());
        assert!(validate_chapter_range("ruth", 4).is_ok());
    }

    #[test]
    fn test_invalid_chapter_range() {
        assert!(validate_chapter_range("gen", 0).is_err());
        assert!(validate_chapter_range("gen", 51).is_err());
        assert!(validate_chapter_range("ruth", 5).is_err());
    }

    #[test]
    fn test_valid_verse_range() {
        assert!(validate_verse_range("gen", 1, 1, None).is_ok());
        assert!(validate_verse_range("gen", 1, 31, None).is_ok());
        assert!(validate_verse_range("gen", 1, 1, Some(5)).is_ok());
        assert!(validate_verse_range("gen", 1, 25, Some(31)).is_ok());
    }

    #[test]
    fn test_invalid_verse_range() {
        assert!(validate_verse_range("gen", 1, 0, None).is_err());
        assert!(validate_verse_range("gen", 1, 32, None).is_err());
        assert!(validate_verse_range("gen", 1, 5, Some(4)).is_err()); // End before start
        assert!(validate_verse_range("gen", 1, 25, Some(32)).is_err()); // End verse too high
    }

    #[test]
    fn test_unknown_book() {
        // Unknown books should not cause errors (graceful fallback)
        assert!(validate_chapter_range("unknown", 100).is_ok());
        assert!(validate_verse_range("unknown", 100, 100, Some(200)).is_ok());
    }
}'''
    
    return rust_code

def main():
    json_file = '/Users/garthdb/Projects/scripture-links/scripture-data/json/lds-scriptures-json.txt'
    
    print("Generating complete Rust scripture data from LDS scriptures JSON...")
    
    scripture_structure, book_info = process_json_data(json_file)
    
    print("\nStatistics:")
    total_chapters = 0
    total_verses = 0
    
    for url_name, chapters in scripture_structure.items():
        book_chapters = len(chapters)
        book_verses = sum(chapters.values())
        total_chapters += book_chapters
        total_verses += book_verses
        print(f"  {book_info[url_name]['name']}: {book_chapters} chapters, {book_verses} verses")
    
    print(f"\nTotal: {len(book_info)} books, {total_chapters} chapters, {total_verses} verses")
    
    # Generate the Rust code
    rust_code = generate_rust_code(scripture_structure, book_info)
    
    # Write to new file
    output_file = '/Users/garthdb/Projects/scripture-links/src/scripture_data_complete.rs'
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write(rust_code)
    
    print(f"\nGenerated complete Rust data structure: {output_file}")
    print("\nTo use this data, replace the contents of src/scripture_data.rs with src/scripture_data_complete.rs")

if __name__ == "__main__":
    main()

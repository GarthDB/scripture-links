//! Scripture book abbreviations and mappings

use crate::types::StandardWork;
use std::collections::HashMap;

/// Type alias for abbreviation mappings: abbreviation -> (url_name, standard_work)
pub type AbbreviationMap = HashMap<&'static str, (&'static str, StandardWork)>;

/// Create a comprehensive mapping of scripture abbreviations to their URL names and standard works
pub fn create_abbreviation_map() -> AbbreviationMap {
    let mut map = HashMap::new();

    // Old Testament - periods are handled by parser normalization
    map.insert("Gen", ("gen", StandardWork::OldTestament));
    map.insert("Ex", ("ex", StandardWork::OldTestament));
    map.insert("Lev", ("lev", StandardWork::OldTestament));
    map.insert("Num", ("num", StandardWork::OldTestament));
    map.insert("Deut", ("deut", StandardWork::OldTestament));
    map.insert("Josh", ("josh", StandardWork::OldTestament));
    map.insert("Judg", ("judg", StandardWork::OldTestament));
    map.insert("Ruth", ("ruth", StandardWork::OldTestament));
    map.insert("1 Sam", ("1-sam", StandardWork::OldTestament));
    map.insert("2 Sam", ("2-sam", StandardWork::OldTestament));
    map.insert("1 Kgs", ("1-kgs", StandardWork::OldTestament));
    map.insert("2 Kgs", ("2-kgs", StandardWork::OldTestament));
    map.insert("1 Chr", ("1-chr", StandardWork::OldTestament));
    map.insert("2 Chr", ("2-chr", StandardWork::OldTestament));
    map.insert("Ezra", ("ezra", StandardWork::OldTestament));
    map.insert("Neh", ("neh", StandardWork::OldTestament));
    map.insert("Esth", ("esth", StandardWork::OldTestament));
    map.insert("Job", ("job", StandardWork::OldTestament));
    map.insert("Ps", ("ps", StandardWork::OldTestament));
    map.insert("Prov", ("prov", StandardWork::OldTestament));
    map.insert("Eccl", ("eccl", StandardWork::OldTestament));
    map.insert("Song", ("song", StandardWork::OldTestament));
    map.insert("Isa", ("isa", StandardWork::OldTestament));
    map.insert("Jer", ("jer", StandardWork::OldTestament));
    map.insert("Lam", ("lam", StandardWork::OldTestament));
    map.insert("Ezek", ("ezek", StandardWork::OldTestament));
    map.insert("Dan", ("dan", StandardWork::OldTestament));
    map.insert("Hosea", ("hosea", StandardWork::OldTestament));
    map.insert("Joel", ("joel", StandardWork::OldTestament));
    map.insert("Amos", ("amos", StandardWork::OldTestament));
    map.insert("Obad", ("obad", StandardWork::OldTestament));
    map.insert("Jonah", ("jonah", StandardWork::OldTestament));
    map.insert("Micah", ("micah", StandardWork::OldTestament));
    map.insert("Nahum", ("nahum", StandardWork::OldTestament));
    map.insert("Hab", ("hab", StandardWork::OldTestament));
    map.insert("Zeph", ("zeph", StandardWork::OldTestament));
    map.insert("Hag", ("hag", StandardWork::OldTestament));
    map.insert("Zech", ("zech", StandardWork::OldTestament));
    map.insert("Mal", ("mal", StandardWork::OldTestament));

    // New Testament - periods are handled by parser normalization
    map.insert("Matt", ("matt", StandardWork::NewTestament));
    map.insert("Mark", ("mark", StandardWork::NewTestament));
    map.insert("Luke", ("luke", StandardWork::NewTestament));
    map.insert("John", ("john", StandardWork::NewTestament));
    map.insert("Acts", ("acts", StandardWork::NewTestament));
    map.insert("Rom", ("rom", StandardWork::NewTestament));
    map.insert("1 Cor", ("1-cor", StandardWork::NewTestament));
    map.insert("2 Cor", ("2-cor", StandardWork::NewTestament));
    map.insert("Gal", ("gal", StandardWork::NewTestament));
    map.insert("Eph", ("eph", StandardWork::NewTestament));
    map.insert("Philip", ("philip", StandardWork::NewTestament));
    map.insert("Col", ("col", StandardWork::NewTestament));
    map.insert("1 Thes", ("1-thes", StandardWork::NewTestament));
    map.insert("2 Thes", ("2-thes", StandardWork::NewTestament));
    map.insert("1 Tim", ("1-tim", StandardWork::NewTestament));
    map.insert("2 Tim", ("2-tim", StandardWork::NewTestament));
    map.insert("Titus", ("titus", StandardWork::NewTestament));
    map.insert("Philem", ("philem", StandardWork::NewTestament));
    map.insert("Heb", ("heb", StandardWork::NewTestament));
    map.insert("James", ("james", StandardWork::NewTestament));
    map.insert("1 Pet", ("1-pet", StandardWork::NewTestament));
    map.insert("2 Pet", ("2-pet", StandardWork::NewTestament));
    map.insert("1 Jn", ("1-jn", StandardWork::NewTestament));
    map.insert("2 Jn", ("2-jn", StandardWork::NewTestament));
    map.insert("3 Jn", ("3-jn", StandardWork::NewTestament));
    map.insert("Jude", ("jude", StandardWork::NewTestament));
    map.insert("Rev", ("rev", StandardWork::NewTestament));

    // Book of Mormon - periods are handled by parser normalization
    map.insert("1 Ne", ("1-ne", StandardWork::BookOfMormon));
    map.insert("2 Ne", ("2-ne", StandardWork::BookOfMormon));
    map.insert("Jacob", ("jacob", StandardWork::BookOfMormon));
    map.insert("Enos", ("enos", StandardWork::BookOfMormon));
    map.insert("Jarom", ("jarom", StandardWork::BookOfMormon));
    map.insert("Omni", ("omni", StandardWork::BookOfMormon));
    map.insert("W of M", ("w-of-m", StandardWork::BookOfMormon));
    map.insert("Mosiah", ("mosiah", StandardWork::BookOfMormon));
    map.insert("Alma", ("alma", StandardWork::BookOfMormon));
    map.insert("Hel", ("hel", StandardWork::BookOfMormon));
    map.insert("3 Ne", ("3-ne", StandardWork::BookOfMormon));
    map.insert("4 Ne", ("4-ne", StandardWork::BookOfMormon));
    map.insert("Morm", ("morm", StandardWork::BookOfMormon));
    map.insert("Ether", ("ether", StandardWork::BookOfMormon));
    map.insert("Moro", ("moro", StandardWork::BookOfMormon));

    // Book of Mormon - Full names (alternative spellings)
    map.insert("1 Nephi", ("1-ne", StandardWork::BookOfMormon));
    map.insert("2 Nephi", ("2-ne", StandardWork::BookOfMormon));
    map.insert("Words of Mormon", ("w-of-m", StandardWork::BookOfMormon));
    map.insert("Helaman", ("hel", StandardWork::BookOfMormon));
    map.insert("3 Nephi", ("3-ne", StandardWork::BookOfMormon));
    map.insert("4 Nephi", ("4-ne", StandardWork::BookOfMormon));
    map.insert("Mormon", ("morm", StandardWork::BookOfMormon));
    map.insert("Moroni", ("moro", StandardWork::BookOfMormon));

    // Book of Mormon - Compact abbreviations (no spaces) - periods handled by parser
    map.insert("1Ne", ("1-ne", StandardWork::BookOfMormon));
    map.insert("2Ne", ("2-ne", StandardWork::BookOfMormon));
    map.insert("3Ne", ("3-ne", StandardWork::BookOfMormon));
    map.insert("4Ne", ("4-ne", StandardWork::BookOfMormon));
    map.insert("Hel", ("hel", StandardWork::BookOfMormon));
    map.insert("Morm", ("morm", StandardWork::BookOfMormon));
    map.insert("Moro", ("moro", StandardWork::BookOfMormon));

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

    // Doctrine and Covenants
    map.insert("D&C", ("dc", StandardWork::DoctrineAndCovenants));
    map.insert("OD", ("od", StandardWork::DoctrineAndCovenants));
    map.insert(
        "Doctrine and Covenants",
        ("dc", StandardWork::DoctrineAndCovenants),
    );
    map.insert(
        "Doctrine & Covenants",
        ("dc", StandardWork::DoctrineAndCovenants),
    );

    // Pearl of Great Price - periods handled by parser normalization
    map.insert("Moses", ("moses", StandardWork::PearlOfGreatPrice));
    map.insert("Abr", ("abr", StandardWork::PearlOfGreatPrice));
    map.insert("JS—M", ("js-m", StandardWork::PearlOfGreatPrice));
    map.insert("JS—H", ("js-h", StandardWork::PearlOfGreatPrice));
    map.insert("A of F", ("a-of-f", StandardWork::PearlOfGreatPrice));

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abbreviation_map_contains_basics() {
        let map = create_abbreviation_map();

        // Test some basic entries (periods are handled by parser normalization)
        assert!(map.contains_key("Gen"));
        assert!(map.contains_key("Matt"));
        assert!(map.contains_key("1 Ne"));
        assert!(map.contains_key("D&C"));
        assert!(map.contains_key("Moses"));
    }

    #[test]
    fn test_abbreviation_map_full_names() {
        let map = create_abbreviation_map();

        // Test full names work
        assert!(map.contains_key("Genesis"));
        assert!(map.contains_key("Matthew"));
        assert!(map.contains_key("2 Nephi"));
        assert!(map.contains_key("Doctrine and Covenants"));
    }

    #[test]
    fn test_abbreviation_map_standard_works() {
        let map = create_abbreviation_map();

        let (_, standard_work) = map.get("Gen").unwrap();
        assert_eq!(*standard_work, StandardWork::OldTestament);

        let (_, standard_work) = map.get("Matt").unwrap();
        assert_eq!(*standard_work, StandardWork::NewTestament);

        let (_, standard_work) = map.get("1 Ne").unwrap();
        assert_eq!(*standard_work, StandardWork::BookOfMormon);
    }

    #[test]
    fn test_abbreviation_map_without_periods() {
        let map = create_abbreviation_map();
        // Test that non-period versions work (periods are handled by parser normalization)
        assert!(map.contains_key("Gen"));
        assert!(map.contains_key("Matt"));
        assert!(map.contains_key("1 Ne"));
        assert!(map.contains_key("2 Ne"));
        assert!(map.contains_key("Hel"));

        // Verify they map to correct values
        assert_eq!(
            map.get("Gen").unwrap(),
            &("gen", StandardWork::OldTestament)
        );
        assert_eq!(
            map.get("Matt").unwrap(),
            &("matt", StandardWork::NewTestament)
        );
        assert_eq!(
            map.get("1 Ne").unwrap(),
            &("1-ne", StandardWork::BookOfMormon)
        );
    }
}

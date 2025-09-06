//! Core types used throughout the application

use serde::{Deserialize, Serialize};

/// Represents a parsed scripture reference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptureReference {
    pub book: String,
    pub chapter: u32,
    pub verse_start: u32,
    pub verse_end: Option<u32>,
    pub standard_work: StandardWork,
}

/// Standard works of LDS scripture
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandardWork {
    OldTestament,
    NewTestament,
    BookOfMormon,
    DoctrineAndCovenants,
    PearlOfGreatPrice,
}

impl StandardWork {
    /// Convert to URL path component
    pub fn to_url_path(&self) -> &'static str {
        match self {
            StandardWork::OldTestament => "ot",
            StandardWork::NewTestament => "nt",
            StandardWork::BookOfMormon => "bofm",
            StandardWork::DoctrineAndCovenants => "dc-testament",
            StandardWork::PearlOfGreatPrice => "pgp",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_work_url_paths() {
        assert_eq!(StandardWork::OldTestament.to_url_path(), "ot");
        assert_eq!(StandardWork::NewTestament.to_url_path(), "nt");
        assert_eq!(StandardWork::BookOfMormon.to_url_path(), "bofm");
        assert_eq!(
            StandardWork::DoctrineAndCovenants.to_url_path(),
            "dc-testament"
        );
        assert_eq!(StandardWork::PearlOfGreatPrice.to_url_path(), "pgp");
    }

    #[test]
    fn test_scripture_reference_creation() {
        let reference = ScriptureReference {
            book: "gen".to_string(),
            chapter: 1,
            verse_start: 1,
            verse_end: None,
            standard_work: StandardWork::OldTestament,
        };

        assert_eq!(reference.book, "gen");
        assert_eq!(reference.chapter, 1);
        assert_eq!(reference.verse_start, 1);
        assert_eq!(reference.verse_end, None);
        assert_eq!(reference.standard_work, StandardWork::OldTestament);
    }
}

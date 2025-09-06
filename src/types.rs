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
    /// For Study Helps, this contains the topic/entry name (e.g., "abel", "faith")
    pub topic: Option<String>,
}

/// Standard works of LDS scripture and study helps
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandardWork {
    OldTestament,
    NewTestament,
    BookOfMormon,
    DoctrineAndCovenants,
    PearlOfGreatPrice,
    StudyHelps,
}

impl StandardWork {
    /// Convert to URL path component
    #[must_use]
    pub const fn to_url_path(&self) -> &'static str {
        match self {
            Self::OldTestament => "ot",
            Self::NewTestament => "nt",
            Self::BookOfMormon => "bofm",
            Self::DoctrineAndCovenants => "dc-testament",
            Self::PearlOfGreatPrice => "pgp",
            Self::StudyHelps => "study-helps", // This won't be used directly for URL generation
        }
    }

    /// Check if this is a study help resource (uses different URL pattern)
    #[must_use]
    pub const fn is_study_help(&self) -> bool {
        matches!(self, Self::StudyHelps)
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
        assert_eq!(StandardWork::StudyHelps.to_url_path(), "study-helps");
    }

    #[test]
    fn test_is_study_help() {
        assert!(!StandardWork::OldTestament.is_study_help());
        assert!(!StandardWork::NewTestament.is_study_help());
        assert!(!StandardWork::BookOfMormon.is_study_help());
        assert!(!StandardWork::DoctrineAndCovenants.is_study_help());
        assert!(!StandardWork::PearlOfGreatPrice.is_study_help());
        assert!(StandardWork::StudyHelps.is_study_help());
    }

    #[test]
    fn test_scripture_reference_creation() {
        let reference = ScriptureReference {
            book: "gen".to_string(),
            chapter: 1,
            verse_start: 1,
            verse_end: None,
            standard_work: StandardWork::OldTestament,
            topic: None,
        };

        assert_eq!(reference.book, "gen");
        assert_eq!(reference.chapter, 1);
        assert_eq!(reference.verse_start, 1);
        assert_eq!(reference.verse_end, None);
        assert_eq!(reference.standard_work, StandardWork::OldTestament);
    }
}

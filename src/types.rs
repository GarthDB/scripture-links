//! Core types used throughout the application

use serde::{Deserialize, Serialize};

/// Output format for scripture links
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// Standard markdown `[text](url)` links
    #[default]
    Markdown,
    /// Obsidian-style `[[Book Chapter]]:Verse` wikilinks
    Wikilink,
}

/// A single verse range: `(start, end)` where `end` is `None` for a bare verse.
pub type VerseRange = (u32, Option<u32>);

/// Represents a parsed scripture reference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptureReference {
    pub book: String,
    pub chapter: u32,
    /// One or more `(start, end)` verse ranges, e.g. `[(1, Some(11)), (19, Some(22))]`
    /// for "1-11, 19-22". Empty for a chapter-only reference (e.g. "Isaiah 29", no
    /// verse specified).
    pub verses: Vec<VerseRange>,
    pub standard_work: StandardWork,
    /// For Study Helps, this contains the topic/entry name (e.g., "abel", "faith")
    pub topic: Option<String>,
}

impl ScriptureReference {
    /// Render the verse ranges as a human-facing suffix, e.g. `"1-11, 19-22"`.
    /// Empty string for a chapter-only reference.
    #[must_use]
    pub fn verse_display(&self) -> String {
        self.verses
            .iter()
            .map(|(start, end)| {
                end.map_or_else(|| start.to_string(), |end| format!("{start}-{end}"))
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
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
            verses: vec![(1, None)],
            standard_work: StandardWork::OldTestament,
            topic: None,
        };

        assert_eq!(reference.book, "gen");
        assert_eq!(reference.chapter, 1);
        assert_eq!(reference.verses, vec![(1, None)]);
        assert_eq!(reference.standard_work, StandardWork::OldTestament);
    }

    #[test]
    fn test_verse_display() {
        let single = ScriptureReference {
            book: "gen".to_string(),
            chapter: 1,
            verses: vec![(1, None)],
            standard_work: StandardWork::OldTestament,
            topic: None,
        };
        assert_eq!(single.verse_display(), "1");

        let range = ScriptureReference {
            book: "gen".to_string(),
            chapter: 1,
            verses: vec![(1, Some(11))],
            standard_work: StandardWork::OldTestament,
            topic: None,
        };
        assert_eq!(range.verse_display(), "1-11");

        let multi = ScriptureReference {
            book: "isa".to_string(),
            chapter: 13,
            verses: vec![(1, Some(11)), (19, Some(22))],
            standard_work: StandardWork::OldTestament,
            topic: None,
        };
        assert_eq!(multi.verse_display(), "1-11, 19-22");

        let chapter_only = ScriptureReference {
            book: "isa".to_string(),
            chapter: 29,
            verses: vec![],
            standard_work: StandardWork::OldTestament,
            topic: None,
        };
        assert_eq!(chapter_only.verse_display(), "");
    }
}

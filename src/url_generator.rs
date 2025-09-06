//! URL generation for scripture references

use crate::types::ScriptureReference;

/// Convert a topic name to a URL slug
/// Examples: "Aaron, Brother of Moses" -> "aaron-brother-of-moses"
///           "Faith" -> "faith"
fn topic_to_slug(topic: &str) -> String {
    topic
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == ',' || c == '.' {
                '-'
            } else {
                c // Keep other characters as-is for now
            }
        })
        .collect::<String>()
        // Remove multiple consecutive dashes and trim dashes from ends
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Generate a URL for a scripture reference on ChurchofJesusChrist.org
///
/// # Examples
///
/// ```
/// use scripture_links_lib::{ScriptureReference, StandardWork, generate_url};
///
/// let scripture = ScriptureReference {
///     book: "gen".to_string(),
///     chapter: 1,
///     verse_start: 1,
///     verse_end: None,
///     standard_work: StandardWork::OldTestament,
///     topic: None,
/// };
///
/// let url = generate_url(&scripture);
/// assert!(url.contains("https://www.churchofjesuschrist.org/study/scriptures"));
/// ```
#[must_use]
pub fn generate_url(scripture: &ScriptureReference) -> String {
    let base_url = "https://www.churchofjesuschrist.org/study/scriptures";

    // Study Helps use different URL patterns
    if scripture.standard_work.is_study_help() {
        if let Some(topic) = &scripture.topic {
            let slug = topic_to_slug(topic);
            // Different Study Helps have different URL patterns
            match scripture.book.as_str() {
                "it" => {
                    // Index to the Triple Combination uses "triple-index" in URL
                    return format!("{base_url}/triple-index/{slug}?lang=eng");
                }
                _ => {
                    // Other Study Helps use the abbreviation directly
                    return format!("{base_url}/{}/{}?lang=eng", scripture.book, slug);
                }
            }
        }
        // If no topic specified, link to the main study help page
        match scripture.book.as_str() {
            "it" => return format!("{base_url}/triple-index?lang=eng"),
            _ => return format!("{base_url}/{}?lang=eng", scripture.book),
        }
    }

    // Regular scripture references
    let standard_work_path = scripture.standard_work.to_url_path();
    let book_path = &scripture.book;

    let id_param = scripture.verse_end.map_or_else(
        || format!("p{}", scripture.verse_start),
        |end_verse| format!("p{}-{}", scripture.verse_start, end_verse),
    );

    let fragment = format!("p{}", scripture.verse_start);

    format!(
        "{}/{}/{}/{}?lang=eng&id={}#{}",
        base_url, standard_work_path, book_path, scripture.chapter, id_param, fragment
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ScriptureReference, StandardWork};

    #[test]
    fn test_generate_url_single_verse() {
        let scripture = ScriptureReference {
            book: "isa".to_string(),
            chapter: 6,
            verse_start: 5,
            verse_end: None,
            standard_work: StandardWork::OldTestament,
            topic: None,
        };
        let url = generate_url(&scripture);
        assert_eq!(
            url,
            "https://www.churchofjesuschrist.org/study/scriptures/ot/isa/6?lang=eng&id=p5#p5"
        );
    }

    #[test]
    fn test_generate_url_verse_range() {
        let scripture = ScriptureReference {
            book: "2-ne".to_string(),
            chapter: 10,
            verse_start: 14,
            verse_end: Some(15),
            standard_work: StandardWork::BookOfMormon,
            topic: None,
        };
        let url = generate_url(&scripture);
        assert_eq!(
            url,
            "https://www.churchofjesuschrist.org/study/scriptures/bofm/2-ne/10?lang=eng&id=p14-15#p14"
        );
    }

    #[test]
    fn test_url_contains_required_components() {
        let scripture = ScriptureReference {
            book: "matt".to_string(),
            chapter: 5,
            verse_start: 3,
            verse_end: Some(4),
            standard_work: StandardWork::NewTestament,
            topic: None,
        };
        let url = generate_url(&scripture);

        assert!(url.contains("https://www.churchofjesuschrist.org/study/scriptures"));
        assert!(url.contains("nt/matt/5"));
        assert!(url.contains("lang=eng"));
        assert!(url.contains("id=p3-4"));
        assert!(url.contains("#p3"));
    }

    #[test]
    fn test_generate_url_study_helps() {
        let topical_guide = ScriptureReference {
            book: "tg".to_string(),
            chapter: 1, // These values are not used for Study Helps
            verse_start: 1,
            verse_end: None,
            standard_work: StandardWork::StudyHelps,
            topic: Some("faith".to_string()),
        };
        let url = generate_url(&topical_guide);
        assert_eq!(
            url,
            "https://www.churchofjesuschrist.org/study/scriptures/tg/faith?lang=eng"
        );

        let bible_dictionary = ScriptureReference {
            book: "bd".to_string(),
            chapter: 1,
            verse_start: 1,
            verse_end: None,
            standard_work: StandardWork::StudyHelps,
            topic: Some("abraham".to_string()),
        };
        let url = generate_url(&bible_dictionary);
        assert_eq!(
            url,
            "https://www.churchofjesuschrist.org/study/scriptures/bd/abraham?lang=eng"
        );

        let jst = ScriptureReference {
            book: "jst".to_string(),
            chapter: 1,
            verse_start: 1,
            verse_end: None,
            standard_work: StandardWork::StudyHelps,
            topic: None, // JST might not have specific topics
        };
        let url = generate_url(&jst);
        assert_eq!(
            url,
            "https://www.churchofjesuschrist.org/study/scriptures/jst?lang=eng"
        );
    }

    #[test]
    fn test_topic_to_slug() {
        assert_eq!(topic_to_slug("Abel"), "abel");
        assert_eq!(
            topic_to_slug("Aaron, Brother of Moses"),
            "aaron-brother-of-moses"
        );
        assert_eq!(topic_to_slug("Faith"), "faith");
        assert_eq!(topic_to_slug("Jesus Christ"), "jesus-christ");
        assert_eq!(topic_to_slug("Plan of Salvation"), "plan-of-salvation");
    }

    #[test]
    fn test_study_helps_with_complex_topics() {
        let complex_topic = ScriptureReference {
            book: "gs".to_string(),
            chapter: 1,
            verse_start: 1,
            verse_end: None,
            standard_work: StandardWork::StudyHelps,
            topic: Some("Aaron, Brother of Moses".to_string()),
        };
        let url = generate_url(&complex_topic);
        assert_eq!(
            url,
            "https://www.churchofjesuschrist.org/study/scriptures/gs/aaron-brother-of-moses?lang=eng"
        );
    }

    #[test]
    fn test_index_to_triple_combination_url() {
        let it_entry = ScriptureReference {
            book: "it".to_string(),
            chapter: 1,
            verse_start: 1,
            verse_end: None,
            standard_work: StandardWork::StudyHelps,
            topic: Some("Accountability, Age of".to_string()),
        };
        let url = generate_url(&it_entry);
        assert_eq!(
            url,
            "https://www.churchofjesuschrist.org/study/scriptures/triple-index/accountability-age-of?lang=eng"
        );

        // Test without topic
        let it_main = ScriptureReference {
            book: "it".to_string(),
            chapter: 1,
            verse_start: 1,
            verse_end: None,
            standard_work: StandardWork::StudyHelps,
            topic: None,
        };
        let url = generate_url(&it_main);
        assert_eq!(
            url,
            "https://www.churchofjesuschrist.org/study/scriptures/triple-index?lang=eng"
        );
    }

    #[test]
    fn test_topic_to_slug_edge_cases() {
        // Empty string
        assert_eq!(topic_to_slug(""), "");

        // Only punctuation
        assert_eq!(topic_to_slug("..."), "");

        // Leading/trailing spaces and punctuation
        assert_eq!(topic_to_slug("  Faith  "), "faith");
        assert_eq!(topic_to_slug("...Faith..."), "faith");

        // Numbers
        assert_eq!(topic_to_slug("3 Nephi"), "3-nephi");
        assert_eq!(topic_to_slug("2nd Coming"), "2nd-coming");

        // Special characters (apostrophes are kept as-is)
        assert_eq!(topic_to_slug("Aaron's Rod"), "aaron's-rod");
        assert_eq!(
            topic_to_slug("Jesus Christ's Atonement"),
            "jesus-christ's-atonement"
        );

        // Mixed case
        assert_eq!(topic_to_slug("FAITH"), "faith");
        assert_eq!(topic_to_slug("FaItH"), "faith");
    }

    #[test]
    fn test_all_study_helps_abbreviations() {
        let test_cases = vec![
            ("jst", "joseph-smith-translation"),
            ("tg", "topical-guide"),
            ("bd", "bible-dictionary"),
            ("gs", "guide-to-scriptures"),
            ("hc", "history-of-church"),
        ];

        for (abbrev, topic) in test_cases {
            let scripture = ScriptureReference {
                book: abbrev.to_string(),
                chapter: 1,
                verse_start: 1,
                verse_end: None,
                standard_work: StandardWork::StudyHelps,
                topic: Some(topic.replace('-', " ")),
            };

            let url = generate_url(&scripture);
            assert!(url.contains(&format!("scriptures/{abbrev}/{topic}")));
            assert!(url.contains("lang=eng"));
        }
    }

    #[test]
    fn test_study_helps_url_encoding() {
        // Test that topics with special characters are properly handled
        let scripture = ScriptureReference {
            book: "bd".to_string(),
            chapter: 1,
            verse_start: 1,
            verse_end: None,
            standard_work: StandardWork::StudyHelps,
            topic: Some("Aaron's Rod & Staff".to_string()),
        };

        let url = generate_url(&scripture);
        // Should convert to slug format (apostrophes and & kept as-is)
        assert!(url.contains("bd/aaron's-rod-&-staff"));
        assert!(url.contains("lang=eng"));
    }
}

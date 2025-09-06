//! Integration tests for the library

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    // Property-based tests
    proptest! {
        #[test]
        fn test_url_generation_always_contains_required_parts(
            chapter in 1u32..150,
            verse_start in 1u32..200,
            verse_end in proptest::option::of(1u32..200)
        ) {
            use crate::{ScriptureReference, StandardWork, generate_url};

            let scripture = ScriptureReference {
                book: "gen".to_string(),
                chapter,
                verse_start,
                verse_end,
                standard_work: StandardWork::OldTestament,
            };
            let url = generate_url(&scripture);

            // All generated URLs should contain these required components
            let expected_path = format!("gen/{chapter}");
            prop_assert!(url.contains("https://www.churchofjesuschrist.org/study/scriptures"));
            prop_assert!(url.contains("lang=eng"));
            prop_assert!(url.contains(&expected_path));
            let expected_id = format!("id=p{verse_start}");
            prop_assert!(url.contains(&expected_id));
            let expected_fragment = format!("#p{verse_start}");
            prop_assert!(url.contains(&expected_fragment));
        }

        #[test]
        fn test_text_processing_preserves_non_scripture_text(
            prefix in r"[ ]{0,10}",
            suffix in r"[ ]{0,10}"
        ) {
            use crate::process_text_for_scripture_references;

            let text = format!("{prefix}Genesis 1:1{suffix}");
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

//! URL generation for scripture references

use crate::types::ScriptureReference;

/// Generate a URL for a scripture reference on ChurchofJesusChrist.org
/// 
/// # Examples
/// 
/// ```
/// use scripture_links::{ScriptureReference, StandardWork, generate_url};
/// 
/// let scripture = ScriptureReference {
///     book: "gen".to_string(),
///     chapter: 1,
///     verse_start: 1,
///     verse_end: None,
///     standard_work: StandardWork::OldTestament,
/// };
/// 
/// let url = generate_url(&scripture);
/// assert!(url.contains("https://www.churchofjesuschrist.org/study/scriptures"));
/// ```
pub fn generate_url(scripture: &ScriptureReference) -> String {
    let base_url = "https://www.churchofjesuschrist.org/study/scriptures";
    let standard_work_path = scripture.standard_work.to_url_path();
    let book_path = &scripture.book;
    
    let id_param = if let Some(end_verse) = scripture.verse_end {
        format!("p{}-{}", scripture.verse_start, end_verse)
    } else {
        format!("p{}", scripture.verse_start)
    };
    
    let fragment = format!("p{}", scripture.verse_start);
    
    format!("{}/{}/{}/{}?lang=eng&id={}#{}", 
            base_url, standard_work_path, book_path, scripture.chapter, id_param, fragment)
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

    #[test]
    fn test_url_contains_required_components() {
        let scripture = ScriptureReference {
            book: "matt".to_string(),
            chapter: 5,
            verse_start: 3,
            verse_end: Some(4),
            standard_work: StandardWork::NewTestament,
        };
        let url = generate_url(&scripture);
        
        assert!(url.contains("https://www.churchofjesuschrist.org/study/scriptures"));
        assert!(url.contains("nt/matt/5"));
        assert!(url.contains("lang=eng"));
        assert!(url.contains("id=p3-4"));
        assert!(url.contains("#p3"));
    }
}

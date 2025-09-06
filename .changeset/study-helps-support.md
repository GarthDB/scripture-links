---
"scripture-links": minor
---

Add comprehensive Study Helps support with configurable text processing

This release adds support for LDS Study Helps abbreviations and references, including:

**New Study Helps Supported:**
- JST (Joseph Smith Translation)
- TG (Topical Guide)
- BD (Bible Dictionary)
- IT (Index to the Triple Combination)
- GS (Guide to the Scriptures)
- HC (History of the Church)

**Key Features:**
- **Smart URL Generation**: Automatically generates correct URLs for Study Helps entries with topic-specific slugs
- **Index to Triple Combination Support**: Special handling for IT entries that use the `triple-index` URL pattern
- **Topic Slug Conversion**: Converts complex topics like "Aaron, Brother of Moses" to URL-friendly slugs like "aaron-brother-of-moses"
- **Configurable Text Processing**: Study Helps processing is disabled by default to avoid false positives, but can be enabled with the new `process_text_with_options()` function
- **False Positive Prevention**: Uses restrictive patterns to minimize false matches (e.g., "IT department" vs "IT Accountability")

**New API:**
- Added `process_text_with_options(text, include_study_helps)` function for configurable text processing
- Extended `ScriptureReference` type with optional `topic` field for Study Helps entries
- Added `StandardWork::StudyHelps` variant with `is_study_help()` method

**Examples:**
- `TG Faith` → `https://www.churchofjesuschrist.org/study/scriptures/tg/faith?lang=eng`
- `BD Abraham` → `https://www.churchofjesuschrist.org/study/scriptures/bd/abraham?lang=eng`
- `IT Accountability, Age of` → `https://www.churchofjesuschrist.org/study/scriptures/triple-index/accountability-age-of?lang=eng`

This enhancement maintains full backward compatibility while significantly expanding the library's capabilities for LDS scripture study resources.

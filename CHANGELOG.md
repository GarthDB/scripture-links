# scripture-links

## 1.2.1

### Patch Changes

- [`4ecfaeb`](https://github.com/GarthDB/scripture-links/commit/4ecfaeb197cfcb18ab2a559dbd11a5de8e5cd897) Thanks [@GarthDB](https://github.com/GarthDB)! - Add CLI version support and comprehensive test coverage improvements

  **New Features:**

  - **CLI Version Support**: Added `--version` and `-V` flags to display the current version
  - **Enhanced Help**: Version flag now appears in `--help` output

  **Testing Improvements:**

  - **96.66% Code Coverage**: Significantly improved test coverage with 97 total tests
  - **16 New Unit Tests**: Comprehensive testing for Study Helps functionality including:
    - Boundary word handling and punctuation boundaries
    - Case sensitivity and false positive prevention
    - Special character handling in topics
    - URL encoding and slug conversion edge cases
  - **3 New CLI Integration Tests**: Full testing of version flag functionality
  - **Edge Case Coverage**: Added tests for complex scenarios and error conditions

  **Quality Improvements:**

  - **100% Coverage** for core modules: abbreviations, scripture_data, text_processor, types, url_generator, json_output
  - **Robust Test Suite**: All edge cases and boundary conditions thoroughly tested
  - **Production Ready**: High reliability with comprehensive error handling coverage

  This patch release focuses on developer experience improvements and ensures the library is thoroughly tested and reliable for production use.

## 1.2.0

### Minor Changes

- [`a644df1`](https://github.com/GarthDB/scripture-links/commit/a644df197e1cf467458dbed6f1d21d860422b19c) Thanks [@GarthDB](https://github.com/GarthDB)! - Add comprehensive Study Helps support with configurable text processing

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

## 1.1.0

### Minor Changes

- Add LLM-friendly CLI features and fix period handling

  ### ✨ New Features

  - **JSON Output**: Add `--json` flag for structured, machine-readable responses
  - **Batch Processing**: Add `--batch` flag to process multiple comma-separated references
  - **Validation Mode**: Add `--validate-only` flag to check references without generating URLs
  - **Structured Errors**: Enhanced error handling with error codes, categories, and suggestions

  ### 🐛 Bug Fixes

  - **Period Optional Fix**: Fixed issue where `philip 4:13` failed while `philip. 4:13` worked
  - **Parser Normalization**: Periods in book abbreviations are now truly optional
  - **Cleaner Abbreviation Map**: Removed duplicate period/non-period entries (~50% size reduction)

  ### 🔧 Technical Improvements

  - Added comprehensive JSON response schemas for LLM integration
  - Enhanced error categorization (InvalidFormat, UnknownBook, InvalidChapter, etc.)
  - Improved suggestion extraction for misspelled book names
  - Better structured output for virtual assistant and automation use cases
  - Maintained full backward compatibility with existing CLI usage

## 1.0.0

### Major Changes

- [`7cf771e`](https://github.com/GarthDB/scripture-links/commit/7cf771e99ce55948c0f1de03a84d994ab23f6b66) Thanks [@GarthDB](https://github.com/GarthDB)! - # 🚀 Scripture Links v1.0.0 - Production Ready Release

  ## ✨ **What's New**

  ### **Comprehensive Scripture Reference Tool**

  - **Single Reference Mode**: Convert individual scripture references to URLs
  - **Text Processing Mode**: Find and convert scripture references in text to markdown links
  - **File Processing Mode**: Process entire files (markdown, text, etc.)
  - **Comprehensive Validation**: Chapter and verse range checking with detailed error messages

  ### **Professional Code Architecture**

  - **Modular Design**: Clean separation into 10 focused modules
  - **Library + Binary**: Can be used as both CLI tool and Rust library
  - **Comprehensive Testing**: 38 tests with property-based testing and integration tests
  - **Quality Assurance**: Zero clippy warnings, full linting, security audits

  ### **Robust Scripture Support**

  - **All Standard Works**: Old Testament, New Testament, Book of Mormon, D&C, Pearl of Great Price
  - **Multiple Formats**: Official abbreviations, full names, compact formats
  - **Case Insensitive**: Works with any capitalization
  - **Flexible Parsing**: Optional spacing, verse ranges supported

  ## 🔧 **Breaking Changes**

  **WHAT**: This is the first stable release, establishing the public API
  **WHY**: Moving from development (0.x) to production-ready (1.x) with stable interfaces
  **HOW**: No migration needed - this is the initial stable release

  ## 🛠️ **For Developers**

  ### **Library Usage**

  ```rust
  use scripture_links::{parse_scripture_reference, generate_url, process_text_for_scripture_references};

  // Parse a single reference
  let scripture = parse_scripture_reference("Genesis 1:1")?;
  let url = generate_url(&scripture);

  // Process text with multiple references
  let result = process_text_for_scripture_references("See Genesis 1:1 and 2 Nephi 10:14");
  ```

  ### **CLI Usage**

  ```bash
  # Single reference
  scripture-links --reference "Genesis 1:1"

  # Process text
  scripture-links --text "See Genesis 1:1 for creation"

  # Process file
  scripture-links --file document.md
  ```

  This release represents a fully-featured, production-ready scripture reference tool with comprehensive testing, documentation, and quality assurance.

### Minor Changes

- [`839dd50`](https://github.com/GarthDB/scripture-links/commit/839dd501fe349499243b2e7b5f9602802199a2a3) Thanks [@GarthDB](https://github.com/GarthDB)! - Add automated release system with cross-platform binary builds

  - Set up Changesets for version management and changelog generation
  - Add GitHub Actions for automated CI/CD with cross-platform builds
  - Support building binaries for Linux (x86_64, musl), macOS (x86_64, ARM64), and Windows
  - Prepare for Homebrew tap integration

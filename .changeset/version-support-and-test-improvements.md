---
"scripture-links": patch
---

Add CLI version support and comprehensive test coverage improvements

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

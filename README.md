# Scripture Links CLI Tool

A Rust command-line tool that converts scripture references into links to [ChurchofJesusChrist.org](https://www.churchofjesuschrist.org/study/scriptures).

## Usage

```bash
scripture-links --reference "Isa. 6:5"
```

## Examples

### Official Abbreviations
| Input | Output |
|-------|--------|
| `"Isa. 6:5"` | `https://www.churchofjesuschrist.org/study/scriptures/ot/isa/6?lang=eng&id=p5#p5` |
| `"Jer. 23:5"` | `https://www.churchofjesuschrist.org/study/scriptures/ot/jer/23?lang=eng&id=p5#p5` |
| `"2 Ne. 10:14-15"` | `https://www.churchofjesuschrist.org/study/scriptures/bofm/2-ne/10?lang=eng&id=p14-15#p14` |
| `"D&C 128:22-23"` | `https://www.churchofjesuschrist.org/study/scriptures/dc-testament/dc/128?lang=eng&id=p22-23#p22` |
| `"Matt. 5:3-4"` | `https://www.churchofjesuschrist.org/study/scriptures/nt/matt/5?lang=eng&id=p3-4#p3` |

### Full Book Names (Also Supported)
| Input | Output |
|-------|--------|
| `"Genesis 1:1"` | `https://www.churchofjesuschrist.org/study/scriptures/ot/gen/1?lang=eng&id=p1#p1` |
| `"2 Nephi 10:14"` | `https://www.churchofjesuschrist.org/study/scriptures/bofm/2-ne/10?lang=eng&id=p14#p14` |
| `"Matthew 5:3"` | `https://www.churchofjesuschrist.org/study/scriptures/nt/matt/5?lang=eng&id=p3#p3` |
| `"Revelation 22:21"` | `https://www.churchofjesuschrist.org/study/scriptures/nt/rev/22?lang=eng&id=p21#p21` |

## Supported Scripture References

The tool supports all standard works with their official abbreviations from [ChurchofJesusChrist.org](https://www.churchofjesuschrist.org/study/scriptures/quad/quad/abbreviations?lang=eng):

- **Old Testament**: Gen., Ex., Lev., Num., Deut., Josh., Judg., Ruth, 1 Sam., 2 Sam., 1 Kgs., 2 Kgs., 1 Chr., 2 Chr., Ezra, Neh., Esth., Job, Ps., Prov., Eccl., Song, Isa., Jer., Lam., Ezek., Dan., Hosea, Joel, Amos, Obad., Jonah, Micah, Nahum, Hab., Zeph., Hag., Zech., Mal.

- **New Testament**: Matt., Mark, Luke, John, Acts, Rom., 1 Cor., 2 Cor., Gal., Eph., Philip., Col., 1 Thes., 2 Thes., 1 Tim., 2 Tim., Titus, Philem., Heb., James, 1 Pet., 2 Pet., 1 Jn., 2 Jn., 3 Jn., Jude, Rev.

- **Book of Mormon**: 1 Ne., 2 Ne., Jacob, Enos, Jarom, Omni, W of M, Mosiah, Alma, Hel., 3 Ne., 4 Ne., Morm., Ether, Moro.

- **Doctrine and Covenants**: D&C, OD

- **Pearl of Great Price**: Moses, Abr., JS—M, JS—H, A of F

## Reference Formats

- Single verse: `"Book Chapter:Verse"` (e.g., `"Isa. 6:5"`)
- Verse range: `"Book Chapter:StartVerse-EndVerse"` (e.g., `"2 Ne. 10:14-15"`)

## Building

```bash
# Build for development
cargo build

# Build optimized release version
cargo build --release

# Run tests
cargo test
```

## Installation

The release binary is located at `target/release/scripture-links` after building.

## Validation Features

The tool includes comprehensive validation for all scripture references:

### Chapter/Verse Range Validation
```bash
$ scripture-links --reference "Gen. 51:1"
Error: Chapter 51 does not exist in Genesis. Genesis has 50 chapters (1-50)

$ scripture-links --reference "Rev. 22:22"
Error: Verse 22 does not exist in Revelation 22. Chapter 22 has 21 verses (1-21)
```

### Format and Book Validation
```bash
$ scripture-links --reference "Isaiah 6:5"
Error: Unknown book abbreviation: 'Isaiah'. Please check the spelling.

$ scripture-links --reference "Isa 6:5"
Error: Invalid scripture reference format: 'Isa 6:5'. Expected format: 'Book Chapter:Verse' or 'Book Chapter:Verse-Verse'
```

## Data Source

Scripture validation data is sourced from the comprehensive [beandog/lds-scriptures](https://github.com/beandog/lds-scriptures) repository, providing accurate chapter and verse counts for all **87 books** across **1,582 chapters** and **41,995 verses** in the LDS standard works.

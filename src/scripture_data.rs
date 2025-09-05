use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct BookInfo {
    pub name: &'static str,
    pub url_name: &'static str,
    pub standard_work: &'static str,
    pub chapters: Vec<u32>, // verse counts per chapter
}

// Complete scripture data generated from beandog/lds-scriptures repository
static SCRIPTURE_DATA: Lazy<HashMap<&'static str, BookInfo>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // OT
    map.insert("1-chr", BookInfo {
        name: "1 Chronicles",
        url_name: "1-chr",
        standard_work: "ot",
        chapters: vec![
        54, 55, 24, 43, 26, 81, 40, 40, 44, 14, // Chapters 1-10
        47, 40, 14, 17, 29, 43, 27, 17, 19, 8, // Chapters 11-20
        30, 19, 32, 31, 31, 32, 34, 21, 30 // Chapters 21-29
    ],
    });
    
    map.insert("1-kgs", BookInfo {
        name: "1 Kings",
        url_name: "1-kgs",
        standard_work: "ot",
        chapters: vec![
        53, 46, 28, 34, 18, 38, 51, 66, 28, 29, // Chapters 1-10
        43, 33, 34, 31, 34, 34, 24, 46, 21, 43, // Chapters 11-20
        29, 53 // Chapters 21-22
    ],
    });
    
    map.insert("1-sam", BookInfo {
        name: "1 Samuel",
        url_name: "1-sam",
        standard_work: "ot",
        chapters: vec![
        28, 36, 21, 22, 12, 21, 17, 22, 27, 27, // Chapters 1-10
        15, 25, 23, 52, 35, 23, 58, 30, 24, 42, // Chapters 11-20
        15, 23, 29, 22, 44, 25, 12, 25, 11, 31, // Chapters 21-30
        13 // Chapters 31-31
    ],
    });
    
    map.insert("2-chr", BookInfo {
        name: "2 Chronicles",
        url_name: "2-chr",
        standard_work: "ot",
        chapters: vec![
        17, 18, 17, 22, 14, 42, 22, 18, 31, 19, // Chapters 1-10
        23, 16, 22, 15, 19, 14, 19, 34, 11, 37, // Chapters 11-20
        20, 12, 21, 27, 28, 23, 9, 27, 36, 27, // Chapters 21-30
        21, 33, 25, 33, 27, 23 // Chapters 31-36
    ],
    });
    
    map.insert("2-kgs", BookInfo {
        name: "2 Kings",
        url_name: "2-kgs",
        standard_work: "ot",
        chapters: vec![
        18, 25, 27, 44, 27, 33, 20, 29, 37, 36, // Chapters 1-10
        21, 21, 25, 29, 38, 20, 41, 37, 37, 21, // Chapters 11-20
        26, 20, 37, 20, 30 // Chapters 21-25
    ],
    });
    
    map.insert("2-sam", BookInfo {
        name: "2 Samuel",
        url_name: "2-sam",
        standard_work: "ot",
        chapters: vec![
        27, 32, 39, 12, 25, 23, 29, 18, 13, 19, // Chapters 1-10
        27, 31, 39, 33, 37, 23, 29, 33, 43, 26, // Chapters 11-20
        22, 51, 39, 25 // Chapters 21-24
    ],
    });
    
    map.insert("amos", BookInfo {
        name: "Amos",
        url_name: "amos",
        standard_work: "ot",
        chapters: vec![15, 16, 15, 13, 27, 14, 17, 14, 15],
    });
    
    map.insert("dan", BookInfo {
        name: "Daniel",
        url_name: "dan",
        standard_work: "ot",
        chapters: vec![
        21, 49, 30, 37, 31, 28, 28, 27, 27, 21, // Chapters 1-10
        45, 13 // Chapters 11-12
    ],
    });
    
    map.insert("deut", BookInfo {
        name: "Deuteronomy",
        url_name: "deut",
        standard_work: "ot",
        chapters: vec![
        46, 37, 29, 49, 33, 25, 26, 20, 29, 22, // Chapters 1-10
        32, 32, 18, 29, 23, 22, 20, 22, 21, 20, // Chapters 11-20
        23, 30, 25, 22, 19, 19, 26, 68, 29, 20, // Chapters 21-30
        30, 52, 29, 12 // Chapters 31-34
    ],
    });
    
    map.insert("eccl", BookInfo {
        name: "Ecclesiastes",
        url_name: "eccl",
        standard_work: "ot",
        chapters: vec![
        18, 26, 22, 16, 20, 12, 29, 17, 18, 20, // Chapters 1-10
        10, 14 // Chapters 11-12
    ],
    });
    
    map.insert("esth", BookInfo {
        name: "Esther",
        url_name: "esth",
        standard_work: "ot",
        chapters: vec![22, 23, 15, 17, 14, 14, 10, 17, 32, 3],
    });
    
    map.insert("ex", BookInfo {
        name: "Exodus",
        url_name: "ex",
        standard_work: "ot",
        chapters: vec![
        22, 25, 22, 31, 23, 30, 25, 32, 35, 29, // Chapters 1-10
        10, 51, 22, 31, 27, 36, 16, 27, 25, 26, // Chapters 11-20
        36, 31, 33, 18, 40, 37, 21, 43, 46, 38, // Chapters 21-30
        18, 35, 23, 35, 35, 38, 29, 31, 43, 38 // Chapters 31-40
    ],
    });
    
    map.insert("ezek", BookInfo {
        name: "Ezekiel",
        url_name: "ezek",
        standard_work: "ot",
        chapters: vec![
        28, 10, 27, 17, 17, 14, 27, 18, 11, 22, // Chapters 1-10
        25, 28, 23, 23, 8, 63, 24, 32, 14, 49, // Chapters 11-20
        32, 31, 49, 27, 17, 21, 36, 26, 21, 26, // Chapters 21-30
        18, 32, 33, 31, 15, 38, 28, 23, 29, 49, // Chapters 31-40
        26, 20, 27, 31, 25, 24, 23, 35 // Chapters 41-48
    ],
    });
    
    map.insert("ezra", BookInfo {
        name: "Ezra",
        url_name: "ezra",
        standard_work: "ot",
        chapters: vec![11, 70, 13, 24, 17, 22, 28, 36, 15, 44],
    });
    
    map.insert("gen", BookInfo {
        name: "Genesis",
        url_name: "gen",
        standard_work: "ot",
        chapters: vec![
        31, 25, 24, 26, 32, 22, 24, 22, 29, 32, // Chapters 1-10
        32, 20, 18, 24, 21, 16, 27, 33, 38, 18, // Chapters 11-20
        34, 24, 20, 67, 34, 35, 46, 22, 35, 43, // Chapters 21-30
        55, 32, 20, 31, 29, 43, 36, 30, 23, 23, // Chapters 31-40
        57, 38, 34, 34, 28, 34, 31, 22, 33, 26 // Chapters 41-50
    ],
    });
    
    map.insert("hab", BookInfo {
        name: "Habakkuk",
        url_name: "hab",
        standard_work: "ot",
        chapters: vec![17, 20, 19],
    });
    
    map.insert("hag", BookInfo {
        name: "Haggai",
        url_name: "hag",
        standard_work: "ot",
        chapters: vec![15, 23],
    });
    
    map.insert("hosea", BookInfo {
        name: "Hosea",
        url_name: "hosea",
        standard_work: "ot",
        chapters: vec![
        11, 23, 5, 19, 15, 11, 16, 14, 17, 15, // Chapters 1-10
        12, 14, 16, 9 // Chapters 11-14
    ],
    });
    
    map.insert("isa", BookInfo {
        name: "Isaiah",
        url_name: "isa",
        standard_work: "ot",
        chapters: vec![
        31, 22, 26, 6, 30, 13, 25, 22, 21, 34, // Chapters 1-10
        16, 6, 22, 32, 9, 14, 14, 7, 25, 6, // Chapters 11-20
        17, 25, 18, 23, 12, 21, 13, 29, 24, 33, // Chapters 21-30
        9, 20, 24, 17, 10, 22, 38, 22, 8, 31, // Chapters 31-40
        29, 25, 28, 28, 25, 13, 15, 22, 26, 11, // Chapters 41-50
        23, 15, 12, 17, 13, 12, 21, 14, 21, 22, // Chapters 51-60
        11, 12, 19, 12, 25, 24 // Chapters 61-66
    ],
    });
    
    map.insert("jer", BookInfo {
        name: "Jeremiah",
        url_name: "jer",
        standard_work: "ot",
        chapters: vec![
        19, 37, 25, 31, 31, 30, 34, 22, 26, 25, // Chapters 1-10
        23, 17, 27, 22, 21, 21, 27, 23, 15, 18, // Chapters 11-20
        14, 30, 40, 10, 38, 24, 22, 17, 32, 24, // Chapters 21-30
        40, 44, 26, 22, 19, 32, 21, 28, 18, 16, // Chapters 31-40
        18, 22, 13, 30, 5, 28, 7, 47, 39, 46, // Chapters 41-50
        64, 34 // Chapters 51-52
    ],
    });
    
    map.insert("job", BookInfo {
        name: "Job",
        url_name: "job",
        standard_work: "ot",
        chapters: vec![
        22, 13, 26, 21, 27, 30, 21, 22, 35, 22, // Chapters 1-10
        20, 25, 28, 22, 35, 22, 16, 21, 29, 29, // Chapters 11-20
        34, 30, 17, 25, 6, 14, 23, 28, 25, 31, // Chapters 21-30
        40, 22, 33, 37, 16, 33, 24, 41, 30, 24, // Chapters 31-40
        34, 17 // Chapters 41-42
    ],
    });
    
    map.insert("joel", BookInfo {
        name: "Joel",
        url_name: "joel",
        standard_work: "ot",
        chapters: vec![20, 32, 21],
    });
    
    map.insert("jonah", BookInfo {
        name: "Jonah",
        url_name: "jonah",
        standard_work: "ot",
        chapters: vec![17, 10, 10, 11],
    });
    
    map.insert("josh", BookInfo {
        name: "Joshua",
        url_name: "josh",
        standard_work: "ot",
        chapters: vec![
        18, 24, 17, 24, 15, 27, 26, 35, 27, 43, // Chapters 1-10
        23, 24, 33, 15, 63, 10, 18, 28, 51, 9, // Chapters 11-20
        45, 34, 16, 33 // Chapters 21-24
    ],
    });
    
    map.insert("judg", BookInfo {
        name: "Judges",
        url_name: "judg",
        standard_work: "ot",
        chapters: vec![
        36, 23, 31, 24, 31, 40, 25, 35, 57, 18, // Chapters 1-10
        40, 15, 25, 20, 20, 31, 13, 31, 30, 48, // Chapters 11-20
        25 // Chapters 21-21
    ],
    });
    
    map.insert("lam", BookInfo {
        name: "Lamentations",
        url_name: "lam",
        standard_work: "ot",
        chapters: vec![22, 22, 66, 22, 22],
    });
    
    map.insert("lev", BookInfo {
        name: "Leviticus",
        url_name: "lev",
        standard_work: "ot",
        chapters: vec![
        17, 16, 17, 35, 19, 30, 38, 36, 24, 20, // Chapters 1-10
        47, 8, 59, 57, 33, 34, 16, 30, 37, 27, // Chapters 11-20
        24, 33, 44, 23, 55, 46, 34 // Chapters 21-27
    ],
    });
    
    map.insert("mal", BookInfo {
        name: "Malachi",
        url_name: "mal",
        standard_work: "ot",
        chapters: vec![14, 17, 18, 6],
    });
    
    map.insert("micah", BookInfo {
        name: "Micah",
        url_name: "micah",
        standard_work: "ot",
        chapters: vec![16, 13, 12, 13, 15, 16, 20],
    });
    
    map.insert("nahum", BookInfo {
        name: "Nahum",
        url_name: "nahum",
        standard_work: "ot",
        chapters: vec![15, 13, 19],
    });
    
    map.insert("neh", BookInfo {
        name: "Nehemiah",
        url_name: "neh",
        standard_work: "ot",
        chapters: vec![
        11, 20, 32, 23, 19, 19, 73, 18, 38, 39, // Chapters 1-10
        36, 47, 31 // Chapters 11-13
    ],
    });
    
    map.insert("num", BookInfo {
        name: "Numbers",
        url_name: "num",
        standard_work: "ot",
        chapters: vec![
        54, 34, 51, 49, 31, 27, 89, 26, 23, 36, // Chapters 1-10
        35, 16, 33, 45, 41, 50, 13, 32, 22, 29, // Chapters 11-20
        35, 41, 30, 25, 18, 65, 23, 31, 40, 16, // Chapters 21-30
        54, 42, 56, 29, 34, 13 // Chapters 31-36
    ],
    });
    
    map.insert("obad", BookInfo {
        name: "Obadiah",
        url_name: "obad",
        standard_work: "ot",
        chapters: vec![21],
    });
    
    map.insert("prov", BookInfo {
        name: "Proverbs",
        url_name: "prov",
        standard_work: "ot",
        chapters: vec![
        33, 22, 35, 27, 23, 35, 27, 36, 18, 32, // Chapters 1-10
        31, 28, 25, 35, 33, 33, 28, 24, 29, 30, // Chapters 11-20
        31, 29, 35, 34, 28, 28, 27, 28, 27, 33, // Chapters 21-30
        31 // Chapters 31-31
    ],
    });
    
    map.insert("ps", BookInfo {
        name: "Psalms",
        url_name: "ps",
        standard_work: "ot",
        chapters: vec![
        6, 12, 8, 8, 12, 10, 17, 9, 20, 18, // Chapters 1-10
        7, 8, 6, 7, 5, 11, 15, 50, 14, 9, // Chapters 11-20
        13, 31, 6, 10, 22, 12, 14, 9, 11, 12, // Chapters 21-30
        24, 11, 22, 22, 28, 12, 40, 22, 13, 17, // Chapters 31-40
        13, 11, 5, 26, 17, 11, 9, 14, 20, 23, // Chapters 41-50
        19, 9, 6, 7, 23, 13, 11, 11, 17, 12, // Chapters 51-60
        8, 12, 11, 10, 13, 20, 7, 35, 36, 5, // Chapters 61-70
        24, 20, 28, 23, 10, 12, 20, 72, 13, 19, // Chapters 71-80
        16, 8, 18, 12, 13, 17, 7, 18, 52, 17, // Chapters 81-90
        16, 15, 5, 23, 11, 13, 12, 9, 9, 5, // Chapters 91-100
        8, 28, 22, 35, 45, 48, 43, 13, 31, 7, // Chapters 101-110
        10, 10, 9, 8, 18, 19, 2, 29, 176, 7, // Chapters 111-120
        8, 9, 4, 8, 5, 6, 5, 6, 8, 8, // Chapters 121-130
        3, 18, 3, 3, 21, 26, 9, 8, 24, 13, // Chapters 131-140
        10, 7, 12, 15, 21, 10, 20, 14, 9, 6 // Chapters 141-150
    ],
    });
    
    map.insert("ruth", BookInfo {
        name: "Ruth",
        url_name: "ruth",
        standard_work: "ot",
        chapters: vec![22, 23, 18, 22],
    });
    
    map.insert("song", BookInfo {
        name: "Song of Solomon",
        url_name: "song",
        standard_work: "ot",
        chapters: vec![17, 17, 11, 16, 16, 13, 13, 14],
    });
    
    map.insert("zech", BookInfo {
        name: "Zechariah",
        url_name: "zech",
        standard_work: "ot",
        chapters: vec![
        21, 13, 10, 14, 11, 15, 14, 23, 17, 12, // Chapters 1-10
        17, 14, 9, 21 // Chapters 11-14
    ],
    });
    
    map.insert("zeph", BookInfo {
        name: "Zephaniah",
        url_name: "zeph",
        standard_work: "ot",
        chapters: vec![18, 15, 20],
    });
    
    // NT
    map.insert("1-cor", BookInfo {
        name: "1 Corinthians",
        url_name: "1-cor",
        standard_work: "nt",
        chapters: vec![
        31, 16, 23, 21, 13, 20, 40, 13, 27, 33, // Chapters 1-10
        34, 31, 13, 40, 58, 24 // Chapters 11-16
    ],
    });
    
    map.insert("1-jn", BookInfo {
        name: "1 John",
        url_name: "1-jn",
        standard_work: "nt",
        chapters: vec![10, 29, 24, 21, 21],
    });
    
    map.insert("1-pet", BookInfo {
        name: "1 Peter",
        url_name: "1-pet",
        standard_work: "nt",
        chapters: vec![25, 25, 22, 19, 14],
    });
    
    map.insert("1-thes", BookInfo {
        name: "1 Thessalonians",
        url_name: "1-thes",
        standard_work: "nt",
        chapters: vec![10, 20, 13, 18, 28],
    });
    
    map.insert("1-tim", BookInfo {
        name: "1 Timothy",
        url_name: "1-tim",
        standard_work: "nt",
        chapters: vec![20, 15, 16, 16, 25, 21],
    });
    
    map.insert("2-cor", BookInfo {
        name: "2 Corinthians",
        url_name: "2-cor",
        standard_work: "nt",
        chapters: vec![
        24, 17, 18, 18, 21, 18, 16, 24, 15, 18, // Chapters 1-10
        33, 21, 14 // Chapters 11-13
    ],
    });
    
    map.insert("2-jn", BookInfo {
        name: "2 John",
        url_name: "2-jn",
        standard_work: "nt",
        chapters: vec![13],
    });
    
    map.insert("2-pet", BookInfo {
        name: "2 Peter",
        url_name: "2-pet",
        standard_work: "nt",
        chapters: vec![21, 22, 18],
    });
    
    map.insert("2-thes", BookInfo {
        name: "2 Thessalonians",
        url_name: "2-thes",
        standard_work: "nt",
        chapters: vec![12, 17, 18],
    });
    
    map.insert("2-tim", BookInfo {
        name: "2 Timothy",
        url_name: "2-tim",
        standard_work: "nt",
        chapters: vec![18, 26, 17, 22],
    });
    
    map.insert("3-jn", BookInfo {
        name: "3 John",
        url_name: "3-jn",
        standard_work: "nt",
        chapters: vec![14],
    });
    
    map.insert("acts", BookInfo {
        name: "Acts",
        url_name: "acts",
        standard_work: "nt",
        chapters: vec![
        26, 47, 26, 37, 42, 15, 60, 40, 43, 48, // Chapters 1-10
        30, 25, 52, 28, 41, 40, 34, 28, 41, 38, // Chapters 11-20
        40, 30, 35, 27, 27, 32, 44, 31 // Chapters 21-28
    ],
    });
    
    map.insert("col", BookInfo {
        name: "Colossians",
        url_name: "col",
        standard_work: "nt",
        chapters: vec![29, 23, 25, 18],
    });
    
    map.insert("eph", BookInfo {
        name: "Ephesians",
        url_name: "eph",
        standard_work: "nt",
        chapters: vec![23, 22, 21, 32, 33, 24],
    });
    
    map.insert("gal", BookInfo {
        name: "Galatians",
        url_name: "gal",
        standard_work: "nt",
        chapters: vec![24, 21, 29, 31, 26, 18],
    });
    
    map.insert("heb", BookInfo {
        name: "Hebrews",
        url_name: "heb",
        standard_work: "nt",
        chapters: vec![
        14, 18, 19, 16, 14, 20, 28, 13, 28, 39, // Chapters 1-10
        40, 29, 25 // Chapters 11-13
    ],
    });
    
    map.insert("james", BookInfo {
        name: "James",
        url_name: "james",
        standard_work: "nt",
        chapters: vec![27, 26, 18, 17, 20],
    });
    
    map.insert("john", BookInfo {
        name: "John",
        url_name: "john",
        standard_work: "nt",
        chapters: vec![
        51, 25, 36, 54, 47, 71, 53, 59, 41, 42, // Chapters 1-10
        57, 50, 38, 31, 27, 33, 26, 40, 42, 31, // Chapters 11-20
        25 // Chapters 21-21
    ],
    });
    
    map.insert("jude", BookInfo {
        name: "Jude",
        url_name: "jude",
        standard_work: "nt",
        chapters: vec![25],
    });
    
    map.insert("luke", BookInfo {
        name: "Luke",
        url_name: "luke",
        standard_work: "nt",
        chapters: vec![
        80, 52, 38, 44, 39, 49, 50, 56, 62, 42, // Chapters 1-10
        54, 59, 35, 35, 32, 31, 37, 43, 48, 47, // Chapters 11-20
        38, 71, 56, 53 // Chapters 21-24
    ],
    });
    
    map.insert("mark", BookInfo {
        name: "Mark",
        url_name: "mark",
        standard_work: "nt",
        chapters: vec![
        45, 28, 35, 41, 43, 56, 37, 38, 50, 52, // Chapters 1-10
        33, 44, 37, 72, 47, 20 // Chapters 11-16
    ],
    });
    
    map.insert("matt", BookInfo {
        name: "Matthew",
        url_name: "matt",
        standard_work: "nt",
        chapters: vec![
        25, 23, 17, 25, 48, 34, 29, 34, 38, 42, // Chapters 1-10
        30, 50, 58, 36, 39, 28, 27, 35, 30, 34, // Chapters 11-20
        46, 46, 39, 51, 46, 75, 66, 20 // Chapters 21-28
    ],
    });
    
    map.insert("philem", BookInfo {
        name: "Philemon",
        url_name: "philem",
        standard_work: "nt",
        chapters: vec![25],
    });
    
    map.insert("philip", BookInfo {
        name: "Philippians",
        url_name: "philip",
        standard_work: "nt",
        chapters: vec![30, 30, 21, 23],
    });
    
    map.insert("rev", BookInfo {
        name: "Revelation",
        url_name: "rev",
        standard_work: "nt",
        chapters: vec![
        20, 29, 22, 11, 14, 17, 17, 13, 21, 11, // Chapters 1-10
        19, 17, 18, 20, 8, 21, 18, 24, 21, 15, // Chapters 11-20
        27, 21 // Chapters 21-22
    ],
    });
    
    map.insert("rom", BookInfo {
        name: "Romans",
        url_name: "rom",
        standard_work: "nt",
        chapters: vec![
        32, 29, 31, 25, 21, 23, 25, 39, 33, 21, // Chapters 1-10
        36, 21, 14, 23, 33, 27 // Chapters 11-16
    ],
    });
    
    map.insert("titus", BookInfo {
        name: "Titus",
        url_name: "titus",
        standard_work: "nt",
        chapters: vec![16, 15, 15],
    });
    
    // BOFM
    map.insert("1-ne", BookInfo {
        name: "1 Nephi",
        url_name: "1-ne",
        standard_work: "bofm",
        chapters: vec![
        20, 24, 31, 38, 22, 6, 22, 38, 6, 22, // Chapters 1-10
        36, 23, 42, 30, 36, 39, 55, 25, 24, 22, // Chapters 11-20
        26, 31 // Chapters 21-22
    ],
    });
    
    map.insert("2-ne", BookInfo {
        name: "2 Nephi",
        url_name: "2-ne",
        standard_work: "bofm",
        chapters: vec![
        32, 30, 25, 35, 34, 18, 11, 25, 54, 25, // Chapters 1-10
        8, 22, 26, 6, 30, 13, 25, 22, 21, 34, // Chapters 11-20
        16, 6, 22, 32, 30, 33, 35, 32, 14, 18, // Chapters 21-30
        21, 9, 15 // Chapters 31-33
    ],
    });
    
    map.insert("3-ne", BookInfo {
        name: "3 Nephi",
        url_name: "3-ne",
        standard_work: "bofm",
        chapters: vec![
        30, 19, 26, 33, 26, 30, 26, 25, 22, 19, // Chapters 1-10
        41, 48, 34, 27, 24, 20, 25, 39, 36, 46, // Chapters 11-20
        29, 17, 14, 18, 6, 21, 33, 40, 9, 2 // Chapters 21-30
    ],
    });
    
    map.insert("4-ne", BookInfo {
        name: "4 Nephi",
        url_name: "4-ne",
        standard_work: "bofm",
        chapters: vec![49],
    });
    
    map.insert("alma", BookInfo {
        name: "Alma",
        url_name: "alma",
        standard_work: "bofm",
        chapters: vec![
        33, 38, 27, 20, 62, 8, 27, 32, 34, 32, // Chapters 1-10
        46, 37, 31, 29, 19, 21, 39, 43, 36, 30, // Chapters 11-20
        23, 35, 18, 30, 17, 37, 30, 14, 17, 60, // Chapters 21-30
        38, 43, 23, 41, 16, 30, 47, 15, 19, 26, // Chapters 31-40
        15, 31, 54, 24, 24, 41, 36, 25, 30, 40, // Chapters 41-50
        37, 40, 23, 24, 35, 57, 36, 41, 13, 36, // Chapters 51-60
        21, 52, 17 // Chapters 61-63
    ],
    });
    
    map.insert("enos", BookInfo {
        name: "Enos",
        url_name: "enos",
        standard_work: "bofm",
        chapters: vec![27],
    });
    
    map.insert("ether", BookInfo {
        name: "Ether",
        url_name: "ether",
        standard_work: "bofm",
        chapters: vec![
        43, 25, 28, 19, 6, 30, 27, 26, 35, 34, // Chapters 1-10
        23, 41, 31, 31, 34 // Chapters 11-15
    ],
    });
    
    map.insert("hel", BookInfo {
        name: "Helaman",
        url_name: "hel",
        standard_work: "bofm",
        chapters: vec![
        34, 14, 37, 26, 52, 41, 29, 28, 41, 19, // Chapters 1-10
        38, 26, 39, 31, 17, 25 // Chapters 11-16
    ],
    });
    
    map.insert("jacob", BookInfo {
        name: "Jacob",
        url_name: "jacob",
        standard_work: "bofm",
        chapters: vec![19, 35, 14, 18, 77, 13, 27],
    });
    
    map.insert("jarom", BookInfo {
        name: "Jarom",
        url_name: "jarom",
        standard_work: "bofm",
        chapters: vec![15],
    });
    
    map.insert("morm", BookInfo {
        name: "Mormon",
        url_name: "morm",
        standard_work: "bofm",
        chapters: vec![19, 29, 22, 23, 24, 22, 10, 41, 37],
    });
    
    map.insert("moro", BookInfo {
        name: "Moroni",
        url_name: "moro",
        standard_work: "bofm",
        chapters: vec![4, 3, 4, 3, 2, 9, 48, 30, 26, 34],
    });
    
    map.insert("mosiah", BookInfo {
        name: "Mosiah",
        url_name: "mosiah",
        standard_work: "bofm",
        chapters: vec![
        18, 41, 27, 30, 15, 7, 33, 21, 19, 22, // Chapters 1-10
        29, 37, 35, 12, 31, 15, 20, 35, 29, 26, // Chapters 11-20
        36, 16, 39, 25, 24, 39, 37, 20, 47 // Chapters 21-29
    ],
    });
    
    map.insert("omni", BookInfo {
        name: "Omni",
        url_name: "omni",
        standard_work: "bofm",
        chapters: vec![30],
    });
    
    map.insert("w-of-m", BookInfo {
        name: "Words of Mormon",
        url_name: "w-of-m",
        standard_work: "bofm",
        chapters: vec![18],
    });
    
    // DC-TESTAMENT
    map.insert("dc", BookInfo {
        name: "Doctrine and Covenants",
        url_name: "dc",
        standard_work: "dc-testament",
        chapters: vec![
        39, 3, 20, 7, 35, 37, 8, 12, 14, 70, // Chapters 1-10
        30, 9, 1, 11, 6, 6, 9, 47, 41, 84, // Chapters 11-20
        12, 4, 7, 19, 16, 2, 18, 16, 50, 11, // Chapters 21-30
        13, 5, 18, 12, 27, 8, 4, 42, 24, 3, // Chapters 31-40
        12, 93, 35, 6, 75, 33, 4, 6, 28, 46, // Chapters 41-50
        20, 44, 7, 10, 6, 20, 16, 65, 24, 17, // Chapters 51-60
        39, 9, 66, 43, 6, 13, 14, 35, 8, 18, // Chapters 61-70
        11, 26, 6, 7, 36, 119, 15, 22, 4, 5, // Chapters 71-80
        7, 24, 6, 120, 12, 11, 8, 141, 21, 37, // Chapters 81-90
        6, 2, 53, 17, 17, 9, 28, 48, 8, 17, // Chapters 91-100
        101, 34, 40, 86, 41, 8, 100, 8, 80, 16, // Chapters 101-110
        11, 34, 10, 2, 19, 1, 16, 6, 7, 1, // Chapters 111-120
        46, 9, 17, 145, 4, 3, 12, 25, 9, 23, // Chapters 121-130
        8, 66, 74, 12, 7, 42, 10, 60 // Chapters 131-138
    ],
    });
    
    // PGP
    map.insert("a-of-f", BookInfo {
        name: "Articles of Faith",
        url_name: "a-of-f",
        standard_work: "pgp",
        chapters: vec![13],
    });
    
    map.insert("abr", BookInfo {
        name: "Abraham",
        url_name: "abr",
        standard_work: "pgp",
        chapters: vec![31, 25, 28, 31, 21],
    });
    
    map.insert("joseph-smith--history", BookInfo {
        name: "Joseph Smith--History",
        url_name: "joseph-smith--history",
        standard_work: "pgp",
        chapters: vec![75],
    });
    
    map.insert("joseph-smith--matthew", BookInfo {
        name: "Joseph Smith--Matthew",
        url_name: "joseph-smith--matthew",
        standard_work: "pgp",
        chapters: vec![55],
    });
    
    map.insert("moses", BookInfo {
        name: "Moses",
        url_name: "moses",
        standard_work: "pgp",
        chapters: vec![42, 31, 25, 32, 59, 68, 69, 30],
    });
    
    map
});

pub fn get_book_info(book_key: &str) -> Option<&BookInfo> {
    SCRIPTURE_DATA.get(book_key)
}

pub fn validate_chapter_range(book_key: &str, chapter: u32) -> Result<(), String> {
    match get_book_info(book_key) {
        Some(book_info) => {
            let total_chapters = book_info.chapters.len() as u32;
            if chapter == 0 {
                Err(format!("Chapter number must be greater than 0"))
            } else if chapter > total_chapters {
                Err(format!(
                    "Chapter {} does not exist in {}. {} has {} chapters (1-{})",
                    chapter, book_info.name, book_info.name, total_chapters, total_chapters
                ))
            } else {
                Ok(())
            }
        }
        None => {
            // If we don't have data for this book, we can't validate
            // This allows the system to work with books we haven't included yet
            Ok(())
        }
    }
}

pub fn validate_verse_range(book_key: &str, chapter: u32, verse_start: u32, verse_end: Option<u32>) -> Result<(), String> {
    match get_book_info(book_key) {
        Some(book_info) => {
            // First validate the chapter exists
            validate_chapter_range(book_key, chapter)?;
            
            let chapter_index = (chapter - 1) as usize;
            let total_verses = book_info.chapters[chapter_index];
            
            // Validate start verse
            if verse_start == 0 {
                return Err(format!("Verse number must be greater than 0"));
            }
            if verse_start > total_verses {
                return Err(format!(
                    "Verse {} does not exist in {} {}. Chapter {} has {} verses (1-{})",
                    verse_start, book_info.name, chapter, chapter, total_verses, total_verses
                ));
            }
            
            // Validate end verse if provided
            if let Some(end_verse) = verse_end {
                if end_verse < verse_start {
                    return Err(format!(
                        "End verse ({}) cannot be less than start verse ({})",
                        end_verse, verse_start
                    ));
                }
                if end_verse > total_verses {
                    return Err(format!(
                        "Verse {} does not exist in {} {}. Chapter {} has {} verses (1-{})",
                        end_verse, book_info.name, chapter, chapter, total_verses, total_verses
                    ));
                }
            }
            
            Ok(())
        }
        None => {
            // If we don't have data for this book, we can't validate
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_data() {
        let genesis = get_book_info("gen").unwrap();
        assert_eq!(genesis.name, "Genesis");
        assert_eq!(genesis.chapters.len(), 50);
        assert_eq!(genesis.chapters[0], 31); // Genesis 1 has 31 verses
    }

    #[test]
    fn test_valid_chapter_range() {
        assert!(validate_chapter_range("gen", 1).is_ok());
        assert!(validate_chapter_range("gen", 50).is_ok());
        assert!(validate_chapter_range("ruth", 4).is_ok());
    }

    #[test]
    fn test_invalid_chapter_range() {
        assert!(validate_chapter_range("gen", 0).is_err());
        assert!(validate_chapter_range("gen", 51).is_err());
        assert!(validate_chapter_range("ruth", 5).is_err());
    }

    #[test]
    fn test_valid_verse_range() {
        assert!(validate_verse_range("gen", 1, 1, None).is_ok());
        assert!(validate_verse_range("gen", 1, 31, None).is_ok());
        assert!(validate_verse_range("gen", 1, 1, Some(5)).is_ok());
        assert!(validate_verse_range("gen", 1, 25, Some(31)).is_ok());
    }

    #[test]
    fn test_invalid_verse_range() {
        assert!(validate_verse_range("gen", 1, 0, None).is_err());
        assert!(validate_verse_range("gen", 1, 32, None).is_err());
        assert!(validate_verse_range("gen", 1, 5, Some(4)).is_err()); // End before start
        assert!(validate_verse_range("gen", 1, 25, Some(32)).is_err()); // End verse too high
    }

    #[test]
    fn test_unknown_book() {
        // Unknown books should not cause errors (graceful fallback)
        assert!(validate_chapter_range("unknown", 100).is_ok());
        assert!(validate_verse_range("unknown", 100, 100, Some(200)).is_ok());
    }
}
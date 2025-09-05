use criterion::{black_box, criterion_group, criterion_main, Criterion};
use scripture_links::*; // Adjust import based on your actual module structure

// Note: You'll need to make your parsing functions public or create a lib.rs

fn benchmark_single_reference_parsing(c: &mut Criterion) {
    c.bench_function("parse single reference", |b| {
        b.iter(|| {
            // This will need to be adjusted based on your actual function signature
            // parse_scripture_reference(black_box("Genesis 1:1"))
        })
    });
}

fn benchmark_text_processing(c: &mut Criterion) {
    let large_text = "See Genesis 1:1 for creation. Also read 2 Nephi 10:14 and D&C 128:22-23 for insights. \
                     Matthew 5:3-4 teaches about the Beatitudes. Check Isaiah 55:8-9 for God's ways. \
                     Read Moroni 10:4-5 for the promise.".repeat(100);

    c.bench_function("process large text", |b| {
        b.iter(|| {
            // This will need to be adjusted based on your actual function signature  
            // process_text_for_scripture_references(black_box(&large_text))
        })
    });
}

criterion_group!(benches, benchmark_single_reference_parsing, benchmark_text_processing);
criterion_main!(benches);

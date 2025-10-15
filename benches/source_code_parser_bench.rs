use criterion::{criterion_group, criterion_main, Criterion};
use gittype::domain::models::languages::Rust;
use gittype::domain::services::source_code_parser::parsers::parse_with_thread_local;
use gittype::domain::services::source_code_parser::ChunkExtractor;
use std::path::Path;

// Load test fixture files
fn load_fixture(filename: &str) -> String {
    let fixture_path = Path::new("tests/fixtures").join(filename);
    std::fs::read_to_string(fixture_path)
        .unwrap_or_else(|_| panic!("Failed to load fixture: {}", filename))
}

fn bench_chunk_extractor(c: &mut Criterion) {
    let mut group = c.benchmark_group("chunk_extractor");

    // Load real fixture data
    let rust_code = load_fixture("complex_rust_service.rs");

    group.bench_function("extract_chunks_from_tree", |b| {
        let tree = parse_with_thread_local("rust", &rust_code).unwrap();
        let file_path = Path::new("complex_rust_service.rs");
        let git_root = Path::new(".");

        b.iter(|| {
            std::hint::black_box(ChunkExtractor::extract_chunks_from_tree(
                std::hint::black_box(&tree),
                std::hint::black_box(&rust_code),
                std::hint::black_box(file_path),
                std::hint::black_box(git_root),
                std::hint::black_box(&Rust),
            ))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_chunk_extractor);
criterion_main!(benches);

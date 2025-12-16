use criterion::{criterion_group, criterion_main, Criterion};
use gittype::domain::models::loading::StepType;
use gittype::domain::models::{CodeChunk, ExtractionOptions, Languages};
use gittype::domain::services::challenge_generator::ChallengeGenerator;
use gittype::domain::services::source_code_parser::SourceCodeParser;
use gittype::presentation::tui::screens::loading_screen::ProgressReporter;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

// Mock ProgressReporter for benchmarking
#[derive(Debug, Default)]
struct BenchProgressReporter {
    #[allow(clippy::type_complexity)]
    count_calls: Arc<Mutex<Vec<(StepType, usize, usize, Option<String>)>>>,
}

impl BenchProgressReporter {
    fn new() -> Self {
        Self::default()
    }
}

impl ProgressReporter for BenchProgressReporter {
    fn set_step(&self, _step_type: StepType) {}

    fn set_current_file(&self, _file: Option<String>) {}

    fn set_file_counts(
        &self,
        step_type: StepType,
        processed: usize,
        total: usize,
        current_file: Option<String>,
    ) {
        self.count_calls
            .lock()
            .unwrap()
            .push((step_type, processed, total, current_file));
    }
}

fn create_real_chunks_from_fixture(fixture_filename: &str) -> Vec<CodeChunk> {
    let fixture_path = PathBuf::from("tests/fixtures").join(fixture_filename);
    if !fixture_path.exists() {
        return Vec::new();
    }

    let mut parser = SourceCodeParser::new().expect("Failed to create parser");
    let rust_lang = Languages::from_extension("rs").expect("Failed to get Rust language");
    let options = ExtractionOptions::default();
    let progress = BenchProgressReporter::new();
    let files_to_process = vec![(fixture_path, rust_lang)];

    parser
        .extract_chunks_with_progress(files_to_process, &options, &progress)
        .unwrap_or_else(|_| Vec::new())
}

fn benchmark_challenge_generator(c: &mut Criterion) {
    let generator = ChallengeGenerator::new();
    let progress = BenchProgressReporter::new();

    // Get all chunks from all fixtures
    let mut all_chunks = create_real_chunks_from_fixture("complex_rust_service.rs");
    all_chunks.extend(create_real_chunks_from_fixture("complex_commented_rust.rs"));

    c.bench_function("challenge_generator_all_chunks", |b| {
        b.iter(|| {
            let challenges = generator.convert_with_progress(
                std::hint::black_box(all_chunks.clone()),
                std::hint::black_box(&progress),
            );
            std::hint::black_box(challenges)
        })
    });
}

criterion_group!(benches, benchmark_challenge_generator);
criterion_main!(benches);

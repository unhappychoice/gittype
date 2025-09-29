use gittype::domain::models::{Challenge, ChunkType, CodeChunk, DifficultyLevel, ExtractionOptions, Languages};
use gittype::domain::services::challenge_generator::ChallengeGenerator;
use gittype::domain::services::source_code_parser::SourceCodeParser;
use gittype::presentation::game::models::StepType;
use gittype::presentation::game::screens::loading_screen::ProgressReporter;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Mock ProgressReporter for testing
#[derive(Debug, Default)]
struct MockProgressReporter {
    step_calls: Arc<Mutex<Vec<StepType>>>,
    file_calls: Arc<Mutex<Vec<Option<String>>>>,
    count_calls: Arc<Mutex<Vec<(StepType, usize, usize, Option<String>)>>>,
}

impl MockProgressReporter {
    fn new() -> Self {
        Self::default()
    }

    fn get_step_calls(&self) -> Vec<StepType> {
        self.step_calls.lock().unwrap().clone()
    }

    fn get_file_calls(&self) -> Vec<Option<String>> {
        self.file_calls.lock().unwrap().clone()
    }

    fn get_count_calls(&self) -> Vec<(StepType, usize, usize, Option<String>)> {
        self.count_calls.lock().unwrap().clone()
    }
}

impl ProgressReporter for MockProgressReporter {
    fn set_step(&self, step_type: StepType) {
        self.step_calls.lock().unwrap().push(step_type);
    }

    fn set_current_file(&self, file: Option<String>) {
        self.file_calls.lock().unwrap().push(file);
    }

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

fn create_real_code_chunks_from_fixture(fixture_filename: &str) -> Vec<CodeChunk> {
    let fixture_path = PathBuf::from("tests/fixtures").join(fixture_filename);

    // Create a temporary parser
    let mut parser = SourceCodeParser::new().expect("Failed to create parser");

    // Get rust language
    let rust_lang = Languages::from_extension("rs").expect("Failed to get Rust language");

    let options = ExtractionOptions::default();
    let progress = MockProgressReporter::new();

    // Extract real chunks using the parser
    let files_to_process = vec![(fixture_path, rust_lang)];

    parser
        .extract_chunks_with_progress(files_to_process, &options, &progress)
        .unwrap_or_else(|_| Vec::new())
}

#[test]
fn new_creates_generator() {
    let generator = ChallengeGenerator::new();
    assert_eq!(std::mem::size_of_val(&generator), 0); // Zero-sized struct
}

#[test]
fn default_creates_generator() {
    let generator = ChallengeGenerator::default();
    assert_eq!(std::mem::size_of_val(&generator), 0); // Zero-sized struct
}

#[test]
fn snapshot_test_complex_rust_service_challenges() {
    let generator = ChallengeGenerator::new();
    let progress = MockProgressReporter::new();

    // Get real chunks from complex_rust_service.rs
    let real_chunks = create_real_code_chunks_from_fixture("complex_rust_service.rs");

    if real_chunks.is_empty() {
        panic!("No chunks extracted from complex_rust_service.rs fixture");
    }

    // Generate challenges
    let challenges = generator.convert_with_progress(real_chunks.clone(), &progress);

    // Verify basic properties
    assert!(!challenges.is_empty(), "Should generate at least some challenges");
    assert!(challenges.len() >= real_chunks.len() / 4, "Should generate reasonable number of challenges");

    // Analyze challenge distribution by difficulty
    let mut difficulty_counts: HashMap<Option<DifficultyLevel>, usize> = HashMap::new();
    for challenge in &challenges {
        *difficulty_counts.entry(challenge.difficulty_level).or_insert(0) += 1;
    }

    // Analyze challenge distribution by chunk type
    let mut chunk_type_counts: HashMap<ChunkType, usize> = HashMap::new();
    for chunk in &real_chunks {
        *chunk_type_counts.entry(chunk.chunk_type.clone()).or_insert(0) += 1;
    }

    // Sort for deterministic output
    let mut chunk_types_sorted: Vec<_> = chunk_type_counts.iter().collect();
    chunk_types_sorted.sort_by_key(|(k, _)| format!("{:?}", k));

    let mut difficulty_sorted: Vec<_> = difficulty_counts.iter().collect();
    difficulty_sorted.sort_by_key(|(k, _)| format!("{:?}", k));

    // Create structured data for JSON snapshot (excluding random IDs)
    let snapshot_data = serde_json::json!({
        "total_chunks": real_chunks.len(),
        "total_challenges": challenges.len(),
        "chunk_types": chunk_types_sorted.iter().map(|(k, v)| (format!("{:?}", k), v)).collect::<Vec<_>>(),
        "difficulty_distribution": difficulty_sorted.iter().map(|(k, v)| (format!("{:?}", k), v)).collect::<Vec<_>>(),
        "challenges": challenges.iter().map(|challenge| {
            serde_json::json!({
                "difficulty": format!("{:?}", challenge.difficulty_level),
                "content_length": challenge.code_content.len(),
                "source_file": challenge.source_file_path.as_deref().unwrap_or("unknown"),
                "start_line": challenge.start_line.unwrap_or(0),
                "end_line": challenge.end_line.unwrap_or(0),
                "code_content": challenge.code_content
            })
        }).collect::<Vec<_>>()
    });

    // Snapshot test with pretty-printed JSON
    let json_string = serde_json::to_string_pretty(&snapshot_data).unwrap();
    insta::assert_snapshot!("complex_rust_service_challenges", json_string);
}

#[test]
fn snapshot_test_complex_commented_rust_challenges() {
    let generator = ChallengeGenerator::new();
    let progress = MockProgressReporter::new();

    // Get real chunks from complex_commented_rust.rs
    let real_chunks = create_real_code_chunks_from_fixture("complex_commented_rust.rs");

    if real_chunks.is_empty() {
        panic!("No chunks extracted from complex_commented_rust.rs fixture");
    }

    // Generate challenges
    let challenges = generator.convert_with_progress(real_chunks.clone(), &progress);

    // Verify basic properties
    assert!(!challenges.is_empty(), "Should generate at least some challenges");

    // Analyze challenge distribution by difficulty
    let mut difficulty_counts: HashMap<Option<DifficultyLevel>, usize> = HashMap::new();
    for challenge in &challenges {
        *difficulty_counts.entry(challenge.difficulty_level).or_insert(0) += 1;
    }

    // Analyze challenge distribution by chunk type
    let mut chunk_type_counts: HashMap<ChunkType, usize> = HashMap::new();
    for chunk in &real_chunks {
        *chunk_type_counts.entry(chunk.chunk_type.clone()).or_insert(0) += 1;
    }

    // Check comment processing
    let challenges_with_comments = challenges.iter()
        .filter(|c| !c.comment_ranges.is_empty())
        .count();

    // Sort for deterministic output
    let mut chunk_types_sorted: Vec<_> = chunk_type_counts.iter().collect();
    chunk_types_sorted.sort_by_key(|(k, _)| format!("{:?}", k));

    let mut difficulty_sorted: Vec<_> = difficulty_counts.iter().collect();
    difficulty_sorted.sort_by_key(|(k, _)| format!("{:?}", k));

    // Create structured data for JSON snapshot (excluding random IDs)
    let snapshot_data = serde_json::json!({
        "total_chunks": real_chunks.len(),
        "total_challenges": challenges.len(),
        "chunk_types": chunk_types_sorted.iter().map(|(k, v)| (format!("{:?}", k), v)).collect::<Vec<_>>(),
        "difficulty_distribution": difficulty_sorted.iter().map(|(k, v)| (format!("{:?}", k), v)).collect::<Vec<_>>(),
        "challenges_with_comments": challenges_with_comments,
        "challenges": challenges.iter().map(|challenge| {
            serde_json::json!({
                "difficulty": format!("{:?}", challenge.difficulty_level),
                "content_length": challenge.code_content.len(),
                "source_file": challenge.source_file_path.as_deref().unwrap_or("unknown"),
                "start_line": challenge.start_line.unwrap_or(0),
                "end_line": challenge.end_line.unwrap_or(0),
                "comment_ranges": challenge.comment_ranges,
                "code_content": challenge.code_content
            })
        }).collect::<Vec<_>>()
    });

    // Snapshot test with pretty-printed JSON
    let json_string = serde_json::to_string_pretty(&snapshot_data).unwrap();
    insta::assert_snapshot!("complex_commented_rust_challenges", json_string);

    // Verify comment processing
    assert!(challenges_with_comments > 0, "Should have some challenges with comment ranges");
}

#[test]
fn verify_challenge_content_quality() {
    let _generator = ChallengeGenerator::new();
    let real_chunks = create_real_code_chunks_from_fixture("complex_rust_service.rs");

    if real_chunks.is_empty() {
        panic!("No chunks available for testing");
    }

    for chunk in real_chunks.iter().take(5) { // Test first 5 chunks
        if let Some(challenge) = Challenge::from_chunk(chunk, None) {
            // Verify challenge has valid content
            assert!(!challenge.code_content.is_empty(), "Challenge content should not be empty");
            assert!(!challenge.id.is_empty(), "Challenge ID should not be empty");

            // Verify line numbers match chunk
            assert_eq!(challenge.start_line, Some(chunk.start_line));
            assert_eq!(challenge.end_line, Some(chunk.end_line));

            // Verify language is set
            assert_eq!(challenge.language, Some(chunk.language.clone()));

            // Verify content comes from chunk
            assert!(chunk.content.contains(&challenge.code_content) ||
                   challenge.code_content.contains(&chunk.content),
                   "Challenge content should be related to chunk content");
        }
    }
}

#[test]
fn test_progress_reporting() {
    let generator = ChallengeGenerator::new();
    let progress = MockProgressReporter::new();
    let real_chunks = create_real_code_chunks_from_fixture("complex_rust_service.rs");

    if real_chunks.is_empty() {
        panic!("No chunks available for testing");
    }

    let _challenges = generator.convert_with_progress(real_chunks.clone(), &progress);

    // Verify progress was reported
    let count_calls = progress.get_count_calls();
    assert!(!count_calls.is_empty(), "Progress should be reported");

    // Verify final call shows completion
    let final_call = count_calls.last().unwrap();
    assert_eq!(final_call.1, final_call.2, "Final progress call should show processed == total");
    assert_eq!(final_call.0, StepType::Generating, "Should report generating step");
}


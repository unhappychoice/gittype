use gittype::presentation::game::context_loader::load_context_lines;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_load_context_lines() {
    let content =
        "line1\nline2\nline3\nTARGET_START\nTARGET_CONTENT\nTARGET_END\nline7\nline8\nline9";
    let temp_file = NamedTempFile::new().unwrap();
    fs::write(&temp_file, content).unwrap();

    let result = load_context_lines(temp_file.path(), 4, 6, 2).unwrap();

    assert_eq!(result.pre_context, vec!["line2", "line3"]);
    assert_eq!(result.post_context, vec!["line7", "line8"]);
}

#[test]
fn test_load_context_at_file_boundaries() {
    let content = "line1\nTARGET\nline3";
    let temp_file = NamedTempFile::new().unwrap();
    fs::write(&temp_file, content).unwrap();

    let result = load_context_lines(temp_file.path(), 2, 2, 5).unwrap();

    assert_eq!(result.pre_context, vec!["line1"]);
    assert_eq!(result.post_context, vec!["line3"]);
}

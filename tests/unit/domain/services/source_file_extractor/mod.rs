use gittype::domain::services::source_file_extractor::SourceFileExtractor;
use gittype::infrastructure::storage::file_storage::FileStorage;
use gittype::presentation::game::models::StepType;
use gittype::presentation::game::screens::loading_screen::ProgressReporter;
use std::path::Path;
use std::sync::RwLock;

struct MockProgressReporter {
    calls: RwLock<Vec<(StepType, usize, usize)>>,
}

impl MockProgressReporter {
    fn new() -> Self {
        Self {
            calls: RwLock::new(Vec::new()),
        }
    }

    fn get_calls(&self) -> Vec<(StepType, usize, usize)> {
        self.calls.read().unwrap().clone()
    }
}

impl ProgressReporter for MockProgressReporter {
    fn set_step(&self, _step_type: StepType) {
        // Mock implementation - do nothing
    }

    fn set_current_file(&self, _file: Option<String>) {
        // Mock implementation - do nothing
    }

    fn set_file_counts(
        &self,
        step_type: StepType,
        processed: usize,
        total: usize,
        _message: Option<String>,
    ) {
        self.calls
            .write()
            .unwrap()
            .push((step_type, processed, total));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_storage_creates_extractor() {
        let mock_storage = FileStorage::new();
        let _extractor = SourceFileExtractor::with_storage(mock_storage);
        // If it compiles and doesn't panic, the test passes
    }

    #[test]
    fn test_collect_with_progress_finds_supported_files() {
        let mut mock_storage = FileStorage::new();
        mock_storage.add_file("main.rs");
        mock_storage.add_file("lib.rs");
        mock_storage.add_file("readme.txt");
        mock_storage.add_file("script.py");

        let extractor = SourceFileExtractor::with_storage(mock_storage);
        let progress = MockProgressReporter::new();
        let result = extractor.collect_with_progress(Path::new("/mock"), &progress);

        assert!(result.is_ok());
        let files = result.unwrap();

        // Should find .rs and .py files but not .txt
        let rs_files: Vec<_> = files
            .iter()
            .filter(|p| p.extension().map_or(false, |e| e == "rs"))
            .collect();
        let py_files: Vec<_> = files
            .iter()
            .filter(|p| p.extension().map_or(false, |e| e == "py"))
            .collect();
        let txt_files: Vec<_> = files
            .iter()
            .filter(|p| p.extension().map_or(false, |e| e == "txt"))
            .collect();

        assert!(!rs_files.is_empty());
        assert!(!py_files.is_empty());
        assert!(txt_files.is_empty());
    }

    #[test]
    fn test_collect_with_progress_calls_progress_reporter() {
        let mut mock_storage = FileStorage::new();
        mock_storage.add_file("main.rs");

        let extractor = SourceFileExtractor::with_storage(mock_storage);
        let progress = MockProgressReporter::new();
        let result = extractor.collect_with_progress(Path::new("/mock"), &progress);

        assert!(result.is_ok());
        let calls = progress.get_calls();
        assert!(!calls.is_empty());

        // Check that all calls are for scanning step
        for (step_type, _, _) in calls {
            assert_eq!(step_type, StepType::Scanning);
        }
    }

    #[test]
    fn test_collect_with_progress_handles_empty_storage() {
        let mock_storage = FileStorage::new();

        let extractor = SourceFileExtractor::with_storage(mock_storage);
        let progress = MockProgressReporter::new();
        let result = extractor.collect_with_progress(Path::new("/mock"), &progress);

        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_collect_with_progress_filters_directories() {
        let mut mock_storage = FileStorage::new();
        mock_storage.add_file("main.rs");
        mock_storage.add_directory("src");
        mock_storage.add_directory("target");

        let extractor = SourceFileExtractor::with_storage(mock_storage);
        let progress = MockProgressReporter::new();
        let result = extractor.collect_with_progress(Path::new("/mock"), &progress);

        assert!(result.is_ok());
        let files = result.unwrap();

        // Should only contain files, not directories
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].file_name().unwrap(), "main.rs");
    }
}

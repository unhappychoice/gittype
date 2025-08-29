use std::path::Path;
use crate::game::Challenge;
use crate::{Result, GitTypeError};
use super::{CodeExtractor, ExtractionOptions, ChallengeConverter, ProgressReporter, NoOpProgressReporter};

pub struct RepositoryLoader {
    extractor: CodeExtractor,
    converter: ChallengeConverter,
}

impl RepositoryLoader {
    pub fn new() -> Result<Self> {
        let extractor = CodeExtractor::new()?;
        let converter = ChallengeConverter::new();
        
        Ok(Self {
            extractor,
            converter,
        })
    }

    pub fn load_challenges_from_repository(
        &mut self,
        repo_path: &Path,
        options: Option<ExtractionOptions>,
    ) -> Result<Vec<Challenge>> {
        self.load_challenges_from_repository_with_progress(repo_path, options, &NoOpProgressReporter)
    }

    pub fn load_challenges_from_repository_with_progress<P: ProgressReporter + ?Sized>(
        &mut self,
        repo_path: &Path,
        options: Option<ExtractionOptions>,
        progress: &P,
    ) -> Result<Vec<Challenge>> {
        if !repo_path.exists() {
            return Err(GitTypeError::RepositoryNotFound(repo_path.to_path_buf()));
        }

        let extraction_options = options.unwrap_or_default();
        let chunks = self.extractor.extract_chunks_with_progress(repo_path, extraction_options, progress)?;
        
        if chunks.is_empty() {
            return Err(GitTypeError::NoSupportedFiles);
        }

        progress.set_phase("Generating challenges".to_string());
        let challenges = self.converter.convert_chunks_to_challenges(chunks);
        
        progress.set_phase("Finalizing".to_string());
        Ok(challenges)
    }


    pub fn load_challenges_with_difficulty<P: ProgressReporter + ?Sized>(
        &mut self,
        repo_path: &Path,
        options: Option<ExtractionOptions>,
        difficulty: &crate::game::stage_builder::DifficultyLevel,
        progress: &P,
    ) -> Result<Vec<Challenge>> {
        use crate::game::stage_builder::DifficultyLevel;
        
        if !repo_path.exists() {
            return Err(GitTypeError::RepositoryNotFound(repo_path.to_path_buf()));
        }

        match difficulty {
            DifficultyLevel::Zen => {
                progress.set_phase("Loading whole files".to_string());
                let file_paths = self.collect_source_files(repo_path)?;
                let challenges = self.converter.convert_whole_files_to_challenges(file_paths);
                progress.set_phase("Finalizing".to_string());
                Ok(challenges)
            }
            _ => {
                let extraction_options = options.unwrap_or_default();
                let chunks = self.extractor.extract_chunks_with_progress(repo_path, extraction_options, progress)?;
                
                if chunks.is_empty() {
                    return Err(GitTypeError::NoSupportedFiles);
                }

                progress.set_phase("Generating challenges".to_string());
                let challenges = self.converter.convert_with_difficulty_split(chunks, difficulty);
                
                progress.set_phase("Finalizing".to_string());
                Ok(challenges)
            }
        }
    }

    fn collect_source_files(&self, repo_path: &Path) -> Result<Vec<std::path::PathBuf>> {
        use std::fs;
        
        let mut files = Vec::new();
        let extensions = vec!["rs", "ts", "tsx", "py", "js", "jsx"];
        
        fn collect_recursive(dir: &Path, extensions: &[&str], files: &mut Vec<std::path::PathBuf>) -> std::io::Result<()> {
            if dir.is_dir() {
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path.is_dir() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if !name.starts_with('.') && name != "target" && name != "node_modules" {
                                collect_recursive(&path, extensions, files)?;
                            }
                        }
                    } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if extensions.contains(&ext) {
                            files.push(path);
                        }
                    }
                }
            }
            Ok(())
        }
        
        collect_recursive(repo_path, &extensions, &mut files)
            .map_err(|e| GitTypeError::IoError(e))?;
            
        Ok(files)
    }

    pub fn load_all_files_as_zen_challenges(&mut self, repo_path: &Path) -> Result<Vec<Challenge>> {
        // Get all source files in the repository
        let all_source_files = self.collect_source_files(repo_path)?;
        
        // Create Zen challenges for each file
        let zen_challenges = self.converter.convert_whole_files_to_zen_challenges_only(all_source_files);
        
        Ok(zen_challenges)
    }
}


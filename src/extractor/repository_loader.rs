use super::{
    ChallengeConverter, CodeExtractor, ExtractionOptions, GitRepositoryExtractor, GitRepository,
    NoOpProgressReporter, ProgressReporter,
};
use crate::models::Challenge;
use crate::{GitTypeError, Result};
use std::path::Path;

pub struct RepositoryLoader {
    extractor: CodeExtractor,
    converter: ChallengeConverter,
    git_repository: Option<GitRepository>,
}

impl RepositoryLoader {
    pub fn new() -> Result<Self> {
        let extractor = CodeExtractor::new()?;
        let converter = ChallengeConverter::new();

        Ok(Self {
            extractor,
            converter,
            git_repository: None,
        })
    }

    pub fn load_challenges_from_repository(
        &mut self,
        repo_path: &Path,
        options: Option<ExtractionOptions>,
    ) -> Result<Vec<Challenge>> {
        self.load_challenges_from_repository_with_progress(
            repo_path,
            options,
            &NoOpProgressReporter,
        )
    }

    pub fn load_challenges_from_repository_with_progress<P: ProgressReporter>(
        &mut self,
        repo_path: &Path,
        options: Option<ExtractionOptions>,
        progress: &P,
    ) -> Result<Vec<Challenge>> {
        if !repo_path.exists() {
            return Err(GitTypeError::RepositoryNotFound(repo_path.to_path_buf()));
        }

        // Extract git information
        self.git_repository = GitRepositoryExtractor::extract_git_repository(repo_path)?;

        let extraction_options = options.unwrap_or_default();
        let chunks =
            self.extractor
                .extract_chunks_with_progress(repo_path, extraction_options, progress)?;

        if chunks.is_empty() {
            return Err(GitTypeError::NoSupportedFiles);
        }

        progress.set_step(crate::game::models::loading_steps::StepType::Generating);
        // Expand chunks into multiple challenges across difficulties
        let challenges = self
            .converter
            .convert_chunks_to_challenges_with_progress(chunks, progress);

        progress.set_step(crate::game::models::loading_steps::StepType::Finalizing);
        Ok(challenges)
    }

    pub fn load_challenges_with_difficulty<P: ProgressReporter>(
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
                progress.set_step(crate::game::models::loading_steps::StepType::Generating);
                let file_paths = self.collect_source_files(repo_path)?;
                let challenges = self.converter.convert_whole_files_to_challenges(file_paths);
                progress.set_step(crate::game::models::loading_steps::StepType::Finalizing);
                Ok(challenges)
            }
            _ => {
                let extraction_options = options.unwrap_or_default();
                let chunks = self.extractor.extract_chunks_with_progress(
                    repo_path,
                    extraction_options,
                    progress,
                )?;

                if chunks.is_empty() {
                    return Err(GitTypeError::NoSupportedFiles);
                }

                progress.set_step(crate::game::models::loading_steps::StepType::Generating);
                let challenges = self
                    .converter
                    .convert_with_difficulty_split(chunks, difficulty);

                progress.set_step(crate::game::models::loading_steps::StepType::Finalizing);
                Ok(challenges)
            }
        }
    }

    fn collect_source_files(&self, repo_path: &Path) -> Result<Vec<std::path::PathBuf>> {
        use std::fs;

        let mut files = Vec::new();
        let extensions = vec!["rs", "ts", "tsx", "py", "js", "jsx"];

        fn collect_recursive(
            dir: &Path,
            extensions: &[&str],
            files: &mut Vec<std::path::PathBuf>,
        ) -> std::io::Result<()> {
            if dir.is_dir() {
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();

                    if path.is_dir() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if !name.starts_with('.') && name != "target" && name != "node_modules"
                            {
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

        collect_recursive(repo_path, &extensions, &mut files).map_err(GitTypeError::IoError)?;

        Ok(files)
    }

    pub fn load_all_files_as_zen_challenges(&mut self, repo_path: &Path) -> Result<Vec<Challenge>> {
        // Get all source files in the repository
        let all_source_files = self.collect_source_files(repo_path)?;

        // Create Zen challenges for each file
        let zen_challenges = self
            .converter
            .convert_whole_files_to_zen_challenges_only(all_source_files);

        Ok(zen_challenges)
    }

    pub fn get_git_repository(&self) -> &Option<GitRepository> {
        &self.git_repository
    }
}

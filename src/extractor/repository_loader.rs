use std::path::Path;
use crate::game::Challenge;
use crate::{Result, GitTypeError};
use super::{CodeExtractor, ExtractionOptions, ChallengeConverter};

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
        if !repo_path.exists() {
            return Err(GitTypeError::RepositoryNotFound(repo_path.to_path_buf()));
        }

        let extraction_options = options.unwrap_or_default();
        let chunks = self.extractor.extract_chunks(repo_path, extraction_options)?;
        
        if chunks.is_empty() {
            return Err(GitTypeError::NoSupportedFiles);
        }

        let challenges = self.converter.convert_chunks_to_challenges(chunks);
        Ok(challenges)
    }

    pub fn load_functions_only(
        &mut self,
        repo_path: &Path,
        options: Option<ExtractionOptions>,
    ) -> Result<Vec<Challenge>> {
        if !repo_path.exists() {
            return Err(GitTypeError::RepositoryNotFound(repo_path.to_path_buf()));
        }

        let extraction_options = options.unwrap_or_default();
        let chunks = self.extractor.extract_chunks(repo_path, extraction_options)?;
        
        if chunks.is_empty() {
            return Err(GitTypeError::NoSupportedFiles);
        }

        let challenges = self.converter.convert_functions_only(chunks);
        Ok(challenges)
    }

    pub fn load_classes_only(
        &mut self,
        repo_path: &Path,
        options: Option<ExtractionOptions>,
    ) -> Result<Vec<Challenge>> {
        if !repo_path.exists() {
            return Err(GitTypeError::RepositoryNotFound(repo_path.to_path_buf()));
        }

        let extraction_options = options.unwrap_or_default();
        let chunks = self.extractor.extract_chunks(repo_path, extraction_options)?;
        
        if chunks.is_empty() {
            return Err(GitTypeError::NoSupportedFiles);
        }

        let challenges = self.converter.convert_classes_only(chunks);
        Ok(challenges)
    }
}


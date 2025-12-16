use rayon::prelude::*;
use shaku::Interface;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::domain::models::loading::StepType;
use crate::domain::models::{Challenge, DifficultyLevel, GitRepository};
use crate::infrastructure::storage::compressed_file_storage::{
    CompressedFileStorage, CompressedFileStorageInterface,
};
use crate::infrastructure::storage::file_storage::FileStorageInterface;
use crate::presentation::tui::screens::loading_screen::ProgressReporter;
use crate::Result;
use rayon::prelude::*;
use shaku::Interface;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ChallengePointer {
    id: String,
    source_file_path: Option<String>,
    start_line: Option<usize>,
    end_line: Option<usize>,
    language: Option<String>,
    comment_ranges: Vec<(usize, usize)>,
    difficulty_level: Option<DifficultyLevel>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CacheData {
    repo_key: String,
    commit_hash: String,
    challenge_pointers: Vec<ChallengePointer>,
}

pub trait ChallengeRepositoryInterface: Interface {
    fn save_challenges(
        &self,
        repo: &GitRepository,
        challenges: &[Challenge],
        reporter: Option<&dyn ProgressReporter>,
    ) -> Result<()>;

    fn load_challenges_with_progress(
        &self,
        repo: &GitRepository,
        reporter: Option<&dyn ProgressReporter>,
    ) -> Result<Option<Vec<Challenge>>>;

    fn get_cache_stats(&self) -> Result<(usize, u64)>;
    fn clear_cache(&self) -> Result<()>;
    fn invalidate_repository(&self, repo: &GitRepository) -> Result<bool>;
    fn list_cache_keys(&self) -> Result<Vec<String>>;
}

#[derive(Debug, Clone, shaku::Component)]
#[shaku(interface = ChallengeRepositoryInterface)]
pub struct ChallengeRepository {
    #[shaku(default)]
    cache_dir: PathBuf,
    #[shaku(inject)]
    storage: Arc<dyn CompressedFileStorageInterface>,
    #[shaku(inject)]
    file_storage: Arc<dyn FileStorageInterface>,
}

impl ChallengeRepository {
    pub fn save_challenges(&self, repo: &GitRepository, challenges: &[Challenge]) -> Result<()> {
        if repo.is_dirty {
            return Ok(());
        }

        let commit_str = match repo.commit_hash.as_deref() {
            Some(h) if !h.is_empty() => h,
            _ => return Ok(()),
        };

        let cache_file = self.get_cache_file(repo);

        let challenge_pointers: Vec<ChallengePointer> = challenges
            .iter()
            .map(|challenge| ChallengePointer {
                id: challenge.id.clone(),
                source_file_path: challenge.source_file_path.clone(),
                start_line: challenge.start_line,
                end_line: challenge.end_line,
                language: challenge.language.clone(),
                comment_ranges: challenge.comment_ranges.clone(),
                difficulty_level: challenge.difficulty_level,
            })
            .collect();

        let cache_data = CacheData {
            repo_key: repo.cache_key(),
            commit_hash: commit_str.to_string(),
            challenge_pointers,
        };

        let storage = (self.storage.as_ref() as &dyn std::any::Any)
            .downcast_ref::<CompressedFileStorage>()
            .ok_or_else(|| {
                crate::GitTypeError::ExtractionFailed("Failed to downcast storage".to_string())
            })?;

        storage.save(&cache_file, &cache_data)
    }

    pub fn load_challenges_with_progress(
        &self,
        repo: &GitRepository,
        progress_reporter: Option<&dyn ProgressReporter>,
    ) -> Option<Vec<Challenge>> {
        if repo.is_dirty {
            return None;
        }

        let cache_file = self.get_cache_file(repo);

        let storage = (self.storage.as_ref() as &dyn std::any::Any)
            .downcast_ref::<CompressedFileStorage>()?;

        let cache_data: CacheData = storage.load(&cache_file).ok()??;

        let current_commit = repo.commit_hash.as_deref().unwrap_or("");
        if cache_data.commit_hash != current_commit {
            return None;
        }

        let repo_root = repo.root_path.as_ref()?;
        let total = cache_data.challenge_pointers.len();
        let processed = Arc::new(Mutex::new(0usize));

        let results: Vec<Option<Challenge>> = cache_data
            .challenge_pointers
            .par_iter()
            .map(|pointer| {
                let challenge = self.reconstruct_challenge(pointer, repo_root);

                if let Some(reporter) = progress_reporter {
                    let mut count = processed.lock().unwrap();
                    *count += 1;
                    let current = *count;
                    drop(count);

                    reporter.set_file_counts(
                        StepType::CacheCheck,
                        current,
                        total,
                        Some(format!("Reconstructing challenge {}/{}", current, total)),
                    );
                }

                challenge
            })
            .collect();

        let challenges: Vec<Challenge> = results.into_iter().flatten().collect();

        if challenges.is_empty() {
            return None;
        }
        Some(challenges)
    }

    pub fn clear_cache(&self) -> Result<()> {
        let cache_dir = self.effective_cache_dir();
        let files = self.storage.list_files_in_dir(&cache_dir);
        for file in files {
            self.storage.delete_file(&file)?;
        }
        Ok(())
    }

    pub fn get_cache_stats(&self) -> Result<(usize, u64)> {
        let cache_dir = self.effective_cache_dir();
        let files = self.storage.list_files_in_dir(&cache_dir);
        let count = files.len();
        let total_size = files
            .iter()
            .filter_map(|path| self.storage.get_file_size(path))
            .sum();
        Ok((count, total_size))
    }

    pub fn invalidate_repository(&self, repo: &GitRepository) -> Result<bool> {
        let cache_file = self.get_cache_file(repo);
        if self.storage.file_exists(&cache_file) {
            self.storage.delete_file(&cache_file)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn list_cache_keys(&self) -> Result<Vec<String>> {
        let cache_dir = self.effective_cache_dir();
        let files = self.storage.list_files_in_dir(&cache_dir);

        let storage = (self.storage.as_ref() as &dyn std::any::Any)
            .downcast_ref::<CompressedFileStorage>()
            .ok_or_else(|| {
                crate::GitTypeError::ExtractionFailed("Failed to downcast storage".to_string())
            })?;

        let mut keys: Vec<String> = files
            .iter()
            .filter_map(|path| {
                if path.file_name()?.to_str()?.ends_with(".bin") {
                    storage
                        .load::<CacheData>(path)
                        .ok()
                        .flatten()
                        .map(|d| format!("{}:{}", d.repo_key, d.commit_hash))
                } else {
                    None
                }
            })
            .collect();

        keys.sort();
        keys.dedup();
        Ok(keys)
    }

    fn reconstruct_challenge(
        &self,
        pointer: &ChallengePointer,
        repo_root: &std::path::Path,
    ) -> Option<Challenge> {
        let file_path = pointer.source_file_path.as_ref()?;
        let absolute_path = repo_root.join(file_path);

        let absolute_path = absolute_path
            .canonicalize()
            .map_err(|e| {
                log::debug!("Failed to canonicalize {}: {}", file_path, e);
                e
            })
            .ok()?;

        let repo_root = repo_root.canonicalize().ok()?;
        if !absolute_path.starts_with(&repo_root) {
            log::debug!("Path security check failed: {}", file_path);
            return None;
        }

        let file_content = self
            .file_storage
            .read_to_string(&absolute_path)
            .map_err(|e| {
                log::debug!("Failed to read file {}: {}", file_path, e);
                e
            })
            .ok()?;

        let lines: Vec<&str> = file_content.lines().collect();

        let code_content = match (pointer.start_line, pointer.end_line) {
            (Some(start), Some(end)) => {
                if start <= lines.len() && end <= lines.len() && start <= end {
                    lines[start.saturating_sub(1)..end].join("\n")
                } else {
                    log::debug!(
                        "Line number mismatch in {}: start={}, end={}, file_lines={}",
                        file_path,
                        start,
                        end,
                        lines.len()
                    );
                    return None;
                }
            }
            _ => file_content,
        };

        Some(Challenge {
            id: pointer.id.clone(),
            source_file_path: pointer.source_file_path.clone(),
            code_content,
            start_line: pointer.start_line,
            end_line: pointer.end_line,
            language: pointer.language.clone(),
            comment_ranges: pointer.comment_ranges.clone(),
            difficulty_level: pointer.difficulty_level,
        })
    }

    fn effective_cache_dir(&self) -> PathBuf {
        if self.cache_dir.as_os_str().is_empty() {
            self.file_storage
                .get_app_data_dir()
                .map(|p| p.join("cache"))
                .unwrap_or_default()
        } else {
            self.cache_dir.clone()
        }
    }

    fn get_cache_file(&self, repo: &GitRepository) -> PathBuf {
        use sha2::{Digest, Sha256};

        let cache_dir = self.effective_cache_dir();
        let _ = self.file_storage.create_dir_all(&cache_dir);
        let commit = repo.commit_hash.as_deref().unwrap_or("nohash");
        let dirty = if repo.is_dirty { "dirty" } else { "clean" };
        let raw = format!("{}:{}:{}", repo.cache_key(), commit, dirty);
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        let digest = hasher.finalize();
        let hex = digest
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        cache_dir.join(format!("{}.bin", hex))
    }
}

impl ChallengeRepositoryInterface for ChallengeRepository {
    fn save_challenges(
        &self,
        repo: &GitRepository,
        challenges: &[Challenge],
        _reporter: Option<&dyn ProgressReporter>,
    ) -> Result<()> {
        ChallengeRepository::save_challenges(self, repo, challenges)
    }

    fn load_challenges_with_progress(
        &self,
        repo: &GitRepository,
        reporter: Option<&dyn ProgressReporter>,
    ) -> Result<Option<Vec<Challenge>>> {
        Ok(ChallengeRepository::load_challenges_with_progress(
            self, repo, reporter,
        ))
    }

    fn get_cache_stats(&self) -> Result<(usize, u64)> {
        ChallengeRepository::get_cache_stats(self)
    }

    fn clear_cache(&self) -> Result<()> {
        ChallengeRepository::clear_cache(self)
    }

    fn invalidate_repository(&self, repo: &GitRepository) -> Result<bool> {
        ChallengeRepository::invalidate_repository(self, repo)
    }

    fn list_cache_keys(&self) -> Result<Vec<String>> {
        ChallengeRepository::list_cache_keys(self)
    }
}

use crate::domain::models::{Challenge, DifficultyLevel, GitRepository};
use crate::infrastructure::storage::compressed_file_storage::CompressedFileStorage;
use crate::infrastructure::storage::file_storage::FileStorage;
use crate::presentation::game::models::StepType;
use crate::presentation::tui::screens::loading_screen::ProgressReporter;
use rayon::prelude::*;
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

#[derive(Debug, Clone)]
pub struct ChallengeRepository {
    cache_dir: PathBuf,
    storage: CompressedFileStorage,
    file_storage: FileStorage,
}

impl ChallengeRepository {
    pub fn new() -> Self {
        let file_storage = FileStorage::new();
        let cache_dir = file_storage
            .get_app_data_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("cache");

        Self {
            cache_dir,
            storage: CompressedFileStorage::new(),
            file_storage,
        }
    }

    pub fn with_cache_dir(cache_dir: PathBuf) -> Self {
        Self {
            cache_dir,
            storage: CompressedFileStorage::new(),
            file_storage: FileStorage::new(),
        }
    }

    pub fn save_challenges(
        &self,
        repo: &GitRepository,
        challenges: &[Challenge],
    ) -> Result<(), String> {
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

        self.storage
            .save(&cache_file, &cache_data)
            .map_err(|e| e.to_string())
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
        let cache_data: CacheData = self.storage.load(&cache_file).ok()??;

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

    pub fn clear_cache(&self) -> Result<(), String> {
        let files = self.storage.list_files_in_dir(&self.cache_dir);
        for file in files {
            self.storage.delete_file(&file).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn get_cache_stats(&self) -> Result<(usize, u64), String> {
        let files = self.storage.list_files_in_dir(&self.cache_dir);
        let count = files.len();
        let total_size = files
            .iter()
            .filter_map(|path| self.storage.get_file_size(path))
            .sum();
        Ok((count, total_size))
    }

    pub fn invalidate_repository(&self, repo: &GitRepository) -> Result<bool, String> {
        let cache_file = self.get_cache_file(repo);
        if self.storage.file_exists(&cache_file) {
            self.storage
                .delete_file(&cache_file)
                .map_err(|e| e.to_string())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn list_cache_keys(&self) -> Result<Vec<String>, String> {
        let files = self.storage.list_files_in_dir(&self.cache_dir);

        let mut keys: Vec<String> = files
            .iter()
            .filter_map(|path| {
                if path.file_name()?.to_str()?.ends_with(".bin") {
                    self.storage
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

    fn get_cache_file(&self, repo: &GitRepository) -> PathBuf {
        let _ = self.file_storage.create_dir_all(&self.cache_dir);
        let commit = repo.commit_hash.as_deref().unwrap_or("nohash");
        let dirty = if repo.is_dirty { "dirty" } else { "clean" };
        let raw = format!("{}:{}:{}", repo.cache_key(), commit, dirty);
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        let digest = hasher.finalize();
        let hex = digest
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        self.cache_dir.join(format!("{}.bin", hex))
    }
}

impl Default for ChallengeRepository {
    fn default() -> Self {
        Self::new()
    }
}

pub static CHALLENGE_REPOSITORY: once_cell::sync::Lazy<ChallengeRepository> =
    once_cell::sync::Lazy::new(ChallengeRepository::new);

use super::gzip_storage::GzipStorage;
use crate::models::{Challenge, GitRepository};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ChallengePointer {
    id: String,
    source_file_path: Option<String>,
    start_line: Option<usize>,
    end_line: Option<usize>,
    language: Option<String>,
    comment_ranges: Vec<(usize, usize)>,
    difficulty_level: Option<crate::game::stage_repository::DifficultyLevel>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CacheData {
    /// For display/CLI purposes; filename becomes opaque once hashed.
    repo_key: String,
    commit_hash: String,
    challenge_pointers: Vec<ChallengePointer>,
}

#[derive(Debug, Clone)]
pub struct ChallengeCache {
    cache_dir: PathBuf,
}

impl ChallengeCache {
    pub fn new() -> Self {
        let mut cache_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        cache_dir.push(".gittype");
        cache_dir.push("cache");

        Self { cache_dir }
    }

    pub fn with_cache_dir(cache_dir: PathBuf) -> Self {
        Self { cache_dir }
    }

    pub fn save(&self, repo: &GitRepository, challenges: &[Challenge]) -> Result<(), String> {
        // Skip caching for dirty repositories
        if repo.is_dirty {
            return Ok(());
        }

        // Do not save when the commit hash is unknown
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

        GzipStorage::save(&cache_file, &cache_data)
    }

    pub fn load_with_progress(
        &self,
        repo: &GitRepository,
        progress_reporter: Option<&dyn crate::game::screens::loading_screen::ProgressReporter>,
    ) -> Option<Vec<Challenge>> {
        use rayon::prelude::*;
        use std::sync::{Arc, Mutex};

        // Skip cache for dirty repositories
        if repo.is_dirty {
            return None;
        }

        let cache_file = self.get_cache_file(repo);
        let cache_data: CacheData = GzipStorage::load(&cache_file)?;

        // Check if commit hash matches
        let current_commit = repo.commit_hash.as_deref().unwrap_or("");
        if cache_data.commit_hash != current_commit {
            return None;
        }

        // Reconstruct challenges from pointers with parallel progress
        let repo_root = repo.root_path.as_ref()?;
        let total = cache_data.challenge_pointers.len();
        let processed = Arc::new(Mutex::new(0usize));

        let results: Vec<Option<Challenge>> = cache_data
            .challenge_pointers
            .par_iter()
            .map(|pointer| {
                let challenge = self.reconstruct_challenge(pointer, repo_root);

                // Report progress atomically
                if let Some(reporter) = progress_reporter {
                    let mut count = processed.lock().unwrap();
                    *count += 1;
                    let current = *count;
                    drop(count);

                    reporter.set_file_counts(
                        crate::game::models::loading_steps::StepType::CacheCheck,
                        current,
                        total,
                        Some(format!("Reconstructing challenge {}/{}", current, total)),
                    );
                }

                challenge
            })
            .collect();

        let challenges: Vec<Challenge> = results.into_iter().filter_map(|r| r).collect();

        if challenges.is_empty() {
            return None;
        }
        Some(challenges)
    }

    pub fn clear(&self) -> Result<(), String> {
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir)
                .map_err(|e| format!("Failed to clear cache: {}", e))?;
        }
        Ok(())
    }

    pub fn stats(&self) -> Result<(usize, u64), String> {
        if !self.cache_dir.exists() {
            return Ok((0, 0));
        }

        let mut count = 0;
        let mut total_size = 0u64;

        for entry in
            fs::read_dir(&self.cache_dir).map_err(|e| format!("Failed to read cache dir: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read cache entry: {}", e))?;
            if entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.ends_with(".bin"))
            {
                count += 1;
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                }
            }
        }

        Ok((count, total_size))
    }

    pub fn invalidate_repo(&self, repo: &GitRepository) -> Result<bool, String> {
        let cache_file = self.get_cache_file(repo);
        if cache_file.exists() {
            fs::remove_file(cache_file)
                .map_err(|e| format!("Failed to invalidate cache: {}", e))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn list_keys(&self) -> Result<Vec<String>, String> {
        if !self.cache_dir.exists() {
            return Ok(Vec::new());
        }

        let mut keys: Vec<String> = fs::read_dir(&self.cache_dir)
            .map_err(|e| format!("Failed to read cache dir: {}", e))?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                // Prefer data inside the file (repo_key + commit_hash) instead of filename.
                if entry.file_name().to_string_lossy().ends_with(".bin") {
                    GzipStorage::load::<CacheData>(&entry.path())
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

        let absolute_path = absolute_path.canonicalize().map_err(|e| {
            log::debug!("Failed to canonicalize {}: {}", file_path, e);
            e
        }).ok()?;

        let repo_root = repo_root.canonicalize().ok()?;
        if !absolute_path.starts_with(&repo_root) {
            log::debug!("Path security check failed: {}", file_path);
            return None;
        }

        // Read file content
        let file_content = fs::read_to_string(&absolute_path).map_err(|e| {
            log::debug!("Failed to read file {}: {}", file_path, e);
            e
        }).ok()?;

        let lines: Vec<&str> = file_content.lines().collect();

        // Extract code content based on line numbers
        let code_content = match (pointer.start_line, pointer.end_line) {
            (Some(start), Some(end)) => {
                if start <= lines.len() && end <= lines.len() && start <= end {
                    lines[start.saturating_sub(1)..end].join("\n")
                } else {
                    log::debug!("Line number mismatch in {}: start={}, end={}, file_lines={}",
                        file_path, start, end, lines.len());
                    return None;
                }
            }
            _ => file_content, // Fallback to entire file if no line info
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
        // Best-effort dir creation; callers handle save/load errors.
        let _ = fs::create_dir_all(&self.cache_dir);
        // Compose a stable, collision-resistant, filesystem-safe key.
        let commit = repo.commit_hash.as_deref().unwrap_or("nohash");
        let dirty = if repo.is_dirty { "dirty" } else { "clean" };
        let raw = format!("{}:{}:{}", repo.cache_key(), commit, dirty);
        // Hash to keep filename short and safe across OSes.
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

impl Default for ChallengeCache {
    fn default() -> Self {
        Self::new()
    }
}

pub static CHALLENGE_CACHE: once_cell::sync::Lazy<ChallengeCache> =
    once_cell::sync::Lazy::new(ChallengeCache::new);

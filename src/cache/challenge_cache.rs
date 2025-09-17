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
            commit_hash: repo.commit_hash.clone().unwrap_or_default(),
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
        if cache_data.commit_hash != repo.commit_hash.as_deref().unwrap_or("") {
            return None;
        }

        // Reconstruct challenges from pointers with parallel progress
        let repo_root = repo.root_path.as_ref()?;
        let total = cache_data.challenge_pointers.len();
        let processed = Arc::new(Mutex::new(0usize));

        let challenges: Vec<Challenge> = cache_data
            .challenge_pointers
            .par_iter()
            .filter_map(|pointer| {
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

        let keys = fs::read_dir(&self.cache_dir)
            .map_err(|e| format!("Failed to read cache dir: {}", e))?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .filter(|name| name.ends_with(".bin"))
                    .and_then(|name| {
                        let cache_key = name.trim_end_matches(".bin");
                        GzipStorage::load::<CacheData>(&entry.path())
                            .map(|cache_data| format!("{}:{}", cache_key, cache_data.commit_hash))
                    })
            })
            .collect();

        Ok(keys)
    }

    fn reconstruct_challenge(
        &self,
        pointer: &ChallengePointer,
        repo_root: &std::path::Path,
    ) -> Option<Challenge> {
        let file_path = pointer.source_file_path.as_ref()?;
        let absolute_path = repo_root.join(file_path);

        // Read file content
        let file_content = fs::read_to_string(&absolute_path).ok()?;
        let lines: Vec<&str> = file_content.lines().collect();

        // Extract code content based on line numbers
        let code_content = match (pointer.start_line, pointer.end_line) {
            (Some(start), Some(end)) => {
                if start <= lines.len() && end <= lines.len() && start <= end {
                    lines[start.saturating_sub(1)..end].join("\n")
                } else {
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
        fs::create_dir_all(&self.cache_dir).ok();
        let cache_key = repo.cache_key();
        self.cache_dir.join(format!("{}.bin", cache_key))
    }
}

impl Default for ChallengeCache {
    fn default() -> Self {
        Self::new()
    }
}

pub static CHALLENGE_CACHE: once_cell::sync::Lazy<ChallengeCache> =
    once_cell::sync::Lazy::new(ChallengeCache::new);

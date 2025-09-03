use crate::game::screens::loading_screen::ProgressReporter;
use crate::{GitTypeError, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RepoInfo {
    pub origin: String,
    pub owner: String,
    pub name: String,
}

pub struct RepositoryManager;

impl RepositoryManager {
    pub fn parse_repo_url(repo_spec: &str) -> Result<RepoInfo> {
        // Handle different formats:
        // 1. owner/repo (short format)
        // 2. https://github.com/owner/repo
        // 3. git@github.com:owner/repo.git

        if repo_spec.contains('@') {
            // SSH format: git@github.com:owner/repo.git
            let parts: Vec<&str> = repo_spec.split(':').collect();
            if parts.len() != 2 {
                return Err(GitTypeError::InvalidRepositoryFormat(
                    "Invalid SSH repository format".to_string(),
                ));
            }

            let host_part = parts[0];
            let repo_part = parts[1].strip_suffix(".git").unwrap_or(parts[1]);

            let origin = if host_part.contains('@') {
                host_part.split('@').nth(1).unwrap_or("github.com")
            } else {
                "github.com"
            };

            let repo_segments: Vec<&str> = repo_part.split('/').collect();
            if repo_segments.len() != 2 {
                return Err(GitTypeError::InvalidRepositoryFormat(
                    "Invalid repository path format".to_string(),
                ));
            }

            Ok(RepoInfo {
                origin: origin.to_string(),
                owner: repo_segments[0].to_string(),
                name: repo_segments[1].to_string(),
            })
        } else if repo_spec.starts_with("http") {
            // HTTPS format: https://github.com/owner/repo
            let url = repo_spec.strip_suffix(".git").unwrap_or(repo_spec);

            // Parse URL
            if let Some(path_start) = url.find("://") {
                let after_protocol = &url[path_start + 3..];
                let parts: Vec<&str> = after_protocol.split('/').collect();

                if parts.len() < 3 {
                    return Err(GitTypeError::InvalidRepositoryFormat(
                        "Invalid HTTPS repository format".to_string(),
                    ));
                }

                let origin = parts[0];
                let owner = parts[1];
                let name = parts[2];

                Ok(RepoInfo {
                    origin: origin.to_string(),
                    owner: owner.to_string(),
                    name: name.to_string(),
                })
            } else {
                Err(GitTypeError::InvalidRepositoryFormat(
                    "Invalid HTTPS URL format".to_string(),
                ))
            }
        } else if repo_spec.contains('/') && !repo_spec.contains(' ') {
            // Short format: owner/repo
            let parts: Vec<&str> = repo_spec.split('/').collect();
            if parts.len() != 2 {
                return Err(GitTypeError::InvalidRepositoryFormat(
                    "Invalid short repository format".to_string(),
                ));
            }

            Ok(RepoInfo {
                origin: "github.com".to_string(),
                owner: parts[0].to_string(),
                name: parts[1].to_string(),
            })
        } else {
            Err(GitTypeError::InvalidRepositoryFormat(format!(
                "Unsupported repository format: {}",
                repo_spec
            )))
        }
    }

    pub fn get_local_repo_path(repo_info: &RepoInfo) -> Result<PathBuf> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            GitTypeError::InvalidRepositoryFormat("Could not determine home directory".to_string())
        })?;

        let repo_path = home_dir
            .join(".gittype")
            .join("repos")
            .join(&repo_info.origin)
            .join(&repo_info.owner)
            .join(&repo_info.name);

        Ok(repo_path)
    }

    pub fn clone_or_update_repo(
        repo_info: &RepoInfo,
        loading_screen: Option<&crate::game::screens::loading_screen::LoadingScreen>,
    ) -> Result<PathBuf> {
        let local_path = Self::get_local_repo_path(repo_info)?;

        if local_path.exists() {
            // Check if the repository is complete (has source files, not just .git)
            if Self::is_repository_complete(&local_path)? {
                return Ok(local_path);
            } else {
                // Remove incomplete repository
                std::fs::remove_dir_all(&local_path)?;
            }
        }

        // Create parent directories
        if let Some(parent) = local_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Construct clone URL
        let clone_url = format!(
            "https://{}/{}/{}.git",
            repo_info.origin, repo_info.owner, repo_info.name
        );

        // Configure shallow clone to minimize download size and time
        let mut builder = git2::build::RepoBuilder::new();

        // Set up fetch options for shallow clone (depth=1)
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.depth(1); // Only fetch the latest commit (no history)

        // Configure progress callbacks for detailed progress display
        let mut remote_callbacks = git2::RemoteCallbacks::new();

        // Clone progress tracking: 3 phases
        // Phase 1: Transfer (0-60%) - downloading objects
        // Phase 2: Update refs (60-80%) - updating references
        // Phase 3: Checkout (80-100%) - checking out files

        let loading_screen_clone = loading_screen.map(|s| s as *const _);
        let mut last_progress_percent = -1.0;

        // Transfer progress callback (Phase 1: 0-60%)
        remote_callbacks.transfer_progress(move |stats| {
            let total_objects = stats.total_objects();
            let received_objects = stats.received_objects();
            let _received_bytes = stats.received_bytes();

            if total_objects > 0 {
                let transfer_progress = received_objects as f64 / total_objects as f64;
                // Map transfer progress to 0-60% of total clone progress
                let total_progress = transfer_progress * 0.6;
                let progress_percent = total_progress * 100.0;

                // Only update display if progress changed by at least 1%
                if (progress_percent - last_progress_percent).abs() >= 1.0 {
                    last_progress_percent = progress_percent;

                    if let Some(screen_ptr) = loading_screen_clone {
                        unsafe {
                            let screen: &crate::game::screens::loading_screen::LoadingScreen =
                                &*screen_ptr;
                            let current_step = (total_progress * 100.0) as usize;
                            let _ = screen.update_progress(total_progress, current_step, 100);
                        }
                    }
                }
            }

            true
        });

        // Update tips callback (Phase 2: 60-80%)
        let loading_screen_clone2 = loading_screen.map(|s| s as *const _);
        let mut ref_count = 0;
        remote_callbacks.update_tips(move |_refname, _old_oid, _new_oid| {
            ref_count += 1;

            if let Some(screen_ptr) = loading_screen_clone2 {
                unsafe {
                    let screen: &crate::game::screens::loading_screen::LoadingScreen = &*screen_ptr;
                    // Map refs progress to 60-80% of total clone progress
                    let refs_progress = 0.6 + (ref_count as f64 * 0.005).min(0.2); // Increment by 0.5% per ref, max 20%
                    let current_step = (refs_progress * 100.0) as usize;
                    let _ = screen.update_progress(refs_progress, current_step, 100);
                }
            }
            true
        });

        // Let git2 handle certificates and credentials using defaults for public repositories

        fetch_options.remote_callbacks(remote_callbacks);
        builder.fetch_options(fetch_options);

        // Checkout progress callback (Phase 3: 80-100%)
        let loading_screen_clone3 = loading_screen.map(|s| s as *const _);
        let mut checkout_builder = git2::build::CheckoutBuilder::new();
        let mut last_checkout_percent = -1.0;
        checkout_builder.progress(move |_path, completed, total| {
            if total > 0 {
                let checkout_progress = completed as f64 / total as f64;
                // Map checkout progress to 80-100% of total clone progress
                let total_progress = 0.8 + (checkout_progress * 0.2);
                let progress_percent = total_progress * 100.0;

                // Only update every 2% or on completion
                if (progress_percent - last_checkout_percent).abs() >= 2.0 || completed == total {
                    last_checkout_percent = progress_percent;

                    if let Some(screen_ptr) = loading_screen_clone3 {
                        unsafe {
                            let screen: &crate::game::screens::loading_screen::LoadingScreen =
                                &*screen_ptr;
                            let current_step = (total_progress * 100.0) as usize;
                            let _ = screen.update_progress(total_progress, current_step, 100);
                        }
                    }
                }
            }
        });

        builder.with_checkout(checkout_builder);

        // Start cloning with progress display
        if let Some(screen) = loading_screen {
            screen.set_step(crate::game::models::loading_steps::StepType::Cloning);
        }

        // Perform the clone operation
        match builder.clone(&clone_url, &local_path) {
            Ok(_) => {}
            Err(e) => {
                // If clone fails, clean up partial clone
                if local_path.exists() {
                    let _ = std::fs::remove_dir_all(&local_path);
                }
                return Err(GitTypeError::RepositoryCloneError(e));
            }
        }

        // Post-clone optimization to minimize disk usage
        Self::optimize_cloned_repo(&local_path)?;

        Ok(local_path)
    }

    /// Optimize cloned repository by removing unnecessary git objects and files
    fn optimize_cloned_repo(repo_path: &std::path::Path) -> Result<()> {
        use std::process::Command;

        // Phase 1: Git garbage collection
        let _output = Command::new("git")
            .args(["gc", "--aggressive", "--prune=now"])
            .current_dir(repo_path)
            .output();

        // Phase 2: Remove unnecessary files
        let _removed_count = Self::remove_unnecessary_files(repo_path)?;

        Ok(())
    }

    /// Remove files that are not needed for code extraction
    /// Returns the number of items removed
    fn remove_unnecessary_files(repo_path: &std::path::Path) -> Result<usize> {
        use std::fs;

        // List of file patterns that can be safely removed for gittype's purposes
        // (Currently we remove only large directories, but this list could be used
        // for more fine-grained file removal in the future)
        let _unnecessary_patterns = vec![
            "*.png",
            "*.jpg",
            "*.jpeg",
            "*.gif",
            "*.svg",
            "*.ico",
            "*.webp",
            "*.mp4",
            "*.mov",
            "*.avi",
            "*.webm",
            "*.pdf",
            "*.doc",
            "*.docx",
            "*.ppt",
            "*.pptx",
            "*.zip",
            "*.tar",
            "*.gz",
            "*.rar",
            "*.7z",
            "node_modules",
            ".git/objects/pack/*",
            ".git/logs/*",
            "target/debug",
            "target/release",
            "__pycache__",
            "*.pyc",
            "*.pyo",
            "*.class",
        ];

        // We'll implement a simple removal of common large directories
        let large_dirs = vec![
            ("node_modules", "Node.js dependencies"),
            ("target", "Rust build artifacts"),
            ("__pycache__", "Python cache"),
            (".git/logs", "Git logs"),
        ];

        let mut removed_count = 0;

        for (dir_name, _description) in large_dirs {
            let dir_path = repo_path.join(dir_name);
            if dir_path.exists() && dir_path.is_dir() {
                match fs::remove_dir_all(&dir_path) {
                    Ok(_) => {
                        removed_count += 1;
                        // Optionally show what was removed (commented out for cleaner output)
                        // eprintln!("    Removed: {} ({})", dir_name, description);
                    }
                    Err(_) => {
                        // Non-critical error, continue silently
                    }
                }
            }
        }

        Ok(removed_count)
    }

    /// Check if a cloned repository is complete (has source files, not just .git)
    fn is_repository_complete(repo_path: &std::path::Path) -> Result<bool> {
        use std::fs;

        // Check if .git directory exists
        let git_dir = repo_path.join(".git");
        if !git_dir.exists() {
            return Ok(false);
        }

        // Check if there are any source files (not just .git)
        let entries = fs::read_dir(repo_path)?;
        let mut has_source_files = false;

        for entry in entries {
            let entry = entry?;
            let file_name = entry.file_name();

            // Skip .git directory and common non-source files
            if let Some(name) = file_name.to_str() {
                if name != ".git" && name != ".gitignore" && name != ".gitattributes" {
                    has_source_files = true;
                    break;
                }
            }
        }

        Ok(has_source_files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_short_format() {
        let repo_info = RepositoryManager::parse_repo_url("owner/repo").unwrap();
        assert_eq!(repo_info.origin, "github.com");
        assert_eq!(repo_info.owner, "owner");
        assert_eq!(repo_info.name, "repo");
    }

    #[test]
    fn test_parse_https_format() {
        let repo_info = RepositoryManager::parse_repo_url("https://github.com/rust-lang/rust").unwrap();
        assert_eq!(repo_info.origin, "github.com");
        assert_eq!(repo_info.owner, "rust-lang");
        assert_eq!(repo_info.name, "rust");
    }

    #[test]
    fn test_parse_https_format_with_git_suffix() {
        let repo_info =
            RepositoryManager::parse_repo_url("https://github.com/microsoft/vscode.git").unwrap();
        assert_eq!(repo_info.origin, "github.com");
        assert_eq!(repo_info.owner, "microsoft");
        assert_eq!(repo_info.name, "vscode");
    }

    #[test]
    fn test_parse_ssh_format() {
        let repo_info =
            RepositoryManager::parse_repo_url("git@github.com:unhappychoice/gittype.git").unwrap();
        assert_eq!(repo_info.origin, "github.com");
        assert_eq!(repo_info.owner, "unhappychoice");
        assert_eq!(repo_info.name, "gittype");
    }
}

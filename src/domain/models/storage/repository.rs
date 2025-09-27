use crate::infrastructure::git::git_repository_ref_parser::GitRepositoryRefParser;

#[derive(Debug, Clone)]
pub struct StoredRepository {
    pub id: i64,
    pub user_name: String,
    pub repository_name: String,
    pub remote_url: String,
}

#[derive(Debug, Clone)]
pub struct StoredRepositoryWithLanguages {
    pub id: i64,
    pub user_name: String,
    pub repository_name: String,
    pub remote_url: String,
    pub languages: Vec<String>,
    // TODO: Add is_cached property and merge it in domain/repository logic
}

impl StoredRepositoryWithLanguages {
    // TODO: Remove this and add http_url property 
    pub fn http_url(&self) -> String {
        GitRepositoryRefParser::parse(&self.remote_url)
            .map(|repo_ref| repo_ref.http_url())
            .unwrap_or_else(|_| self.remote_url.clone())
    }
}

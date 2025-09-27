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
}

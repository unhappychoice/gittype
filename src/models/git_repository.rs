use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitRepository {
    pub user_name: String,
    pub repository_name: String,
    pub remote_url: String,
    pub branch: Option<String>,
    pub commit_hash: Option<String>,
    pub is_dirty: bool,
}
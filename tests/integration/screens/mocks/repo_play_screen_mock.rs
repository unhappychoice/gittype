use gittype::domain::models::storage::StoredRepositoryWithLanguages;
use gittype::presentation::game::models::ScreenDataProvider;
use gittype::Result;

pub struct MockRepoPlayDataProvider;

impl ScreenDataProvider for MockRepoPlayDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let repositories = vec![
            StoredRepositoryWithLanguages {
                id: 1,
                user_name: "unhappychoice".to_string(),
                repository_name: "gittype".to_string(),
                remote_url: "https://github.com/unhappychoice/gittype".to_string(),
                languages: vec!["Rust".to_string(), "Shell".to_string()],
            },
            StoredRepositoryWithLanguages {
                id: 2,
                user_name: "rails".to_string(),
                repository_name: "rails".to_string(),
                remote_url: "https://github.com/rails/rails".to_string(),
                languages: vec!["Ruby".to_string(), "HTML".to_string(), "JavaScript".to_string()],
            },
            StoredRepositoryWithLanguages {
                id: 3,
                user_name: "golang".to_string(),
                repository_name: "go".to_string(),
                remote_url: "https://github.com/golang/go".to_string(),
                languages: vec!["Go".to_string(), "Assembly".to_string()],
            },
        ];

        Ok(Box::new(repositories))
    }
}

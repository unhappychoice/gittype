use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

use crate::domain::error::{GitTypeError, Result};
use crate::infrastructure::cache::TrendingRepository;

#[derive(Debug, Clone)]
pub struct OssInsightClient {
    client: Client,
}

impl OssInsightClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_trending_repositories(
        &self,
        language: Option<&str>,
        period: &str,
    ) -> Result<Vec<TrendingRepository>> {
        let api_period = match period {
            "daily" => "past_24_hours",
            "weekly" => "past_week",
            "monthly" => "past_month",
            _ => "past_24_hours",
        };

        let mut url = format!(
            "https://api.ossinsight.io/v1/trends/repos/?period={}",
            api_period
        );

        if let Some(lang) = language {
            let api_lang = self.map_language_name(lang);
            url = format!("{}&language={}", url, urlencoding::encode(&api_lang));
        }

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "gittype")
            .header("Accept", "application/json")
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GitTypeError::ApiError(format!(
                "OSS Insight API request failed: {}",
                response.status()
            )));
        }

        let api_response: ApiResponse = response.json().await?;
        let repositories = self.convert_api_response(api_response);

        Ok(repositories)
    }

    fn map_language_name(&self, lang: &str) -> String {
        match lang.to_lowercase().as_str() {
            "javascript" => "JavaScript".to_string(),
            "typescript" => "TypeScript".to_string(),
            "c++" => "C++".to_string(),
            "c#" => "C#".to_string(),
            "php" => "PHP".to_string(),
            _ => {
                let mut chars = lang.chars();
                match chars.next() {
                    None => lang.to_string(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            }
        }
    }

    fn convert_api_response(&self, api_response: ApiResponse) -> Vec<TrendingRepository> {
        api_response
            .data
            .rows
            .into_iter()
            .map(|row| TrendingRepository {
                repo_name: row.repo_name,
                primary_language: row.primary_language,
                description: row.description,
                stars: row.stars.unwrap_or_else(|| "0".to_string()),
                forks: row.forks.unwrap_or_else(|| "0".to_string()),
                total_score: row.total_score.unwrap_or_else(|| "0".to_string()),
            })
            .collect()
    }
}

impl Default for OssInsightClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize)]
struct ApiResponse {
    data: ApiData,
}

#[derive(Deserialize)]
struct ApiData {
    rows: Vec<RowData>,
}

#[derive(Deserialize)]
struct RowData {
    repo_name: String,
    primary_language: Option<String>,
    description: Option<String>,
    stars: Option<String>,
    forks: Option<String>,
    total_score: Option<String>,
}

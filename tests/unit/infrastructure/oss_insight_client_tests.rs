use gittype::infrastructure::http::oss_insight_client::{
    OssInsightClient, OssInsightClientInterface,
};

#[tokio::test]
async fn fetch_trending_repositories_returns_empty_mock_data() {
    let client = OssInsightClient::new();

    let repositories = client
        .fetch_trending_repositories(Some("rust"), "weekly")
        .await
        .unwrap();

    assert!(repositories.is_empty());
}

#[tokio::test]
async fn trait_fetch_trending_repositories_uses_mock_data() {
    let client = OssInsightClient::new();

    let repositories =
        OssInsightClientInterface::fetch_trending_repositories(&client, None, "daily")
            .await
            .unwrap();

    assert!(repositories.is_empty());
}

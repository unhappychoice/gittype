use gittype::presentation::game::models::ScreenDataProvider;
use gittype::Result;

pub struct MockTrendingRepositorySelectionDataProvider;

impl ScreenDataProvider for MockTrendingRepositorySelectionDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        // (language, period)
        let data = (Some("Rust".to_string()), "daily".to_string());
        Ok(Box::new(data))
    }
}

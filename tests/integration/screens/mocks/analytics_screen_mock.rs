use gittype::application::service::analytics_service::{AnalyticsData, LangStats, RepoStats};
use gittype::presentation::tui::ScreenDataProvider;
use gittype::Result;
use std::collections::HashMap;

pub struct MockAnalyticsDataProvider;

impl ScreenDataProvider for MockAnalyticsDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let mut repository_stats = HashMap::new();
        repository_stats.insert(
            "test/repo1".to_string(),
            RepoStats {
                avg_cpm: 350.0,
                avg_wpm: 70.0,
                avg_accuracy: 95.0,
                total_sessions: 10,
                total_keystrokes: 1000,
                total_mistakes: 50,
                total_duration_ms: 60000,
                avg_score: 500.0,
                best_cpm: 400.0,
                best_accuracy: 98.0,
                stages_completed: 8,
                stages_attempted: 10,
                stages_skipped: 0,
            },
        );

        let mut language_stats = HashMap::new();
        language_stats.insert(
            "Rust".to_string(),
            LangStats {
                avg_cpm: 340.0,
                avg_wpm: 68.0,
                avg_accuracy: 94.0,
                total_sessions: 5,
                total_keystrokes: 500,
                total_mistakes: 30,
                total_duration_ms: 30000,
                avg_score: 480.0,
                best_cpm: 380.0,
                best_accuracy: 97.0,
                stages_completed: 15,
                stages_attempted: 20,
                stages_skipped: 0,
            },
        );

        let data = AnalyticsData {
            total_sessions: 10,
            avg_cpm: 350.0,
            avg_accuracy: 95.0,
            total_time_hours: 1.0,
            cpm_trend: vec![
                ("Day 1".to_string(), 300.0),
                ("Day 2".to_string(), 350.0),
                ("Day 3".to_string(), 400.0),
            ],
            accuracy_trend: vec![
                ("Day 1".to_string(), 90.0),
                ("Day 2".to_string(), 95.0),
                ("Day 3".to_string(), 97.0),
            ],
            top_repositories: vec![
                ("test/repo1".to_string(), 350.0),
                ("test/repo2".to_string(), 300.0),
            ],
            top_languages: vec![
                ("Rust".to_string(), 340.0, 20),
                ("Python".to_string(), 300.0, 15),
            ],
            daily_sessions: HashMap::new(),
            best_cpm: 400.0,
            total_mistakes: 50,
            avg_session_duration: 6.0,
            current_streak: 3,
            repository_stats,
            language_stats,
        };

        Ok(Box::new(data))
    }
}

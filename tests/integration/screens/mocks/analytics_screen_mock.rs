use gittype::domain::services::analytics_service::{AnalyticsData, LangStats, RepoStats};
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
            reference_date: None,
        };

        Ok(Box::new(data))
    }
}

/// Provider with daily session data to test chart rendering
pub struct MockAnalyticsDataProviderWithActivity;

impl ScreenDataProvider for MockAnalyticsDataProviderWithActivity {
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

        // Add daily session data for the chart
        let mut daily_sessions = HashMap::new();
        daily_sessions.insert("01-15".to_string(), 2);
        daily_sessions.insert("01-16".to_string(), 5);
        daily_sessions.insert("01-17".to_string(), 3);
        daily_sessions.insert("01-18".to_string(), 7);
        daily_sessions.insert("01-19".to_string(), 4);
        daily_sessions.insert("01-20".to_string(), 6);
        daily_sessions.insert("01-21".to_string(), 8);

        use chrono::NaiveDate;
        let reference_date = NaiveDate::from_ymd_opt(2025, 1, 22);

        let data = AnalyticsData {
            total_sessions: 35,
            avg_cpm: 350.0,
            avg_accuracy: 95.0,
            total_time_hours: 3.5,
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
                (
                    "test/very-long-repository-name-that-should-be-truncated".to_string(),
                    300.0,
                ),
            ],
            top_languages: vec![
                ("Rust".to_string(), 340.0, 20),
                (
                    "VeryLongLanguageNameThatShouldBeTruncated".to_string(),
                    300.0,
                    15,
                ),
            ],
            daily_sessions,
            best_cpm: 400.0,
            total_mistakes: 50,
            avg_session_duration: 6.0,
            current_streak: 7,
            repository_stats,
            language_stats,
            reference_date,
        };

        Ok(Box::new(data))
    }
}

/// Provider with empty data to test empty state rendering
pub struct MockAnalyticsDataProviderEmpty;

impl ScreenDataProvider for MockAnalyticsDataProviderEmpty {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let data = AnalyticsData {
            total_sessions: 0,
            avg_cpm: 0.0,
            avg_accuracy: 0.0,
            total_time_hours: 0.0,
            cpm_trend: vec![],
            accuracy_trend: vec![],
            top_repositories: vec![],
            top_languages: vec![],
            daily_sessions: HashMap::new(),
            best_cpm: 0.0,
            total_mistakes: 0,
            avg_session_duration: 0.0,
            current_streak: 0,
            repository_stats: HashMap::new(),
            language_stats: HashMap::new(),
            reference_date: None,
        };

        Ok(Box::new(data))
    }
}

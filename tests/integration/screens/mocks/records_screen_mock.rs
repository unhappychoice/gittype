use chrono::{TimeZone, Utc};
use gittype::domain::models::storage::{SessionResultData, StoredRepository, StoredSession};
use gittype::presentation::game::models::ScreenDataProvider;
use gittype::presentation::game::screens::records_screen::RecordsScreenData;
use gittype::presentation::game::screens::session_detail_screen::SessionDisplayData;
use gittype::Result;

pub struct MockRecordsDataProvider;

impl ScreenDataProvider for MockRecordsDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let repositories = vec![
            StoredRepository {
                id: 1,
                user_name: "unhappychoice".to_string(),
                repository_name: "gittype".to_string(),
                remote_url: "https://github.com/unhappychoice/gittype".to_string(),
            },
            StoredRepository {
                id: 2,
                user_name: "rails".to_string(),
                repository_name: "rails".to_string(),
                remote_url: "https://github.com/rails/rails".to_string(),
            },
        ];

        let sessions = vec![
            SessionDisplayData {
                session: StoredSession {
                    id: 1,
                    repository_id: Some(1),
                    started_at: Utc.with_ymd_and_hms(2024, 10, 7, 12, 30, 0).unwrap(),
                    completed_at: Some(Utc.with_ymd_and_hms(2024, 10, 7, 12, 31, 0).unwrap()),
                    branch: Some("main".to_string()),
                    commit_hash: Some("abc123".to_string()),
                    is_dirty: false,
                    game_mode: "default".to_string(),
                    difficulty_level: Some("Normal".to_string()),
                    max_stages: Some(3),
                    time_limit_seconds: None,
                },
                repository: Some(repositories[0].clone()),
                session_result: Some(SessionResultData {
                    keystrokes: 500,
                    mistakes: 20,
                    duration_ms: 60000,
                    wpm: 75.0,
                    cpm: 375.0,
                    accuracy: 96.0,
                    stages_completed: 3,
                    stages_attempted: 3,
                    stages_skipped: 0,
                    score: 1200.0,
                    rank_name: Some("Advanced".to_string()),
                    tier_name: Some("Gold".to_string()),
                    rank_position: Some(5),
                    rank_total: Some(100),
                    position: Some(5),
                    total: Some(100),
                }),
            },
            SessionDisplayData {
                session: StoredSession {
                    id: 2,
                    repository_id: Some(2),
                    started_at: Utc.with_ymd_and_hms(2024, 10, 6, 15, 20, 0).unwrap(),
                    completed_at: Some(Utc.with_ymd_and_hms(2024, 10, 6, 15, 22, 0).unwrap()),
                    branch: Some("main".to_string()),
                    commit_hash: Some("def456".to_string()),
                    is_dirty: false,
                    game_mode: "default".to_string(),
                    difficulty_level: Some("Hard".to_string()),
                    max_stages: Some(3),
                    time_limit_seconds: None,
                },
                repository: Some(repositories[1].clone()),
                session_result: Some(SessionResultData {
                    keystrokes: 650,
                    mistakes: 35,
                    duration_ms: 120000,
                    wpm: 65.0,
                    cpm: 325.0,
                    accuracy: 94.6,
                    stages_completed: 3,
                    stages_attempted: 3,
                    stages_skipped: 0,
                    score: 980.0,
                    rank_name: Some("Intermediate".to_string()),
                    tier_name: Some("Silver".to_string()),
                    rank_position: Some(15),
                    rank_total: Some(100),
                    position: Some(15),
                    total: Some(100),
                }),
            },
            SessionDisplayData {
                session: StoredSession {
                    id: 3,
                    repository_id: Some(1),
                    started_at: Utc.with_ymd_and_hms(2024, 10, 5, 9, 10, 0).unwrap(),
                    completed_at: Some(Utc.with_ymd_and_hms(2024, 10, 5, 9, 11, 0).unwrap()),
                    branch: Some("develop".to_string()),
                    commit_hash: Some("ghi789".to_string()),
                    is_dirty: false,
                    game_mode: "default".to_string(),
                    difficulty_level: Some("Easy".to_string()),
                    max_stages: Some(3),
                    time_limit_seconds: None,
                },
                repository: Some(repositories[0].clone()),
                session_result: Some(SessionResultData {
                    keystrokes: 400,
                    mistakes: 15,
                    duration_ms: 50000,
                    wpm: 80.0,
                    cpm: 400.0,
                    accuracy: 96.2,
                    stages_completed: 3,
                    stages_attempted: 3,
                    stages_skipped: 0,
                    score: 1300.0,
                    rank_name: Some("Expert".to_string()),
                    tier_name: Some("Platinum".to_string()),
                    rank_position: Some(3),
                    rank_total: Some(100),
                    position: Some(3),
                    total: Some(100),
                }),
            },
        ];

        let data = RecordsScreenData {
            sessions,
            repositories,
        };

        Ok(Box::new(data))
    }
}

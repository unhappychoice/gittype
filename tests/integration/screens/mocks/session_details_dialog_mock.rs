use gittype::domain::models::storage::SessionResultData;
use gittype::domain::models::{GitRepository, SessionResult, StageResult};
use gittype::domain::repositories::session_repository::{BestRecords, BestStatus};
use gittype::presentation::tui::ScreenDataProvider;
use gittype::presentation::tui::screens::session_details_dialog::SessionDetailsDialogData;
use gittype::Result;
use std::time::{Duration, Instant};

pub struct MockSessionDetailsDialogDataProvider;

impl ScreenDataProvider for MockSessionDetailsDialogDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let repo_info = Some(GitRepository {
            user_name: "unhappychoice".to_string(),
            repository_name: "gittype".to_string(),
            remote_url: "https://github.com/unhappychoice/gittype".to_string(),
            branch: Some("main".to_string()),
            commit_hash: Some("abc123def456".to_string()),
            is_dirty: false,
            root_path: None,
        });

        let stage_results = vec![
            StageResult {
                cpm: 350.0,
                wpm: 70.0,
                accuracy: 96.7,
                keystrokes: 150,
                mistakes: 5,
                consistency_streaks: vec![5, 8, 10],
                completion_time: Duration::from_millis(18000),
                challenge_score: 380.0,
                rank_name: "Beginner".to_string(),
                tier_name: "Bronze".to_string(),
                tier_position: 10,
                tier_total: 100,
                overall_position: 10,
                overall_total: 100,
                was_skipped: false,
                was_failed: false,
                challenge_path: "src/main.rs".to_string(),
            },
            StageResult {
                cpm: 375.0,
                wpm: 75.0,
                accuracy: 96.0,
                keystrokes: 200,
                mistakes: 8,
                consistency_streaks: vec![6, 9, 12],
                completion_time: Duration::from_millis(22000),
                challenge_score: 420.0,
                rank_name: "Intermediate".to_string(),
                tier_name: "Silver".to_string(),
                tier_position: 8,
                tier_total: 100,
                overall_position: 8,
                overall_total: 100,
                was_skipped: false,
                was_failed: false,
                challenge_path: "src/lib.rs".to_string(),
            },
            StageResult {
                cpm: 400.0,
                wpm: 80.0,
                accuracy: 95.3,
                keystrokes: 150,
                mistakes: 7,
                consistency_streaks: vec![7, 11, 15],
                completion_time: Duration::from_millis(20000),
                challenge_score: 400.0,
                rank_name: "Advanced".to_string(),
                tier_name: "Gold".to_string(),
                tier_position: 5,
                tier_total: 100,
                overall_position: 5,
                overall_total: 100,
                was_skipped: false,
                was_failed: false,
                challenge_path: "src/utils.rs".to_string(),
            },
        ];

        let session_result = Some(SessionResult {
            session_start_time: Instant::now(),
            session_duration: Duration::from_millis(60000),
            valid_session_duration: Duration::from_millis(60000),
            invalid_session_duration: Duration::from_millis(0),
            stages_completed: 3,
            stages_attempted: 3,
            stages_skipped: 0,
            stage_results,
            overall_accuracy: 96.0,
            overall_wpm: 75.0,
            overall_cpm: 375.0,
            valid_keystrokes: 500,
            valid_mistakes: 20,
            invalid_keystrokes: 0,
            invalid_mistakes: 0,
            best_stage_wpm: 80.0,
            worst_stage_wpm: 70.0,
            best_stage_accuracy: 96.7,
            worst_stage_accuracy: 95.3,
            session_score: 1200.0,
            session_successful: true,
        });

        let best_status = Some(BestStatus {
            is_todays_best: true,
            is_weekly_best: false,
            is_all_time_best: false,
            best_type: Some("today".to_string()),
            todays_best_score: 1200.0,
            weekly_best_score: 1500.0,
            all_time_best_score: 1800.0,
        });

        let best_records = Some(BestRecords {
            todays_best: Some(SessionResultData {
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
            weekly_best: Some(SessionResultData {
                keystrokes: 600,
                mistakes: 15,
                duration_ms: 55000,
                wpm: 85.0,
                cpm: 425.0,
                accuracy: 97.5,
                stages_completed: 3,
                stages_attempted: 3,
                stages_skipped: 0,
                score: 1500.0,
                rank_name: Some("Expert".to_string()),
                tier_name: Some("Platinum".to_string()),
                rank_position: Some(2),
                rank_total: Some(100),
                position: Some(2),
                total: Some(100),
            }),
            all_time_best: Some(SessionResultData {
                keystrokes: 700,
                mistakes: 10,
                duration_ms: 50000,
                wpm: 95.0,
                cpm: 475.0,
                accuracy: 98.5,
                stages_completed: 3,
                stages_attempted: 3,
                stages_skipped: 0,
                score: 1800.0,
                rank_name: Some("Master".to_string()),
                tier_name: Some("Diamond".to_string()),
                rank_position: Some(1),
                rank_total: Some(100),
                position: Some(1),
                total: Some(100),
            }),
        });

        let data = SessionDetailsDialogData {
            session_result,
            repo_info,
            best_status,
            best_records,
        };

        Ok(Box::new(data))
    }
}

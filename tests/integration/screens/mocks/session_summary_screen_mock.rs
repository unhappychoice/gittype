use gittype::domain::models::{GitRepository, SessionResult};
use gittype::presentation::tui::screens::session_summary_screen::SessionSummaryScreenData;
use gittype::presentation::tui::ScreenDataProvider;
use gittype::Result;
use std::time::{Duration, Instant};

pub struct MockSessionSummaryDataProvider;

impl ScreenDataProvider for MockSessionSummaryDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let session_result = Some(SessionResult {
            session_start_time: Instant::now(),
            session_duration: Duration::from_secs(180),
            valid_session_duration: Duration::from_secs(150),
            invalid_session_duration: Duration::from_secs(30),
            stages_completed: 5,
            stages_attempted: 5,
            stages_skipped: 0,
            stage_results: vec![],
            overall_accuracy: 94.5,
            overall_wpm: 52.0,
            overall_cpm: 260.0,
            valid_keystrokes: 780,
            valid_mistakes: 30,
            invalid_keystrokes: 15,
            invalid_mistakes: 15,
            best_stage_wpm: 60.0,
            worst_stage_wpm: 45.0,
            best_stage_accuracy: 98.0,
            worst_stage_accuracy: 90.0,
            session_score: 9500.0,
            session_successful: true,
        });

        let git_repository = Some(GitRepository {
            user_name: "unhappychoice".to_string(),
            repository_name: "gittype".to_string(),
            remote_url: "https://github.com/unhappychoice/gittype.git".to_string(),
            branch: Some("main".to_string()),
            commit_hash: Some("def456".to_string()),
            is_dirty: false,
            root_path: None,
        });

        let data = SessionSummaryScreenData {
            session_result,
            git_repository,
        };
        Ok(Box::new(data))
    }
}

pub struct MockLoadBalancerPrimarchDataProvider;

impl ScreenDataProvider for MockLoadBalancerPrimarchDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let session_result = Some(SessionResult {
            session_start_time: Instant::now(),
            session_duration: Duration::from_secs(180),
            valid_session_duration: Duration::from_secs(150),
            invalid_session_duration: Duration::from_secs(30),
            stages_completed: 5,
            stages_attempted: 5,
            stages_skipped: 0,
            stage_results: vec![],
            overall_accuracy: 99.5,
            overall_wpm: 120.0,
            overall_cpm: 600.0,
            valid_keystrokes: 1800,
            valid_mistakes: 10,
            invalid_keystrokes: 5,
            invalid_mistakes: 5,
            best_stage_wpm: 130.0,
            worst_stage_wpm: 110.0,
            best_stage_accuracy: 100.0,
            worst_stage_accuracy: 98.0,
            session_score: 13000.0, // Load Balancer Primarch range: 12801-13400
            session_successful: true,
        });

        let git_repository = Some(GitRepository {
            user_name: "unhappychoice".to_string(),
            repository_name: "gittype".to_string(),
            remote_url: "https://github.com/unhappychoice/gittype.git".to_string(),
            branch: Some("main".to_string()),
            commit_hash: Some("def456".to_string()),
            is_dirty: false,
            root_path: None,
        });

        let data = SessionSummaryScreenData {
            session_result,
            git_repository,
        };
        Ok(Box::new(data))
    }
}

pub struct MockCompilerDataProvider;

impl ScreenDataProvider for MockCompilerDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let session_result = Some(SessionResult {
            session_start_time: Instant::now(),
            session_duration: Duration::from_secs(180),
            valid_session_duration: Duration::from_secs(150),
            invalid_session_duration: Duration::from_secs(30),
            stages_completed: 5,
            stages_attempted: 5,
            stages_skipped: 0,
            stage_results: vec![],
            overall_accuracy: 96.0,
            overall_wpm: 80.0,
            overall_cpm: 400.0,
            valid_keystrokes: 1200,
            valid_mistakes: 40,
            invalid_keystrokes: 10,
            invalid_mistakes: 10,
            best_stage_wpm: 90.0,
            worst_stage_wpm: 70.0,
            best_stage_accuracy: 98.0,
            worst_stage_accuracy: 94.0,
            session_score: 9600.0, // Compiler range: 9501-9800
            session_successful: true,
        });

        let git_repository = Some(GitRepository {
            user_name: "unhappychoice".to_string(),
            repository_name: "gittype".to_string(),
            remote_url: "https://github.com/unhappychoice/gittype.git".to_string(),
            branch: Some("main".to_string()),
            commit_hash: Some("def456".to_string()),
            is_dirty: false,
            root_path: None,
        });

        let data = SessionSummaryScreenData {
            session_result,
            git_repository,
        };
        Ok(Box::new(data))
    }
}

use chrono::{DateTime, Utc};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedData {
    pub repositories: Vec<SeedRepository>,
    pub sessions: Vec<SeedSession>,
    pub challenges: Vec<SeedChallenge>,
    pub stages: Vec<SeedStage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedRepository {
    pub id: i64,
    pub user_name: String,
    pub repository_name: String,
    pub remote_url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedSession {
    pub id: i64,
    pub repository_id: i64,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub branch: String,
    pub commit_hash: String,
    pub is_dirty: bool,
    pub game_mode: String,
    pub difficulty_level: String,
    pub session_result: Option<SeedSessionResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedSessionResult {
    pub keystrokes: i32,
    pub mistakes: i32,
    pub duration_ms: i64,
    pub wpm: f64,
    pub cpm: f64,
    pub accuracy: f64,
    pub stages_completed: i32,
    pub stages_attempted: i32,
    pub stages_skipped: i32,
    pub partial_effort_keystrokes: i32,
    pub partial_effort_mistakes: i32,
    pub best_stage_wpm: Option<f64>,
    pub worst_stage_wpm: Option<f64>,
    pub best_stage_accuracy: Option<f64>,
    pub worst_stage_accuracy: Option<f64>,
    pub score: f64,
    pub rank_name: Option<String>,
    pub tier_name: Option<String>,
    pub rank_position: Option<i32>,
    pub rank_total: Option<i32>,
    pub position: Option<i32>,
    pub total: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedChallenge {
    pub id: String,
    pub file_path: String,
    pub start_line: i32,
    pub end_line: i32,
    pub language: String,
    pub code_content: String,
    pub comment_ranges: String,
    pub difficulty_level: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedStage {
    pub id: i64,
    pub session_id: i64,
    pub challenge_id: String,
    pub stage_number: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl SeedData {
    #[allow(dead_code)]
    pub fn default_seed_data() -> Self {
        Self::generate_seed_data(10, 1000, 3000)
    }

    pub fn generate_seed_data(repo_count: usize, session_count: usize, stage_count: usize) -> Self {
        let mut rng = rand::rng();

        let now = Utc::now();

        let repositories = Self::generate_repositories(repo_count, &mut rng, now);
        let (sessions, challenges) =
            Self::generate_sessions_and_challenges(session_count, &repositories, &mut rng, now);
        let stages = Self::generate_stages(stage_count, &sessions, &challenges, &mut rng, now);

        Self {
            repositories,
            sessions,
            challenges,
            stages,
        }
    }

    fn generate_repositories(
        count: usize,
        rng: &mut impl Rng,
        now: DateTime<Utc>,
    ) -> Vec<SeedRepository> {
        let languages = [
            "rust",
            "typescript",
            "javascript",
            "python",
            "ruby",
            "go",
            "swift",
            "kotlin",
            "java",
            "php",
            "csharp",
            "cpp",
        ];
        let project_types = [
            "web-app",
            "cli-tool",
            "library",
            "api",
            "mobile-app",
            "desktop-app",
            "game",
            "service",
        ];
        let org_names = [
            "awesome", "modern", "super", "next-gen", "fast", "secure", "smart", "micro",
        ];

        (1..=count)
            .map(|i| {
                let lang = languages.choose(rng).unwrap();
                let project_type = project_types.choose(rng).unwrap();
                let org = org_names.choose(rng).unwrap();
                let created_days_ago = rng.random_range(1..=365);

                SeedRepository {
                    id: i as i64,
                    user_name: org.to_string(),
                    repository_name: format!(
                        "{}-{}-{}",
                        lang,
                        project_type,
                        rng.random_range(1..=999)
                    ),
                    remote_url: format!(
                        "https://github.com/{}/{}-{}-{}",
                        org,
                        lang,
                        project_type,
                        rng.random_range(1..=999)
                    ),
                    created_at: now - chrono::Duration::days(created_days_ago),
                }
            })
            .collect()
    }

    fn generate_sessions_and_challenges(
        session_count: usize,
        repositories: &[SeedRepository],
        rng: &mut impl Rng,
        now: DateTime<Utc>,
    ) -> (Vec<SeedSession>, Vec<SeedChallenge>) {
        let languages = [
            "rust",
            "typescript",
            "javascript",
            "python",
            "ruby",
            "go",
            "swift",
            "kotlin",
            "java",
            "php",
        ];
        let game_modes = ["Normal", "Time Attack", "Practice", "Challenge"];
        let difficulty_levels = ["Easy", "Medium", "Hard"];
        let branch_names = [
            "main",
            "master",
            "develop",
            "feature/new-ui",
            "fix/bug-123",
            "refactor/cleanup",
        ];

        let mut sessions = Vec::new();
        let mut challenges = Vec::new();
        let mut challenge_id_counter = 1;

        for i in 1..=session_count {
            let repo = repositories.choose(rng).unwrap();
            let started_days_ago = rng.random_range(0..=30);
            let started_at = now
                - chrono::Duration::days(started_days_ago)
                - chrono::Duration::hours(rng.random_range(0..=23))
                - chrono::Duration::minutes(rng.random_range(0..=59));

            let is_completed = rng.random_bool(0.85); // 85% completed sessions
            let completed_at = if is_completed {
                Some(started_at + chrono::Duration::minutes(rng.random_range(5..=45)))
            } else {
                None
            };

            let game_mode = game_modes.choose(rng).unwrap().to_string();
            let difficulty = difficulty_levels.choose(rng).unwrap().to_string();

            let session_result = if is_completed {
                let stages_attempted = rng.random_range(5..=20);
                let stages_completed = rng.random_range(3..=stages_attempted);
                let stages_skipped = stages_attempted - stages_completed;
                let keystrokes = rng.random_range(800..=2500);
                let mistakes = rng.random_range(10..=keystrokes / 15);
                let duration_ms = rng.random_range(300000..=2700000); // 5-45 minutes
                let accuracy = (keystrokes - mistakes) as f64 / keystrokes as f64 * 100.0;
                let wpm = (keystrokes as f64 / 5.0) / (duration_ms as f64 / 60000.0);
                let cpm = keystrokes as f64 / (duration_ms as f64 / 60000.0);

                Some(SeedSessionResult {
                    keystrokes,
                    mistakes,
                    duration_ms,
                    wpm,
                    cpm,
                    accuracy,
                    stages_completed,
                    stages_attempted,
                    stages_skipped,
                    partial_effort_keystrokes: rng.random_range(0..=100),
                    partial_effort_mistakes: rng.random_range(0..=10),
                    best_stage_wpm: Some(wpm * rng.random_range(1.1..=1.4)),
                    worst_stage_wpm: Some(wpm * rng.random_range(0.6..=0.9)),
                    best_stage_accuracy: Some(accuracy * rng.random_range(1.01..=1.05)),
                    worst_stage_accuracy: Some(accuracy * rng.random_range(0.85..=0.95)),
                    score: wpm * accuracy / 10.0 + stages_completed as f64 * 50.0,
                    rank_name: Some(Self::get_rank_name(wpm)),
                    tier_name: Some(Self::get_tier_name(accuracy)),
                    rank_position: Some(rng.random_range(1..=100)),
                    rank_total: Some(100),
                    position: Some(rng.random_range(1..=500)),
                    total: Some(500),
                })
            } else {
                None
            };

            // Generate 2-5 challenges per session
            let challenge_count = rng.random_range(2..=5);
            for _j in 0..challenge_count {
                let lang = languages.choose(rng).unwrap();
                let file_extensions = match *lang {
                    "rust" => vec!["rs"],
                    "typescript" => vec!["ts", "tsx"],
                    "javascript" => vec!["js", "jsx"],
                    "python" => vec!["py"],
                    "ruby" => vec!["rb"],
                    "go" => vec!["go"],
                    "swift" => vec!["swift"],
                    "kotlin" => vec!["kt"],
                    "java" => vec!["java"],
                    "php" => vec!["php"],
                    _ => vec!["txt"],
                };

                let ext = file_extensions.choose(rng).unwrap();
                let file_path = format!(
                    "src/{}/{}.{}",
                    Self::random_directory(rng),
                    Self::random_filename(lang, rng),
                    ext
                );

                challenges.push(SeedChallenge {
                    id: format!("challenge_{}", challenge_id_counter),
                    file_path,
                    start_line: rng.random_range(1..=50),
                    end_line: rng.random_range(51..=200),
                    language: lang.to_string(),
                    code_content: Self::generate_code_content(lang, rng),
                    comment_ranges: "[]".to_string(),
                    difficulty_level: difficulty_levels.choose(rng).unwrap().to_string(),
                    created_at: started_at - chrono::Duration::minutes(rng.random_range(1..=30)),
                });
                challenge_id_counter += 1;
            }

            sessions.push(SeedSession {
                id: i as i64,
                repository_id: repo.id,
                started_at,
                completed_at,
                branch: branch_names.choose(rng).unwrap().to_string(),
                commit_hash: Self::generate_commit_hash(rng),
                is_dirty: rng.random_bool(0.3), // 30% dirty
                game_mode,
                difficulty_level: difficulty,
                session_result,
            });
        }

        (sessions, challenges)
    }

    fn generate_stages(
        target_count: usize,
        sessions: &[SeedSession],
        challenges: &[SeedChallenge],
        rng: &mut impl Rng,
        _now: DateTime<Utc>,
    ) -> Vec<SeedStage> {
        let mut stages = Vec::new();
        let mut stage_id = 1;

        let stages_per_session = target_count / sessions.len().max(1);

        for session in sessions {
            let session_stages = if session.session_result.is_some() {
                rng.random_range(stages_per_session.saturating_sub(2)..=stages_per_session + 2)
            } else {
                rng.random_range(1..=3) // Incomplete sessions have fewer stages
            };

            for stage_num in 1..=session_stages {
                let challenge = challenges.choose(rng).unwrap();
                let stage_duration_mins = rng.random_range(1..=8);

                let started_at = session.started_at
                    + chrono::Duration::minutes((stage_num - 1) as i64 * stage_duration_mins);
                let completed_at = if session.session_result.is_some() && rng.random_bool(0.9) {
                    Some(started_at + chrono::Duration::minutes(stage_duration_mins))
                } else {
                    None
                };

                stages.push(SeedStage {
                    id: stage_id,
                    session_id: session.id,
                    challenge_id: challenge.id.clone(),
                    stage_number: stage_num as i32,
                    started_at: Some(started_at),
                    completed_at,
                });

                stage_id += 1;

                if stages.len() >= target_count {
                    return stages;
                }
            }
        }

        stages
    }

    fn get_rank_name(wpm: f64) -> String {
        match wpm as i32 {
            0..=30 => "Beginner",
            31..=50 => "Intermediate",
            51..=70 => "Advanced",
            71..=90 => "Expert",
            _ => "Master",
        }
        .to_string()
    }

    fn get_tier_name(accuracy: f64) -> String {
        match accuracy as i32 {
            0..=80 => "Bronze",
            81..=90 => "Silver",
            91..=95 => "Gold",
            96..=98 => "Platinum",
            _ => "Diamond",
        }
        .to_string()
    }

    fn random_directory(rng: &mut impl Rng) -> String {
        let dirs = [
            "components",
            "utils",
            "models",
            "services",
            "controllers",
            "handlers",
            "lib",
            "core",
            "api",
        ];
        dirs.choose(rng).unwrap().to_string()
    }

    fn random_filename(lang: &str, rng: &mut impl Rng) -> String {
        let names = match lang {
            "rust" => vec![
                "main", "lib", "parser", "handler", "config", "utils", "client", "server",
            ],
            "typescript" => vec![
                "component",
                "service",
                "model",
                "utils",
                "api",
                "types",
                "hooks",
                "store",
            ],
            "javascript" => vec![
                "app",
                "component",
                "utils",
                "api",
                "service",
                "helper",
                "config",
                "index",
            ],
            "python" => vec![
                "main", "models", "views", "utils", "service", "client", "parser", "config",
            ],
            "java" => vec![
                "Main",
                "Service",
                "Controller",
                "Model",
                "Utils",
                "Client",
                "Handler",
                "Config",
            ],
            _ => vec![
                "main", "utils", "service", "model", "config", "helper", "client", "handler",
            ],
        };
        names.choose(rng).unwrap().to_string()
    }

    fn generate_commit_hash(rng: &mut impl Rng) -> String {
        const CHARS: &[u8] = b"0123456789abcdef";
        (0..12)
            .map(|_| {
                let idx = rng.random_range(0..CHARS.len());
                CHARS[idx] as char
            })
            .collect()
    }

    fn generate_code_content(lang: &str, rng: &mut impl Rng) -> String {
        let samples = match lang {
            "rust" => vec![
                r#"pub fn calculate_fibonacci(n: usize) -> u64 {
    if n <= 1 { return n as u64; }
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}"#,
                r#"impl<T: Clone> Vec<T> {
    pub fn duplicate_elements(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.len() * 2);
        for item in self {
            result.push(item.clone());
            result.push(item.clone());
        }
        result
    }
}"#,
                r#"use std::collections::HashMap;

pub struct Cache<K, V> {
    data: HashMap<K, V>,
    capacity: usize,
}

impl<K, V> Cache<K, V> 
where 
    K: std::hash::Hash + Eq,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            data: HashMap::new(),
            capacity,
        }
    }
}"#,
            ],
            "typescript" => vec![
                r#"interface UserProfile {
    id: string;
    email: string;
    name: string;
    avatar?: string;
    preferences: {
        theme: 'light_original' | 'dark_original';
        notifications: boolean;
    };
}

export const createUserProfile = (data: Partial<UserProfile>): UserProfile => {
    return {
        id: crypto.randomUUID(),
        email: data.email || '',
        name: data.name || '',
        preferences: {
            theme: 'light_original',
            notifications: true,
            ...data.preferences
        },
        ...data
    };
};"#,
                r#"class ApiClient {
    private baseUrl: string;
    private headers: Record<string, string>;

    constructor(baseUrl: string, apiKey?: string) {
        this.baseUrl = baseUrl;
        this.headers = {
            'Content-Type': 'application/json',
            ...(apiKey && { 'Authorization': `Bearer ${apiKey}` })
        };
    }

    async get<T>(endpoint: string): Promise<T> {
        const response = await fetch(`${this.baseUrl}${endpoint}`, {
            method: 'GET',
            headers: this.headers
        });
        return response.json();
    }
}"#,
            ],
            "python" => vec![
                r#"from typing import List, Optional, Dict, Any
import json
from datetime import datetime

class DataProcessor:
    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.processed_count = 0
    
    def process_batch(self, items: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        results = []
        for item in items:
            processed = self._process_single_item(item)
            if processed:
                results.append(processed)
                self.processed_count += 1
        return results
    
    def _process_single_item(self, item: Dict[str, Any]) -> Optional[Dict[str, Any]]:
        if not self._validate_item(item):
            return None
        
        return {
            'id': item.get('id'),
            'processed_at': datetime.now().isoformat(),
            'data': self._transform_data(item.get('data', {}))
        }"#,
            ],
            _ => vec![
                r#"function processData(input) {
    const results = [];
    for (let i = 0; i < input.length; i++) {
        const item = input[i];
        if (validateItem(item)) {
            results.push(transformItem(item));
        }
    }
    return results;
}"#,
            ],
        };

        samples.choose(rng).unwrap().to_string()
    }

    // Keep the old small dataset method for testing
    #[allow(dead_code)]
    pub fn small_seed_data() -> Self {
        let now = Utc::now();
        let hour_ago = now - chrono::Duration::hours(1);
        let day_ago = now - chrono::Duration::days(1);
        let week_ago = now - chrono::Duration::weeks(1);

        Self {
            repositories: vec![
                SeedRepository {
                    id: 1,
                    user_name: "example".to_string(),
                    repository_name: "sample-rust-project".to_string(),
                    remote_url: "https://github.com/example/sample-rust-project".to_string(),
                    created_at: day_ago,
                },
                SeedRepository {
                    id: 2,
                    user_name: "example".to_string(),
                    repository_name: "sample-typescript-app".to_string(),
                    remote_url: "https://github.com/example/sample-typescript-app".to_string(),
                    created_at: week_ago,
                },
                SeedRepository {
                    id: 3,
                    user_name: "local".to_string(),
                    repository_name: "python-project".to_string(),
                    remote_url: "https://github.com/local/python-project".to_string(),
                    created_at: hour_ago,
                },
            ],
            sessions: vec![
                SeedSession {
                    id: 1,
                    repository_id: 1,
                    started_at: day_ago,
                    completed_at: Some(day_ago + chrono::Duration::minutes(15)),
                    branch: "main".to_string(),
                    commit_hash: "abc123def456".to_string(),
                    is_dirty: false,
                    game_mode: "Normal".to_string(),
                    difficulty_level: "Medium".to_string(),
                    session_result: Some(SeedSessionResult {
                        keystrokes: 1250,
                        mistakes: 42,
                        duration_ms: 900000,
                        wpm: 65.8,
                        cpm: 329.0,
                        accuracy: 96.6,
                        stages_completed: 8,
                        stages_attempted: 10,
                        stages_skipped: 2,
                        partial_effort_keystrokes: 50,
                        partial_effort_mistakes: 3,
                        best_stage_wpm: Some(78.2),
                        worst_stage_wpm: Some(52.1),
                        best_stage_accuracy: Some(98.9),
                        worst_stage_accuracy: Some(92.3),
                        score: 1840.5,
                        rank_name: Some("Advanced".to_string()),
                        tier_name: Some("Silver".to_string()),
                        rank_position: Some(15),
                        rank_total: Some(100),
                        position: Some(15),
                        total: Some(100),
                    }),
                },
                SeedSession {
                    id: 2,
                    repository_id: 2,
                    started_at: week_ago,
                    completed_at: Some(week_ago + chrono::Duration::minutes(22)),
                    branch: "feature/new-ui".to_string(),
                    commit_hash: "xyz789abc012".to_string(),
                    is_dirty: true,
                    game_mode: "Time Attack".to_string(),
                    difficulty_level: "Hard".to_string(),
                    session_result: Some(SeedSessionResult {
                        keystrokes: 1800,
                        mistakes: 95,
                        duration_ms: 1320000,
                        wpm: 52.3,
                        cpm: 261.5,
                        accuracy: 94.7,
                        stages_completed: 12,
                        stages_attempted: 15,
                        stages_skipped: 3,
                        partial_effort_keystrokes: 120,
                        partial_effort_mistakes: 8,
                        best_stage_wpm: Some(68.4),
                        worst_stage_wpm: Some(41.2),
                        best_stage_accuracy: Some(97.1),
                        worst_stage_accuracy: Some(89.5),
                        score: 2150.8,
                        rank_name: Some("Intermediate".to_string()),
                        tier_name: Some("Bronze".to_string()),
                        rank_position: Some(35),
                        rank_total: Some(100),
                        position: Some(35),
                        total: Some(100),
                    }),
                },
                SeedSession {
                    id: 3,
                    repository_id: 1,
                    started_at: hour_ago,
                    completed_at: None,
                    branch: "main".to_string(),
                    commit_hash: "def456ghi789".to_string(),
                    is_dirty: false,
                    game_mode: "Practice".to_string(),
                    difficulty_level: "Easy".to_string(),
                    session_result: None,
                },
            ],
            challenges: vec![
                SeedChallenge {
                    id: "challenge_1".to_string(),
                    file_path: "src/main.rs".to_string(),
                    start_line: 1,
                    end_line: 6,
                    language: "rust".to_string(),
                    code_content: r#"fn main() {
    println!("Hello, world!");
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}"#
                    .to_string(),
                    comment_ranges: "[]".to_string(),
                    difficulty_level: "Easy".to_string(),
                    created_at: day_ago,
                },
                SeedChallenge {
                    id: "challenge_2".to_string(),
                    file_path: "src/lib.rs".to_string(),
                    start_line: 1,
                    end_line: 20,
                    language: "rust".to_string(),
                    code_content: r#"pub struct Calculator {
    result: f64,
}

impl Calculator {
    pub fn new() -> Self {
        Self { result: 0.0 }
    }
    
    pub fn add(&mut self, value: f64) -> &mut Self {
        self.result += value;
        self
    }
    
    pub fn multiply(&mut self, value: f64) -> &mut Self {
        self.result *= value;
        self
    }
    
    pub fn get_result(&self) -> f64 {
        self.result
    }
}"#
                    .to_string(),
                    comment_ranges: "[]".to_string(),
                    difficulty_level: "Medium".to_string(),
                    created_at: day_ago,
                },
                SeedChallenge {
                    id: "challenge_3".to_string(),
                    file_path: "src/components/Button.tsx".to_string(),
                    start_line: 1,
                    end_line: 20,
                    language: "typescript".to_string(),
                    code_content: r#"interface ButtonProps {
    label: string;
    onClick: () => void;
    variant?: 'primary' | 'secondary';
    disabled?: boolean;
}

export const Button: React.FC<ButtonProps> = ({
    label,
    onClick,
    variant = 'primary',
    disabled = false
}) => {
    return (
        <button 
            className={`btn btn-${variant}`}
            onClick={onClick}
            disabled={disabled}
        >
            {label}
        </button>
    );
};"#
                    .to_string(),
                    comment_ranges: "[]".to_string(),
                    difficulty_level: "Hard".to_string(),
                    created_at: week_ago,
                },
            ],
            stages: vec![
                SeedStage {
                    id: 1,
                    session_id: 1,
                    challenge_id: "challenge_1".to_string(),
                    stage_number: 1,
                    started_at: Some(day_ago),
                    completed_at: Some(day_ago + chrono::Duration::minutes(2)),
                },
                SeedStage {
                    id: 2,
                    session_id: 1,
                    challenge_id: "challenge_2".to_string(),
                    stage_number: 2,
                    started_at: Some(day_ago + chrono::Duration::minutes(3)),
                    completed_at: Some(day_ago + chrono::Duration::minutes(6)),
                },
                SeedStage {
                    id: 3,
                    session_id: 2,
                    challenge_id: "challenge_3".to_string(),
                    stage_number: 1,
                    started_at: Some(week_ago),
                    completed_at: Some(week_ago + chrono::Duration::minutes(5)),
                },
                SeedStage {
                    id: 4,
                    session_id: 2,
                    challenge_id: "challenge_1".to_string(),
                    stage_number: 2,
                    started_at: Some(week_ago + chrono::Duration::minutes(6)),
                    completed_at: Some(week_ago + chrono::Duration::minutes(8)),
                },
                SeedStage {
                    id: 5,
                    session_id: 3,
                    challenge_id: "challenge_2".to_string(),
                    stage_number: 1,
                    started_at: Some(hour_ago),
                    completed_at: None,
                },
            ],
        }
    }
}

use super::data::SeedData;
use gittype::infrastructure::database::database::{Database, DatabaseInterface, HasDatabase};
use gittype::Result;
use std::sync::Arc;

pub struct DatabaseSeeder {
    database: Arc<dyn DatabaseInterface>,
}

impl DatabaseSeeder {
    pub fn new(database: Arc<dyn DatabaseInterface>) -> Self {
        Self { database }
    }

    #[allow(dead_code)]
    pub fn seed(&self) -> Result<()> {
        let seed_data = SeedData::default_seed_data();
        self.seed_data(&seed_data)
    }

    pub fn seed_with_counts(
        &self,
        repo_count: usize,
        session_count: usize,
        stage_count: usize,
    ) -> Result<()> {
        let seed_data = SeedData::generate_seed_data(repo_count, session_count, stage_count);
        self.seed_data(&seed_data)
    }

    fn seed_data(&self, seed_data: &SeedData) -> Result<()> {
        println!("🗃️  Seeding repositories...");
        self.seed_repositories(seed_data)?;

        println!("📊 Seeding sessions...");
        self.seed_sessions(seed_data)?;

        println!("🎯 Seeding challenges...");
        self.seed_challenges(seed_data)?;

        println!("🎮 Seeding stages...");
        self.seed_stages(seed_data)?;

        Ok(())
    }

    fn seed_repositories(&self, seed_data: &SeedData) -> Result<()> {
        let conn = self.database.get_connection()?;

        for repo in &seed_data.repositories {
            conn.execute(
                "INSERT INTO repositories (id, user_name, repository_name, remote_url, created_at)
                 VALUES (?, ?, ?, ?, ?)",
                [
                    &repo.id as &dyn rusqlite::ToSql,
                    &repo.user_name,
                    &repo.repository_name,
                    &repo.remote_url,
                    &repo.created_at.to_rfc3339(),
                ],
            )?;
        }
        println!("  ✅ {} repositories seeded", seed_data.repositories.len());
        Ok(())
    }

    fn seed_sessions(&self, seed_data: &SeedData) -> Result<()> {
        let conn = self.database.get_connection()?;

        for session in &seed_data.sessions {
            conn.execute(
                "INSERT INTO sessions (id, repository_id, started_at, completed_at, branch, commit_hash, is_dirty, game_mode, difficulty_level)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                [
                    &session.id as &dyn rusqlite::ToSql,
                    &session.repository_id,
                    &session.started_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                    &session.completed_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                    &session.branch,
                    &session.commit_hash,
                    &session.is_dirty,
                    &session.game_mode,
                    &session.difficulty_level,
                ],
            )?;

            if let Some(result) = &session.session_result {
                conn.execute(
                    "INSERT INTO session_results (session_id, repository_id, keystrokes, mistakes, duration_ms, wpm, cpm, accuracy, stages_completed, stages_attempted, stages_skipped, partial_effort_keystrokes, partial_effort_mistakes, best_stage_wpm, worst_stage_wpm, best_stage_accuracy, worst_stage_accuracy, score, rank_name, tier_name, rank_position, rank_total, position, total, game_mode, difficulty_level)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                    [
                        &session.id as &dyn rusqlite::ToSql,
                        &session.repository_id,
                        &result.keystrokes,
                        &result.mistakes,
                        &result.duration_ms,
                        &result.wpm,
                        &result.cpm,
                        &result.accuracy,
                        &result.stages_completed,
                        &result.stages_attempted,
                        &result.stages_skipped,
                        &result.partial_effort_keystrokes,
                        &result.partial_effort_mistakes,
                        &result.best_stage_wpm,
                        &result.worst_stage_wpm,
                        &result.best_stage_accuracy,
                        &result.worst_stage_accuracy,
                        &result.score,
                        &result.rank_name,
                        &result.tier_name,
                        &result.rank_position,
                        &result.rank_total,
                        &result.position,
                        &result.total,
                        &session.game_mode,
                        &session.difficulty_level,
                    ],
                )?;
            }
        }
        println!("  ✅ {} sessions seeded", seed_data.sessions.len());
        Ok(())
    }

    fn seed_challenges(&self, seed_data: &SeedData) -> Result<()> {
        let conn = self.database.get_connection()?;

        for challenge in &seed_data.challenges {
            conn.execute(
                "INSERT INTO challenges (id, file_path, start_line, end_line, language, code_content, comment_ranges, difficulty_level, created_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                [
                    &challenge.id as &dyn rusqlite::ToSql,
                    &challenge.file_path,
                    &challenge.start_line,
                    &challenge.end_line,
                    &challenge.language,
                    &challenge.code_content,
                    &challenge.comment_ranges,
                    &challenge.difficulty_level,
                    &challenge.created_at.to_rfc3339(),
                ],
            )?;
        }
        println!("  ✅ {} challenges seeded", seed_data.challenges.len());
        Ok(())
    }

    fn seed_stages(&self, seed_data: &SeedData) -> Result<()> {
        let conn = self.database.get_connection()?;

        for stage in &seed_data.stages {
            conn.execute(
                "INSERT INTO stages (id, session_id, challenge_id, stage_number, started_at, completed_at)
                 VALUES (?, ?, ?, ?, ?, ?)",
                [
                    &stage.id as &dyn rusqlite::ToSql,
                    &stage.session_id,
                    &stage.challenge_id,
                    &stage.stage_number,
                    &stage.started_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                    &stage.completed_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                ],
            )?;

            // Add stage_result if stage is completed
            if let Some(completed_at) = stage.completed_at {
                // Find the session to get repository_id
                let session = seed_data
                    .sessions
                    .iter()
                    .find(|s| s.id == stage.session_id)
                    .expect("Session not found for stage");

                // Find the challenge to get language and difficulty
                let challenge = seed_data
                    .challenges
                    .iter()
                    .find(|c| c.id == stage.challenge_id)
                    .expect("Challenge not found for stage");

                // Generate realistic stage result data
                use rand::RngExt;
                let mut rng = rand::rng();
                let keystrokes = rng.random_range(20..200);
                let mistakes = rng.random_range(0..10);
                let duration_ms = rng.random_range(5000..60000);
                let wpm = (keystrokes as f64 * 60000.0 / duration_ms as f64) / 5.0; // 5 chars per word
                let cpm = keystrokes as f64 * 60000.0 / duration_ms as f64;
                let accuracy = if keystrokes > 0 {
                    ((keystrokes - mistakes) as f64 / keystrokes as f64 * 100.0).max(0.0)
                } else {
                    100.0
                };
                let score = wpm * accuracy / 10.0;

                // Generate ranking data
                let rank_position = rng.random_range(1..=100);
                let rank_total = 100;
                let position = rng.random_range(1..=500);
                let total = 500;

                // Generate rank and tier names
                let rank_name = if wpm > 80.0 {
                    "Expert"
                } else if wpm > 60.0 {
                    "Advanced"
                } else if wpm > 40.0 {
                    "Intermediate"
                } else {
                    "Beginner"
                };
                let tier_name = if accuracy > 95.0 {
                    "Gold"
                } else if accuracy > 90.0 {
                    "Silver"
                } else {
                    "Bronze"
                };

                conn.execute(
                    "INSERT INTO stage_results (stage_id, session_id, repository_id, keystrokes, mistakes, duration_ms, wpm, cpm, accuracy, consistency_streaks, score, rank_name, tier_name, rank_position, rank_total, position, total, was_skipped, was_failed, completed_at, language, difficulty_level)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                    [
                        &stage.id as &dyn rusqlite::ToSql,
                        &stage.session_id,
                        &session.repository_id,
                        &keystrokes,
                        &mistakes,
                        &duration_ms,
                        &wpm,
                        &cpm,
                        &accuracy,
                        &"[5, 3, 8, 2]", // consistency_streaks JSON
                        &score,
                        &rank_name,
                        &tier_name,
                        &rank_position,
                        &rank_total,
                        &position,
                        &total,
                        &false, // was_skipped
                        &false, // was_failed
                        &completed_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                        &challenge.language,
                        &challenge.difficulty_level,
                    ],
                )?;
            }
        }
        println!("  ✅ {} stages seeded", seed_data.stages.len());
        Ok(())
    }
}

impl HasDatabase for DatabaseSeeder {
    fn database(&self) -> &Arc<dyn DatabaseInterface> {
        &self.database
    }
}

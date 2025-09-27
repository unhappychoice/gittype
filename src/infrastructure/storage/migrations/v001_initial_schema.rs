use super::Migration;
use crate::Result;
use rusqlite::Connection;

pub struct InitialSchema;

impl Migration for InitialSchema {
    fn version(&self) -> i32 {
        1
    }

    fn description(&self) -> &str {
        "Create initial normalized database schema with repositories, sessions, session_results, stages, and stage_results tables"
    }

    fn up(&self, conn: &Connection) -> Result<()> {
        self.create_repositories_table(conn)?;
        self.create_sessions_table(conn)?;
        self.create_challenges_table(conn)?;
        self.create_stages_table(conn)?;
        self.create_session_results_table(conn)?;
        self.create_stage_results_table(conn)?;
        self.create_indexes(conn)?;
        Ok(())
    }
}

impl InitialSchema {
    fn create_repositories_table(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS repositories (
                id INTEGER PRIMARY KEY,
                user_name TEXT NOT NULL,
                repository_name TEXT NOT NULL,
                remote_url TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(user_name, repository_name)
            )",
            [],
        )?;
        Ok(())
    }

    fn create_sessions_table(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY,
                repository_id INTEGER,
                started_at DATETIME NOT NULL,
                completed_at DATETIME,
                branch TEXT,
                commit_hash TEXT,
                is_dirty BOOLEAN DEFAULT FALSE,
                game_mode TEXT NOT NULL,
                difficulty_level TEXT,
                max_stages INTEGER,
                time_limit_seconds INTEGER,
                FOREIGN KEY (repository_id) REFERENCES repositories (id)
            )",
            [],
        )?;
        Ok(())
    }

    fn create_session_results_table(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS session_results (
                id INTEGER PRIMARY KEY,
                session_id INTEGER NOT NULL,
                repository_id INTEGER NOT NULL,      -- Denormalized for aggregation
                keystrokes INTEGER NOT NULL,
                mistakes INTEGER NOT NULL,
                duration_ms INTEGER NOT NULL,
                wpm REAL,
                cpm REAL,
                accuracy REAL,
                stages_completed INTEGER NOT NULL,
                stages_attempted INTEGER NOT NULL,
                stages_skipped INTEGER NOT NULL,
                partial_effort_keystrokes INTEGER DEFAULT 0,
                partial_effort_mistakes INTEGER DEFAULT 0,
                best_stage_wpm REAL,
                worst_stage_wpm REAL,
                best_stage_accuracy REAL,
                worst_stage_accuracy REAL,
                score REAL,
                rank_name TEXT,
                tier_name TEXT,
                rank_position INTEGER,
                rank_total INTEGER,
                position INTEGER,
                total INTEGER,
                game_mode TEXT,                       -- Denormalized from sessions.game_mode
                difficulty_level TEXT,                -- Denormalized from sessions.difficulty_level  
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (session_id) REFERENCES sessions (id),
                FOREIGN KEY (repository_id) REFERENCES repositories (id)
            )",
            [],
        )?;
        Ok(())
    }

    fn create_challenges_table(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS challenges (
                id TEXT PRIMARY KEY,
                file_path TEXT,
                start_line INTEGER,
                end_line INTEGER,
                language TEXT,
                code_content TEXT NOT NULL,
                comment_ranges TEXT, -- JSON array of [start, end] ranges
                difficulty_level TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        Ok(())
    }

    fn create_stages_table(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS stages (
                id INTEGER PRIMARY KEY,
                session_id INTEGER NOT NULL,
                challenge_id TEXT NOT NULL,
                stage_number INTEGER NOT NULL,
                started_at DATETIME,
                completed_at DATETIME,
                FOREIGN KEY (session_id) REFERENCES sessions (id),
                FOREIGN KEY (challenge_id) REFERENCES challenges (id)
            )",
            [],
        )?;
        Ok(())
    }

    fn create_stage_results_table(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS stage_results (
                id INTEGER PRIMARY KEY,
                stage_id INTEGER NOT NULL,
                session_id INTEGER NOT NULL,        -- Denormalized for aggregation
                repository_id INTEGER NOT NULL,     -- Denormalized for aggregation
                keystrokes INTEGER NOT NULL,
                mistakes INTEGER NOT NULL,
                duration_ms INTEGER NOT NULL,
                wpm REAL,
                cpm REAL,
                accuracy REAL,
                consistency_streaks TEXT, -- JSON array of streak lengths
                score REAL,
                rank_name TEXT,
                tier_name TEXT,
                rank_position INTEGER,
                rank_total INTEGER,
                position INTEGER,
                total INTEGER,
                was_skipped BOOLEAN DEFAULT FALSE,
                was_failed BOOLEAN DEFAULT FALSE,
                completed_at DATETIME NOT NULL,
                language TEXT,                       -- Denormalized from challenges.language
                difficulty_level TEXT,               -- Denormalized from challenges.difficulty_level
                FOREIGN KEY (stage_id) REFERENCES stages (id),
                FOREIGN KEY (session_id) REFERENCES sessions (id),
                FOREIGN KEY (repository_id) REFERENCES repositories (id)
            )",
            [],
        )?;
        Ok(())
    }

    fn create_indexes(&self, conn: &Connection) -> Result<()> {
        // Indexes for aggregation queries
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_stage_results_repo_date 
             ON stage_results(repository_id, completed_at)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_stage_results_language 
             ON stage_results(language)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_stage_results_session 
             ON stage_results(session_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_session_results_repo_date 
             ON session_results(repository_id, created_at)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_sessions_repo_date 
             ON sessions(repository_id, started_at)",
            [],
        )?;

        Ok(())
    }
}

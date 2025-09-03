use crate::models::{StageResult, RankingTitle, Rank};
use crate::Result;
use std::ops::Add;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Keystroke {
    pub character: char,
    pub position: usize,
    pub is_correct: bool,
    pub timestamp: Instant,
}

#[derive(Clone)]
pub struct ScoringEngine {
    start_time: Option<Instant>,
    keystrokes: Vec<Keystroke>,
    target_text: String,
    current_streak: usize,
    streaks: Vec<usize>,
    recorded_duration: Option<std::time::Duration>,
    paused_time: Option<Instant>,
    total_paused_duration: std::time::Duration,
}

impl ScoringEngine {
    pub fn new(target_text: String) -> Self {
        Self {
            start_time: None,
            keystrokes: Vec::new(),
            target_text,
            current_streak: 0,
            streaks: Vec::new(),
            recorded_duration: None,
            paused_time: None,
            total_paused_duration: std::time::Duration::ZERO,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn pause(&mut self) {
        if self.paused_time.is_none() {
            self.paused_time = Some(Instant::now());
        }
    }

    pub fn resume(&mut self) {
        if let Some(paused_time) = self.paused_time {
            self.total_paused_duration += paused_time.elapsed();
            self.paused_time = None;
        }
    }

    pub fn finish(&mut self) {
        // Make sure we're not paused when finishing
        self.resume();
        if let Some(start) = self.start_time {
            self.recorded_duration = Some(start.elapsed() - self.total_paused_duration);
        }
    }

    pub fn is_finished(&self) -> bool {
        self.recorded_duration.is_some()
    }

    pub fn get_elapsed_time(&self) -> std::time::Duration {
        if let Some(start) = self.start_time {
            let total_elapsed = start.elapsed();
            let paused_duration = if let Some(paused_time) = self.paused_time {
                self.total_paused_duration + paused_time.elapsed()
            } else {
                self.total_paused_duration
            };
            total_elapsed - paused_duration
        } else {
            std::time::Duration::ZERO
        }
    }

    pub fn get_current_streak(&self) -> usize {
        self.current_streak
    }

    pub fn record_keystroke(&mut self, ch: char, position: usize) {
        // Don't accept keystrokes after finish() has been called
        if self.is_finished() {
            return;
        }

        let is_correct = if position < self.target_text.len() {
            self.target_text.chars().nth(position).unwrap_or('\0') == ch
        } else {
            false
        };

        let keystroke = Keystroke {
            character: ch,
            position,
            is_correct,
            timestamp: Instant::now(),
        };

        self.keystrokes.push(keystroke);

        if is_correct {
            self.current_streak += 1;
        } else if self.current_streak > 0 {
            self.streaks.push(self.current_streak);
            self.current_streak = 0;
        }
    }

    /// Get elapsed time since start (use recorded duration if available)
    pub fn elapsed(&self) -> std::time::Duration {
        self.recorded_duration
            .unwrap_or_else(|| self.start_time.map(|t| t.elapsed()).unwrap_or_default())
    }

    /// Calculate current CPM (Characters Per Minute) - primary metric
    pub fn cpm(&self) -> f64 {
        if self.keystrokes.is_empty() {
            return 0.1; // Minimum to prevent 0 score
        }

        let correct_chars = self.keystrokes.iter().filter(|k| k.is_correct).count() as f64;
        let elapsed_secs = self.elapsed().as_secs_f64().max(0.1);
        (correct_chars / elapsed_secs) * 60.0
    }

    /// Calculate current WPM (Words Per Minute) - derived from CPM
    pub fn wpm(&self) -> f64 {
        self.cpm() / 5.0
    }

    /// Calculate current accuracy
    pub fn accuracy(&self) -> f64 {
        if self.keystrokes.is_empty() {
            return 0.0;
        }

        let correct_chars = self.keystrokes.iter().filter(|k| k.is_correct).count();
        (correct_chars as f64 / self.keystrokes.len() as f64) * 100.0
    }

    /// Get current mistake count
    pub fn mistakes(&self) -> usize {
        self.keystrokes.iter().filter(|k| !k.is_correct).count()
    }

    /// Get total characters typed
    pub fn total_chars(&self) -> usize {
        self.keystrokes.len()
    }

    /// Get total correct characters typed
    pub fn correct_chars(&self) -> usize {
        self.keystrokes.iter().filter(|k| k.is_correct).count()
    }

    /// Get all streaks including current
    pub fn all_streaks(&self) -> Vec<usize> {
        let mut all_streaks = self.streaks.clone();
        if self.current_streak > 0 {
            all_streaks.push(self.current_streak);
        }
        all_streaks
    }

    /// Get ranking title for current engine state
    pub fn get_ranking_title(&self) -> RankingTitle {
        let score = self.calculate_challenge_score();
        Self::get_ranking_title_for_score(score)
    }

    /// Legacy method that returns title name as string for backward compatibility
    pub fn get_ranking_title_string(&self) -> String {
        self.get_ranking_title().name().to_string()
    }

    /// Get ranking title for a specific score (pure function for testing)
    pub fn get_ranking_title_for_score(score: f64) -> RankingTitle {
        RankingTitle::for_score(score)
    }

    /// Calculate tier position and total for a given score
    pub fn calculate_tier_info(score: f64) -> (String, usize, usize, usize, usize) {
        let all_titles = RankingTitle::all_titles();
        let current_title = Self::get_ranking_title_for_score(score);

        // Find titles in the same tier
        let same_tier_titles: Vec<_> = all_titles
            .iter()
            .filter(|title| title.tier() == current_title.tier())
            .collect();

        let tier_name = match current_title.tier() {
            Rank::Beginner => "Beginner",
            Rank::Intermediate => "Intermediate",
            Rank::Advanced => "Advanced",
            Rank::Expert => "Expert",
            Rank::Legendary => "Legendary",
        }
        .to_string();

        // Find position within tier (1-based, highest score = rank 1)
        let tier_position = same_tier_titles
            .iter()
            .rev() // Reverse to get highest scores first
            .position(|title| title.name() == current_title.name())
            .map(|pos| pos + 1)
            .unwrap_or(1);

        let tier_total = same_tier_titles.len();

        // Find position in all titles (1-based, highest score = rank 1)
        let overall_position = all_titles
            .iter()
            .rev() // Reverse to get highest scores first
            .position(|title| title.name() == current_title.name())
            .map(|pos| pos + 1)
            .unwrap_or(1);

        let overall_total = all_titles.len();

        (
            tier_name,
            tier_position,
            tier_total,
            overall_position,
            overall_total,
        )
    }

    /// Legacy method that returns title name as string for a score for backward compatibility
    pub fn get_ranking_title_string_for_score(score: f64) -> String {
        match score as usize {
            // Beginner Level (clean boundaries, ~even progression)
            0..=800 => "Hello World".to_string(),
            801..=1200 => "Syntax Error".to_string(),
            1201..=1600 => "Rubber Duck".to_string(),
            1601..=2000 => "Script Kid".to_string(),
            2001..=2450 => "Bash Newbie".to_string(),
            2451..=2900 => "CLI Wanderer".to_string(),
            2901..=3300 => "Tab Tamer".to_string(),
            3301..=3700 => "Bracket Juggler".to_string(),
            3701..=4150 => "Copy-Paste Engineer".to_string(),
            4151..=4550 => "Linter Apprentice".to_string(),
            4551..=5000 => "Unit Test Trainee".to_string(),
            5001..=5600 => "Code Monkey".to_string(),

            // Intermediate Level (clean midpoints rounded to 50/100)
            5601..=5850 => "Ticket Picker".to_string(),
            5851..=6000 => "Junior Dev".to_string(),
            6001..=6100 => "Git Ninja".to_string(),
            6101..=6250 => "Merge Wrangler".to_string(),
            6251..=6400 => "API Crafter".to_string(),
            6401..=6550 => "Frontend Dev".to_string(),
            6551..=6700 => "Backend Dev".to_string(),
            6701..=6850 => "CI Tinkerer".to_string(),
            6851..=7000 => "Test Pilot".to_string(),
            7001..=7100 => "Build Tamer".to_string(),
            7101..=7250 => "Code Reviewer".to_string(),
            7251..=7500 => "Release Handler".to_string(),

            // Advanced Level (clean midpoints rounded)
            7501..=7800 => "Refactorer".to_string(),
            7801..=8000 => "Senior Dev".to_string(),
            8001..=8100 => "DevOps Engineer".to_string(),
            8101..=8250 => "Incident Responder".to_string(),
            8251..=8400 => "Reliability Guardian".to_string(),
            8401..=8500 => "Security Engineer".to_string(),
            8501..=8650 => "Performance Alchemist".to_string(),
            8651..=8800 => "Data Pipeline Master".to_string(),
            8801..=8950 => "Tech Lead".to_string(),
            8951..=9100 => "Architect".to_string(),
            9101..=9250 => "Protocol Artisan".to_string(),
            9251..=9500 => "Kernel Hacker".to_string(),

            // Expert Level (clean boundaries aiming 50/100 steps)
            9501..=9800 => "Compiler".to_string(),
            9801..=9950 => "Bytecode Interpreter".to_string(),
            9951..=10100 => "Virtual Machine".to_string(),
            10101..=10200 => "Operating System".to_string(),
            10201..=10350 => "Filesystem".to_string(),
            10351..=10500 => "Network Stack".to_string(),
            10501..=10650 => "Database Engine".to_string(),
            10651..=10800 => "Query Optimizer".to_string(),
            10801..=10950 => "Cloud Platform".to_string(),
            10951..=11100 => "Container Orchestrator".to_string(),
            11101..=11200 => "Stream Processor".to_string(),
            11201..=11400 => "Quantum Computer".to_string(),

            // Legendary Level (start at 11420; rounded bands)
            11401..=11700 => "GPU Cluster".to_string(),
            11701..=12250 => "DNS Overlord".to_string(),
            12251..=12800 => "CDN Sentinel".to_string(),
            12801..=13400 => "Load Balancer Primarch".to_string(),
            13401..=13950 => "Singularity".to_string(),
            13951..=14500 => "The Machine".to_string(),
            14501..=15100 => "Origin".to_string(),
            15101..=15650 => "SegFault".to_string(),
            15651..=16200 => "Buffer Overflow".to_string(),
            16201..=16800 => "Memory Leak".to_string(),
            16801..=17350 => "Null Pointer Exception".to_string(),
            17351..=17900 => "Undefined Behavior".to_string(),
            17901..=18500 => "Heisenbug".to_string(),
            18501..=19100 => "Blue Screen".to_string(),
            _ => "Kernel Panic".to_string(),
        }
    }

    /// Get legacy ranking title name for a specific score (deprecated - use get_ranking_title_for_score instead)
    pub fn get_ranking_title_legacy_for_score(score: f64) -> String {
        Self::get_ranking_title_for_score(score).name().to_string()
    }

    /// Calculate base score from current metrics
    #[allow(dead_code)]
    fn calculate_base_score(&self) -> f64 {
        self.cpm() * (self.accuracy() / 100.0) * 10.0 // 10x scaling for 1000+ range
    }

    /// Calculate consistency bonus for real-time estimation
    #[allow(dead_code)]
    fn calculate_realtime_consistency_bonus(&self) -> f64 {
        let base_score = self.calculate_base_score();
        let accuracy = self.accuracy().clamp(0.0, 100.0) / 100.0; // normalize to 0..1
        let factor = if accuracy <= 0.7 {
            0.0
        } else if accuracy < 0.9 {
            // Smoothly increase 0 -> 0.5 between 70% and 90%
            let t = (accuracy - 0.7) / 0.2; // 0..1
            let s = t * t * (3.0 - 2.0 * t); // smoothstep
            0.5 * s
        } else if accuracy < 0.95 {
            // Keep 90–95% plateau at 0.5 (stable zone)
            0.5
        } else {
            // 95–100%: increase 0.5 -> 0.7 smoothly
            let t = (accuracy - 0.95) / 0.05; // 0..1
            let s = t * t * (3.0 - 2.0 * t); // smoothstep
            0.5 + 0.2 * s
        };
        base_score * factor
    }

    /// Calculate consistency bonus from actual streak data
    #[allow(dead_code)]
    fn calculate_streak_consistency_bonus(&self) -> f64 {
        let streaks = self.all_streaks();
        if streaks.is_empty() {
            return 0.0;
        }

        let base_score = self.calculate_base_score();
        let avg_streak = streaks.iter().sum::<usize>() as f64 / streaks.len() as f64;
        let max_streak = *streaks.iter().max().unwrap_or(&0) as f64;

        // More sophisticated consistency calculation
        let streak_score = (avg_streak * 2.0) + (max_streak * 1.5);
        base_score * (streak_score / 100.0).min(0.8) // Cap at 80% bonus
    }

    /// Calculate time bonus for speed
    #[allow(dead_code)]
    fn calculate_time_bonus(&self) -> f64 {
        let total_chars = self.total_chars();
        if total_chars <= 50 {
            return 0.0;
        }

        let elapsed_secs = self.elapsed().as_secs_f64();
        let ideal_time = total_chars as f64 / 10.0; // 10 chars per second ideal
        if elapsed_secs < ideal_time {
            (ideal_time - elapsed_secs) * 20.0
        } else {
            0.0
        }
    }

    /// Calculate mistake penalty
    #[allow(dead_code)]
    fn calculate_mistake_penalty(&self) -> f64 {
        self.mistakes() as f64 * 5.0
    }

    /// Calculate final challenge score
    fn calculate_challenge_score(&self) -> f64 {
        Self::calculate_score_from_metrics(
            self.cpm(),
            self.accuracy(),
            self.mistakes(),
            self.elapsed().as_secs_f64(),
            self.total_chars(),
        )
    }

    /// Pure function to calculate score from metrics (for testing)
    pub fn calculate_score_from_metrics(
        cpm: f64,
        accuracy: f64,
        mistakes: usize,
        elapsed_secs: f64,
        total_chars: usize,
    ) -> f64 {
        // Base score calculation
        let base_score = cpm * (accuracy / 100.0) * 10.0;

        // Consistency bonus based on accuracy (continuous, smoothstep)
        let a = accuracy.clamp(0.0, 100.0) / 100.0; // 0..1
        let consistency_factor = if a <= 0.7 {
            0.0
        } else if a < 0.9 {
            // 70–90%: 0 -> 0.5
            let t = (a - 0.7) / 0.2; // 0..1
            let s = t * t * (3.0 - 2.0 * t); // smoothstep
            0.5 * s
        } else if a < 0.95 {
            // 90–95%: plateau at 0.5
            0.5
        } else {
            // 95–100%: 0.5 -> 0.7 smoothly
            let t = (a - 0.95) / 0.05; // 0..1
            let s = t * t * (3.0 - 2.0 * t); // smoothstep
            0.5 + 0.2 * s
        };
        let consistency_bonus = base_score * consistency_factor;

        // Time bonus for speed
        let time_bonus = if total_chars > 50 {
            let ideal_time = total_chars as f64 / 10.0; // 10 chars per second ideal
            if elapsed_secs < ideal_time {
                (ideal_time - elapsed_secs) * 20.0
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Mistake penalty
        let mistake_penalty = mistakes as f64 * 5.0;

        // Final calculation
        let raw_score = base_score + consistency_bonus + time_bonus - mistake_penalty;
        (raw_score * 2.0 + 100.0).max(0.0) // Final scaling + base offset
    }

    pub fn calculate_result(&self) -> Result<StageResult> {
        self.calculate_result_with_status(false, false)
    }

    pub fn calculate_result_with_skip_status(&self, was_skipped: bool) -> Result<StageResult> {
        self.calculate_result_with_status(was_skipped, false)
    }

    pub fn calculate_result_with_status(
        &self,
        was_skipped: bool,
        was_failed: bool,
    ) -> Result<StageResult> {
        if self.start_time.is_none() {
            return Err(crate::GitTypeError::TerminalError(
                "Scoring not started".to_string(),
            ));
        }

        let challenge_score = self.calculate_challenge_score();
        let ranking_title = Self::get_ranking_title_for_score(challenge_score)
            .name()
            .to_string();
        let (tier_name, tier_position, tier_total, overall_position, overall_total) =
            Self::calculate_tier_info(challenge_score);

        Ok(StageResult {
            cpm: self.cpm(),
            wpm: self.wpm(),
            accuracy: self.accuracy(),
            mistakes: self.mistakes(),
            consistency_streaks: self.all_streaks(),
            completion_time: self.elapsed(),
            challenge_score,
            ranking_title,
            rank: tier_name,
            tier_position,
            tier_total,
            overall_position,
            overall_total,
            was_skipped,
            was_failed,
        })
    }

    /// Calculate metrics from current position during real-time typing
    /// This uses the same logic as the full ScoringEngine but works with current state
    pub fn calculate_real_time_result(
        current_position: usize,
        mistakes: usize,
        start_time: &std::time::Instant,
    ) -> StageResult {
        // Create temporary engine with real-time data
        let mut temp_engine = ScoringEngine::new(String::new());
        temp_engine.start_time = Some(*start_time);
        temp_engine.recorded_duration = Some(start_time.elapsed());

        // Simulate keystrokes for calculations
        for i in 0..current_position {
            temp_engine.keystrokes.push(Keystroke {
                character: 'x', // Placeholder
                position: i,
                is_correct: i < current_position.saturating_sub(mistakes),
                timestamp: *start_time,
            });
        }

        let challenge_score = temp_engine.calculate_challenge_score();
        let ranking_title = Self::get_ranking_title_for_score(challenge_score)
            .name()
            .to_string();
        let (tier_name, tier_position, tier_total, overall_position, overall_total) =
            Self::calculate_tier_info(challenge_score);

        StageResult {
            cpm: temp_engine.cpm(),
            wpm: temp_engine.wpm(),
            accuracy: temp_engine.accuracy(),
            mistakes: temp_engine.mistakes(),
            consistency_streaks: vec![], // Real-time doesn't track actual streaks
            completion_time: temp_engine.elapsed(),
            challenge_score,
            ranking_title,
            rank: tier_name,
            tier_position,
            tier_total,
            overall_position,
            overall_total,
            was_skipped: false, // Real-time metrics are not skipped
            was_failed: false,  // Real-time metrics are not failed
        }
    }
}

impl Add for ScoringEngine {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Combine keystrokes from both engines
        let mut combined_keystrokes = self.keystrokes;
        combined_keystrokes.extend(other.keystrokes);

        // Combine streaks from both engines
        let mut combined_streaks = self.streaks;
        combined_streaks.extend(other.streaks);

        // Use the earlier start time
        let combined_start_time = match (self.start_time, other.start_time) {
            (Some(start1), Some(start2)) => Some(start1.min(start2)),
            (Some(start), None) | (None, Some(start)) => Some(start),
            (None, None) => None,
        };

        // Combine durations by adding them together
        let combined_duration = match (self.recorded_duration, other.recorded_duration) {
            (Some(dur1), Some(dur2)) => Some(dur1 + dur2),
            (Some(dur), None) | (None, Some(dur)) => Some(dur),
            (None, None) => None,
        };

        // Combine target texts (for consistency, though not strictly needed for calculations)
        let combined_target_text = if self.target_text.is_empty() {
            other.target_text
        } else if other.target_text.is_empty() {
            self.target_text
        } else {
            format!("{}\n{}", self.target_text, other.target_text)
        };

        ScoringEngine {
            start_time: combined_start_time,
            keystrokes: combined_keystrokes,
            target_text: combined_target_text,
            current_streak: 0, // Reset current streak for combined engine
            streaks: combined_streaks,
            recorded_duration: combined_duration,
            paused_time: None,
            total_paused_duration: std::time::Duration::ZERO,
        }
    }
}

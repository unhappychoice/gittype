use crate::Result;
use crate::scoring::{TypingMetrics, ScoringEngine};
use crossterm::terminal;
use super::{
    challenge::Challenge,
    screens::{TitleScreen, ResultScreen, CountdownScreen, TypingScreen, TitleAction},
    stage_builder::{StageBuilder, GameMode, DifficultyLevel},
};

pub struct StageManager {
    available_challenges: Vec<Challenge>,
    current_challenges: Vec<Challenge>,
    current_stage: usize,
    stage_engines: Vec<(String, ScoringEngine)>,
}

impl StageManager {
    pub fn new(challenges: Vec<Challenge>) -> Self {
        Self {
            available_challenges: challenges,
            current_challenges: Vec::new(),
            current_stage: 0,
            stage_engines: Vec::new(),
        }
    }

    pub fn run_session(&mut self) -> Result<()> {
        // Enable raw mode for entire application session
        match terminal::enable_raw_mode() {
            Ok(_) => {},
            Err(e) => {
                return Err(crate::error::GitTypeError::TerminalError(
                    format!("Failed to enable raw mode: {}", e)
                ));
            }
        }

        loop {
            // Count challenges by difficulty level
            let challenge_counts = self.count_challenges_by_difficulty();
            
            match TitleScreen::show_with_challenge_counts(&challenge_counts)? {
                TitleAction::Start(difficulty) => {
                    // Build stages based on selected difficulty using pre-generated challenges
                    let game_mode = GameMode::Custom {
                        max_stages: Some(3),
                        time_limit: None,
                        difficulty: difficulty.clone(),
                    };
                    
                    let stage_builder = StageBuilder::with_mode(game_mode);
                    self.current_challenges = stage_builder.build_stages(self.available_challenges.clone());
                    
                    // Debug output
                    println!("Selected difficulty: {:?}", difficulty);
                    println!("Built {} challenges with difficulty {:?}", self.current_challenges.len(), difficulty);
                    for (i, challenge) in self.current_challenges.iter().enumerate() {
                        println!("  Challenge {}: {} ({} lines)", i+1, challenge.id, challenge.code_content.lines().count());
                    }
                    
                    if self.current_challenges.is_empty() {
                        println!("No challenges found for difficulty {:?}", difficulty);
                        continue; // Go back to title screen
                    }
                    
                    // Reset session metrics
                    self.current_stage = 0;
                    self.stage_engines.clear();
                    
                    match self.run_stages() {
                        Ok(_session_complete) => {
                            // After session completes, continue to title screen
                            // User can choose to play again or quit
                        },
                        Err(e) => {
                            terminal::disable_raw_mode()?;
                            return Err(e);
                        }
                    }
                },
                TitleAction::Quit => break,
            }
        }

        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn run_stages(&mut self) -> Result<bool> {
        while self.current_stage < self.current_challenges.len() {
            let challenge = &self.current_challenges[self.current_stage];
            
            // Show countdown before each stage
            if self.current_stage == 0 {
                // First stage - show initial countdown
                CountdownScreen::show()?;
            } else {
                // Subsequent stages - show stage transition countdown
                CountdownScreen::show_stage_transition(self.current_stage + 1, self.current_challenges.len())?;
            }
            
            let mut screen = TypingScreen::new_with_challenge(challenge)?;
            let metrics = screen.show()?; // Use show method like other screens
            
            // Record stage completion
            self.stage_engines.push((challenge.get_display_title(), screen.get_scoring_engine().clone()));
            
            // Show brief result and auto-advance
            self.show_stage_completion(&metrics)?;
            
            // Move to next stage
            self.current_stage += 1;
        }
        
        // All stages completed - show final results (raw mode still enabled)
        self.show_session_summary()?;
        Ok(true) // Return true to indicate session completed
    }

    fn show_stage_completion(&self, metrics: &TypingMetrics) -> Result<()> {
        // Get keystrokes from the latest scoring engine
        let keystrokes = if let Some((_, engine)) = self.stage_engines.last() {
            engine.total_chars()
        } else {
            0
        };
        
        ResultScreen::show_stage_completion(
            metrics, 
            self.current_stage + 1, 
            self.current_challenges.len(),
            self.current_stage < self.current_challenges.len() - 1, // has_next_stage
            keystrokes
        )
    }


    fn show_session_summary(&self) -> Result<()> {
        let _result = ResultScreen::show_session_summary(
            self.current_challenges.len(),
            self.stage_engines.len(),
            &self.stage_engines,
        )?;
        Ok(())
    }

    pub fn get_current_stage(&self) -> usize {
        self.current_stage
    }

    pub fn get_total_stages(&self) -> usize {
        self.current_challenges.len()
    }

    
    fn count_challenges_by_difficulty(&self) -> [usize; 5] {
        let mut counts = [0usize; 5];
        
        for challenge in &self.available_challenges {
            if let Some(ref difficulty) = challenge.difficulty_level {
                match difficulty {
                    DifficultyLevel::Easy => counts[0] += 1,
                    DifficultyLevel::Normal => counts[1] += 1,
                    DifficultyLevel::Hard => counts[2] += 1,
                    DifficultyLevel::Wild => counts[3] += 1,
                    DifficultyLevel::Zen => counts[4] += 1,
                }
            }
        }
        
        counts
    }
}
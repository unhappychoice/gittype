use crate::Result;
use crate::scoring::TypingMetrics;
use crossterm::terminal;
use super::{
    challenge::Challenge,
    screens::{TitleScreen, ResultScreen, CountdownScreen, TypingScreen, TitleAction},
};

#[derive(Debug, Clone)]
pub struct SessionMetrics {
    pub total_stages: usize,
    pub completed_stages: usize,
    pub stage_metrics: Vec<(String, TypingMetrics)>,
    pub total_wpm: f64,
    pub total_accuracy: f64,
    pub total_mistakes: usize,
    pub session_score: f64,
}

pub struct StageManager {
    challenges: Vec<Challenge>,
    current_stage: usize,
    session_metrics: SessionMetrics,
}

impl StageManager {
    pub fn new(challenges: Vec<Challenge>) -> Self {
        let total_stages = challenges.len();
        Self {
            challenges,
            current_stage: 0,
            session_metrics: SessionMetrics {
                total_stages,
                completed_stages: 0,
                stage_metrics: Vec::new(),
                total_wpm: 0.0,
                total_accuracy: 0.0,
                total_mistakes: 0,
                session_score: 0.0,
            },
        }
    }

    pub fn run_session(&mut self) -> Result<()> {
        // Set up signal handler for Ctrl+C
        ctrlc::set_handler(move || {
            terminal::disable_raw_mode().ok();
            std::process::exit(0);
        }).expect("Error setting Ctrl-C handler");

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
            match TitleScreen::show()? {
                TitleAction::Start => {
                    self.current_stage = 0;
                    self.session_metrics.completed_stages = 0;
                    self.session_metrics.stage_metrics.clear();
                    self.session_metrics.total_wpm = 0.0;
                    self.session_metrics.total_accuracy = 0.0;
                    self.session_metrics.total_mistakes = 0;
                    self.session_metrics.session_score = 0.0;
                    
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
        while self.current_stage < self.challenges.len() {
            let challenge = &self.challenges[self.current_stage];
            
            // Show countdown before each stage
            if self.current_stage == 0 {
                // First stage - show initial countdown
                CountdownScreen::show()?;
            } else {
                // Subsequent stages - show stage transition countdown
                CountdownScreen::show_stage_transition(self.current_stage + 1, self.challenges.len())?;
            }
            
            let mut screen = TypingScreen::new_with_challenge(challenge);
            let metrics = screen.show()?; // Use show method like other screens
            
            // Record stage completion
            self.session_metrics.stage_metrics.push((challenge.get_display_title(), metrics.clone()));
            self.session_metrics.completed_stages += 1;
            self.update_session_metrics(&metrics);
            
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
        ResultScreen::show_stage_completion(
            metrics, 
            self.current_stage + 1, 
            self.challenges.len(),
            self.current_stage < self.challenges.len() - 1  // has_next_stage
        )
    }

    fn update_session_metrics(&mut self, stage_metrics: &TypingMetrics) {
        let completed = self.session_metrics.completed_stages as f64;
        
        // Calculate running averages
        self.session_metrics.total_wpm = 
            (self.session_metrics.total_wpm * (completed - 1.0) + stage_metrics.wpm) / completed;
        
        self.session_metrics.total_accuracy = 
            (self.session_metrics.total_accuracy * (completed - 1.0) + stage_metrics.accuracy) / completed;
        
        self.session_metrics.total_mistakes += stage_metrics.mistakes;
        
        self.session_metrics.session_score = 
            self.session_metrics.total_wpm * (self.session_metrics.total_accuracy / 100.0);
    }

    fn show_session_summary(&self) -> Result<()> {
        let _result = ResultScreen::show_session_summary(&self.session_metrics)?;
        Ok(())
    }

    pub fn get_current_stage(&self) -> usize {
        self.current_stage
    }

    pub fn get_total_stages(&self) -> usize {
        self.challenges.len()
    }

    pub fn get_session_metrics(&self) -> &SessionMetrics {
        &self.session_metrics
    }
}
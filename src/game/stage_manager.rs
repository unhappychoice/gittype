use crate::Result;
use crate::scoring::{TypingMetrics, ScoringEngine};
use crate::extractor::GitRepositoryInfo;
use crossterm::terminal;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use super::{
    challenge::Challenge,
    screens::{TitleScreen, ResultScreen, CountdownScreen, TypingScreen, TitleAction, result_screen::ResultAction, ExitSummaryScreen},
    stage_builder::{StageBuilder, GameMode, DifficultyLevel},
    session_tracker::SessionTracker,
};

// Global session tracker for Ctrl+C handler
static GLOBAL_SESSION_TRACKER: Lazy<Arc<Mutex<Option<SessionTracker>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

pub struct StageManager {
    available_challenges: Vec<Challenge>,
    current_challenges: Vec<Challenge>,
    current_stage: usize,
    stage_engines: Vec<(String, ScoringEngine)>,
    current_game_mode: Option<GameMode>,
    session_tracker: SessionTracker,
    git_info: Option<GitRepositoryInfo>,
}

impl StageManager {
    pub fn new(challenges: Vec<Challenge>) -> Self {
        Self {
            available_challenges: challenges,
            current_challenges: Vec::new(),
            current_stage: 0,
            stage_engines: Vec::new(),
            current_game_mode: None,
            session_tracker: SessionTracker::new(),
            git_info: None,
        }
    }
    
    pub fn set_git_info(&mut self, git_info: Option<GitRepositoryInfo>) {
        self.git_info = git_info;
    }

    pub fn run_session(&mut self) -> Result<()> {
        // Set global session tracker for Ctrl+C handler
        {
            let mut global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
            *global_tracker = Some(self.session_tracker.clone());
        }

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
            
            match TitleScreen::show_with_challenge_counts_and_git_info(&challenge_counts, self.git_info.as_ref())? {
                TitleAction::Start(difficulty) => {
                    // Build stages based on selected difficulty using pre-generated challenges
                    let game_mode = GameMode::Custom {
                        max_stages: Some(3),
                        time_limit: None,
                        difficulty: difficulty.clone(),
                    };
                    
                    self.current_game_mode = Some(game_mode.clone());
                    
                    loop {
                        let stage_builder = StageBuilder::with_mode(game_mode.clone());
                        self.current_challenges = stage_builder.build_stages(self.available_challenges.clone());
                        
                        // Debug output
                        println!("Selected difficulty: {:?}", difficulty);
                        println!("Built {} challenges with difficulty {:?}", self.current_challenges.len(), difficulty);
                        for (i, challenge) in self.current_challenges.iter().enumerate() {
                            println!("  Challenge {}: {} ({} lines)", i+1, challenge.id, challenge.code_content.lines().count());
                        }
                        
                        if self.current_challenges.is_empty() {
                            println!("No challenges found for difficulty {:?}", difficulty);
                            break; // Go back to title screen
                        }
                        
                        // Reset session metrics
                        self.current_stage = 0;
                        self.stage_engines.clear();
                        
                        match self.run_stages() {
                            Ok(session_complete) => {
                                if !session_complete {
                                    break; // User chose to quit or back to title
                                }
                                // If session_complete is true, retry with same settings
                            },
                            Err(e) => {
                                terminal::disable_raw_mode()?;
                                return Err(e);
                            }
                        }
                    }
                },
                TitleAction::Quit => {
                    // Show session summary before exiting
                    let session_summary = self.session_tracker.clone().finalize_and_get_summary();
                    let _ = ExitSummaryScreen::show(&session_summary)?;
                    break;
                },
            }
        }

        // Clear global session tracker
        {
            let mut global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
            *global_tracker = None;
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
            let stage_name = challenge.get_display_title();
            let engine = screen.get_scoring_engine().clone();
            
            self.stage_engines.push((stage_name.clone(), engine.clone()));
            
            // Track in session tracker
            self.session_tracker.record_stage_completion(stage_name, metrics.clone(), &engine);
            
            // Update global session tracker
            {
                let mut global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
                if let Some(ref mut tracker) = *global_tracker {
                    *tracker = self.session_tracker.clone();
                }
            }
            
            // Show brief result and auto-advance
            self.show_stage_completion(&metrics)?;
            
            // Move to next stage
            self.current_stage += 1;
        }
        
        // All stages completed - show final results (raw mode still enabled)
        match self.show_session_summary()? {
            ResultAction::Retry => Ok(true), // Return true to indicate retry requested
            ResultAction::Quit => {
                // Show session summary before exiting
                terminal::disable_raw_mode()?;
                let session_summary = self.session_tracker.clone().finalize_and_get_summary();
                terminal::enable_raw_mode()?;
                let _ = ExitSummaryScreen::show(&session_summary)?;
                terminal::disable_raw_mode()?;
                std::process::exit(0);
            },
            _ => Ok(false), // Return false for back to title
        }
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


    fn show_session_summary(&self) -> Result<ResultAction> {
        ResultScreen::show_session_summary_with_input(
            self.current_challenges.len(),
            self.stage_engines.len(),
            &self.stage_engines,
        )
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

// Public function for Ctrl+C handler
pub fn show_session_summary_on_interrupt() {
    let _ = terminal::disable_raw_mode();
    
    let global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
    if let Some(ref tracker) = *global_tracker {
        let session_summary = tracker.clone().finalize_and_get_summary();
        
        // Show session summary
        let _ = ExitSummaryScreen::show(&session_summary);
    } else {
        // Show simple interruption message in terminal UI style
        use crossterm::{execute, style::{Print, SetForegroundColor, Color, ResetColor}, cursor::MoveTo};
        use std::io::stdout;
        
        let mut stdout = stdout();
        let _ = execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All));
        let _ = execute!(stdout, MoveTo(10, 5));
        let _ = execute!(stdout, SetForegroundColor(Color::Yellow));
        let _ = execute!(stdout, Print("Interrupted by user - no session data available"));
        let _ = execute!(stdout, ResetColor);
        let _ = execute!(stdout, MoveTo(10, 7));
        let _ = execute!(stdout, Print("Thanks for playing GitType!"));
        let _ = execute!(stdout, MoveTo(10, 9));
        let _ = execute!(stdout, SetForegroundColor(Color::Grey));
        let _ = execute!(stdout, Print("Press any key to exit..."));
        let _ = execute!(stdout, ResetColor);
        
        // Wait for any key
        use crossterm::event;
        loop {
            if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
                if let Ok(event::Event::Key(_)) = event::read() {
                    break;
                }
            }
        }
    }
}
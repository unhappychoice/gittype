use crate::Result;
use crate::scoring::{TypingMetrics, ScoringEngine};
use crate::extractor::GitRepositoryInfo;
use crossterm::{terminal, execute, event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags, PopKeyboardEnhancementFlags}};
use std::io::stdout;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use super::{
    challenge::Challenge,
    screens::{TitleScreen, ResultScreen, CountdownScreen, TypingScreen, TitleAction, result_screen::ResultAction, ExitSummaryScreen, exit_summary_screen::ExitAction, typing_screen::GameState},
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
    skips_remaining: usize,
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
            skips_remaining: 3,
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

        // Enable keyboard enhancement flags to better detect modifier combinations
        let mut stdout_handle = stdout();
        execute!(stdout_handle, PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES | 
            KeyboardEnhancementFlags::REPORT_EVENT_TYPES |
            KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
        )).ok(); // Ignore errors in case terminal doesn't support it

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
                        self.skips_remaining = 3;
                        
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

        // Disable keyboard enhancement flags
        let mut stdout_handle = stdout();
        execute!(stdout_handle, PopKeyboardEnhancementFlags).ok();

        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn run_stages(&mut self) -> Result<bool> {
        while self.current_stage < self.current_challenges.len() {
            let challenge = &self.current_challenges[self.current_stage];
            
            // Show countdown before each stage
            if self.current_stage == 0 {
                // First stage - show initial countdown with challenge info
                CountdownScreen::show_with_challenge(Some(challenge))?;
            } else {
                // Subsequent stages - show stage transition countdown with challenge info
                CountdownScreen::show_stage_transition_with_challenge(self.current_stage + 1, self.current_challenges.len(), Some(challenge))?;
            }
            
            let mut screen = TypingScreen::new_with_challenge(challenge)?;
            screen.set_skips_remaining(self.skips_remaining);
            let (metrics, final_state) = screen.show_with_state()?;
            self.skips_remaining = screen.get_skips_remaining();
            
            // Handle different exit states
            match final_state {
                GameState::Complete => {
                    // Normal completion - advance to next stage
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
                },
                GameState::Skip => {
                    // Skipped - record skip and partial effort
                    let engine = screen.get_scoring_engine();
                    self.session_tracker.record_skip();
                    self.session_tracker.record_partial_effort(engine, &metrics);
                    
                    // Update global session tracker
                    {
                        let mut global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
                        if let Some(ref mut tracker) = *global_tracker {
                            *tracker = self.session_tracker.clone();
                        }
                    }
                    
                    self.show_stage_completion(&metrics)?;
                    
                    // Generate a new challenge for the current stage
                    if let Some(ref game_mode) = self.current_game_mode {
                        let stage_builder = StageBuilder::with_mode(game_mode.clone());
                        let new_challenges = stage_builder.build_stages(self.available_challenges.clone());
                        
                        if !new_challenges.is_empty() && self.current_stage < new_challenges.len() {
                            // Replace current challenge with a new one
                            self.current_challenges[self.current_stage] = new_challenges[self.current_stage].clone();
                        }
                    }
                    // Don't increment current_stage - retry same stage with new challenge
                },
                GameState::Failed => {
                    // Failed - show fail result screen with navigation options
                    let stage_name = challenge.get_display_title();
                    let engine = screen.get_scoring_engine().clone();
                    
                    self.stage_engines.push((stage_name.clone(), engine.clone()));
                    
                    // Track in session tracker
                    self.session_tracker.record_stage_completion(stage_name, metrics.clone(), &engine);
                    
                    // Show fail result screen and handle navigation
                    return self.handle_fail_result_navigation();
                },
                GameState::Exit => {
                    // User wants to exit - record partial effort only
                    let engine = screen.get_scoring_engine();
                    self.session_tracker.record_partial_effort(engine, &metrics);
                    
                    // Update global session tracker with current state
                    {
                        let mut global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
                        if let Some(ref mut tracker) = *global_tracker {
                            *tracker = self.session_tracker.clone();
                        }
                    }
                    
                    // Show session summary and exit
                    let session_summary = self.session_tracker.clone().finalize_and_get_summary();
                    terminal::disable_raw_mode()?;
                    let _ = ExitSummaryScreen::show(&session_summary)?;
                    std::process::exit(0);
                },
                GameState::Continue | GameState::ShowDialog => {
                    // This shouldn't happen in final state
                    unreachable!("Continue/ShowDialog state should not be final");
                }
            }
        }
        
        // All stages completed - show final results (raw mode still enabled)
        let mut first_show = true;
        loop {
            let action = if first_show {
                first_show = false;
                self.show_session_summary()?
            } else {
                self.show_session_summary_no_animation()?
            };

            match action {
                ResultAction::Retry => return Ok(true), // Return true to indicate retry requested
                ResultAction::Share => {
                    // Show sharing menu
                    if let Some(last_engine) = self.stage_engines.last() {
                        if let Ok(metrics) = last_engine.1.calculate_metrics() {
                            let _ = ResultScreen::show_sharing_menu(&metrics);
                        }
                    }
                    // Continue showing the summary screen after sharing (without animation)
                    continue;
                },
                ResultAction::Quit => {
                    // Show session summary before exiting
                    let session_summary = self.session_tracker.clone().finalize_and_get_summary();
                    
                    loop {
                        eprintln!("Debug: About to call ExitSummaryScreen::show");
                        eprintln!("Debug: Calling ExitSummaryScreen::show...");
                        let exit_action = match ExitSummaryScreen::show(&session_summary) {
                            Ok(action) => {
                                eprintln!("Debug: ExitSummaryScreen::show returned: {:?}", action);
                                action
                            },
                            Err(e) => {
                                eprintln!("Error in ExitSummaryScreen::show: {}", e);
                                return Err(e);
                            }
                        };
                        
                        match exit_action {
                            ExitAction::Exit => {
                                terminal::disable_raw_mode()?;
                                std::process::exit(0);
                            },
                            ExitAction::Share => {
                                eprintln!("Debug: Share action received, calling show_sharing_menu");
                                if let Err(e) = ExitSummaryScreen::show_sharing_menu(&session_summary) {
                                    eprintln!("Failed to show sharing menu: {}", e);
                                } else {
                                    eprintln!("Debug: Sharing menu returned successfully");
                                }
                                // Continue showing exit screen after sharing
                                eprintln!("Debug: Continuing to show exit screen");
                                continue;
                            },
                        }
                    }
                },
                _ => return Ok(false), // Return false for back to title
            }
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
        self.show_session_summary_internal(true)
    }

    fn show_session_summary_no_animation(&self) -> Result<ResultAction> {
        self.show_session_summary_internal(false)
    }

    fn show_session_summary_internal(&self, show_animation: bool) -> Result<ResultAction> {
        if show_animation {
            ResultScreen::show_session_summary_with_input(
                self.current_challenges.len(),
                self.stage_engines.len(),
                &self.stage_engines,
            )
        } else {
            ResultScreen::show_session_summary_with_input_no_animation(
                self.current_challenges.len(),
                self.stage_engines.len(),
                &self.stage_engines,
            )
        }
    }

    pub fn get_current_stage(&self) -> usize {
        self.current_stage
    }

    pub fn get_total_stages(&self) -> usize {
        self.current_challenges.len()
    }

    
    fn handle_fail_result_navigation(&self) -> Result<bool> {
        use crossterm::event::{self, Event, KeyCode};
        
        // Show fail result screen
        ResultScreen::show_session_summary_fail_mode(
            self.current_challenges.len(),
            self.stage_engines.len(),
            &self.stage_engines,
        )?;
        
        // Wait for navigation input
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Enter => {
                            // Back to title screen
                            return Ok(false);
                        },
                        KeyCode::Esc => {
                            // Show session summary and exit
                            let session_summary = self.session_tracker.clone().finalize_and_get_summary();
                            let _ = ExitSummaryScreen::show(&session_summary)?;
                            std::process::exit(0);
                        },
                        KeyCode::Char('c') if key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                            std::process::exit(0);
                        },
                        _ => {}
                    }
                }
            }
        }
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
    // Keep raw mode enabled since ExitSummaryScreen needs it for input handling
    
    let global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
    if let Some(ref tracker) = *global_tracker {
        let session_summary = tracker.clone().finalize_and_get_summary();
        
        // Show session summary with raw mode enabled
        let _ = ExitSummaryScreen::show(&session_summary);
        // Disable raw mode after ExitSummaryScreen completes
        let _ = terminal::disable_raw_mode();
    } else {
        // Show simple interruption message
        let _ = terminal::disable_raw_mode();
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
        
        // Enable raw mode temporarily for input
        let _ = terminal::enable_raw_mode();
        use crossterm::event;
        loop {
            if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
                if let Ok(event::Event::Key(_)) = event::read() {
                    break;
                }
            }
        }
        let _ = terminal::disable_raw_mode();
    }
}
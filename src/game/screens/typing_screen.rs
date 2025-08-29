use crate::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal,
};
use crate::scoring::{TypingMetrics, engine::ScoringEngine};
use super::{
    super::{
        text_processor::TextProcessor,
        display_ratatui::GameDisplayRatatui,
        challenge::Challenge,
    },
    {TitleScreen, ResultScreen, CountdownScreen, TitleAction, ResultAction},
};

pub struct TypingScreen {
    challenge: Option<Challenge>,
    challenge_text: String,
    challenge_chars: Vec<char>,
    current_position: usize,
    mistakes: usize,
    start_time: std::time::Instant,
    line_starts: Vec<usize>,
    comment_ranges: Vec<(usize, usize)>,
    mistake_positions: Vec<usize>,
    current_mistake_position: Option<usize>,
    display: GameDisplayRatatui,
    scoring_engine: ScoringEngine,
}

pub enum ScreenState {
    Title,
    Playing,
    Results(TypingMetrics),
}

enum GameState {
    Continue,
    Complete,
    Exit,
}

impl TypingScreen {
    pub fn new(challenge_text: String) -> Result<Self> {
        let processed_text = TextProcessor::process_challenge_text(&challenge_text);
        let challenge_chars: Vec<char> = processed_text.chars().collect();
        let line_starts = TextProcessor::calculate_line_starts(&processed_text);
        let comment_ranges = vec![]; // No comment info available without Challenge
        let initial_position = TextProcessor::find_first_non_whitespace_or_comment(&processed_text, 0, &comment_ranges);
        let display = GameDisplayRatatui::new(&processed_text)?;
        let mut scoring_engine = ScoringEngine::new(processed_text.clone());
        scoring_engine.start(); // Start timing immediately
        
        Ok(Self {
            challenge: None,
            challenge_text: processed_text,
            challenge_chars,
            current_position: initial_position,
            mistakes: 0,
            start_time: std::time::Instant::now(),
            line_starts,
            comment_ranges,
            mistake_positions: Vec::new(),
            current_mistake_position: None,
            display,
            scoring_engine,
        })
    }

    pub fn new_with_challenge(challenge: &Challenge) -> Result<Self> {
        // Apply basic text processing (remove empty lines, etc.)
        // Indentation normalization is already done in extractor
        let (processed_text, mapped_comment_ranges) = TextProcessor::process_challenge_text_with_comment_mapping(
            &challenge.code_content, 
            &challenge.comment_ranges
        );
        
        let challenge_chars: Vec<char> = processed_text.chars().collect();
        let line_starts = TextProcessor::calculate_line_starts(&processed_text);
        let initial_position = TextProcessor::find_first_non_whitespace_or_comment(&processed_text, 0, &mapped_comment_ranges);
        let display = GameDisplayRatatui::new(&processed_text)?;
        let mut scoring_engine = ScoringEngine::new(processed_text.clone());
        scoring_engine.start(); // Start timing immediately
        
        Ok(Self {
            challenge: Some(challenge.clone()),
            challenge_text: processed_text,
            challenge_chars,
            current_position: initial_position,
            mistakes: 0,
            start_time: std::time::Instant::now(),
            line_starts,
            comment_ranges: mapped_comment_ranges,
            mistake_positions: Vec::new(),
            current_mistake_position: None,
            display,
            scoring_engine,
        })
    }

    pub fn run_full_session(&mut self) -> Result<()> {
        match terminal::enable_raw_mode() {
            Ok(_) => {},
            Err(e) => {
                return Err(crate::error::GitTypeError::TerminalError(
                    format!("Failed to enable raw mode: {}", e)
                ));
            }
        }

        let mut current_state = ScreenState::Title;
        
        loop {
            match current_state {
                ScreenState::Title => {
                    match TitleScreen::show()? {
                        TitleAction::Start(_) => {
                            self.reset_game();
                            current_state = ScreenState::Playing;
                        },
                        TitleAction::Quit => break,
                    }
                },
                ScreenState::Playing => {
                    let metrics = self.start_session()?;
                    current_state = ScreenState::Results(metrics);
                },
                ScreenState::Results(ref metrics) => {
                    match ResultScreen::show(metrics)? {
                        ResultAction::Restart => {
                            self.reset_game();
                            current_state = ScreenState::Playing;
                        },
                        ResultAction::BackToTitle => {
                            current_state = ScreenState::Title;
                        },
                        ResultAction::Quit => break,
                    }
                }
            }
        }

        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn reset_game(&mut self) {
        self.current_position = TextProcessor::find_first_non_whitespace_or_comment(&self.challenge_text, 0, &self.comment_ranges);
        self.mistakes = 0;
        self.start_time = std::time::Instant::now();
        self.mistake_positions.clear();
        self.current_mistake_position = None;
    }

    pub fn start_session(&mut self) -> Result<TypingMetrics> {
        match terminal::enable_raw_mode() {
            Ok(_) => {},
            Err(e) => {
                return Err(crate::error::GitTypeError::TerminalError(
                    format!("Failed to enable raw mode: {}", e)
                ));
            }
        }

        // Only show countdown for standalone sessions (not when called from StageManager)
        if self.challenge.is_none() {
            CountdownScreen::show()?;
        }
        
        // Reset start time after countdown
        self.start_time = std::time::Instant::now();

        self.display.display_challenge_with_info(
            &self.challenge_text,
            self.current_position,
            self.mistakes,
            &self.start_time,
            &self.line_starts,
            &self.comment_ranges,
            self.challenge.as_ref(),
            self.current_mistake_position,
        )?;
        
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match self.handle_key(key_event)? {
                        GameState::Continue => {
                            self.update_display()?;
                        },
                        GameState::Complete => {
                            break;
                        },
                        GameState::Exit => {
                            break;
                        }
                    }
                }
            }
        }

        self.display.cleanup()?;
        terminal::disable_raw_mode()?;
        self.scoring_engine.finish(); // Record final duration
        Ok(self.calculate_metrics())
    }

    pub fn show(&mut self) -> Result<TypingMetrics> {
        // For stage manager - assumes raw mode is already enabled
        self.start_time = std::time::Instant::now();

        self.display.display_challenge_with_info(
            &self.challenge_text,
            self.current_position,
            self.mistakes,
            &self.start_time,
            &self.line_starts,
            &self.comment_ranges,
            self.challenge.as_ref(),
            self.current_mistake_position,
        )?;
        
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match self.handle_key(key_event)? {
                        GameState::Continue => {
                            self.update_display()?;
                        },
                        GameState::Complete => {
                            break;
                        },
                        GameState::Exit => {
                            break;
                        }
                    }
                }
            }
        }

        self.scoring_engine.finish(); // Record final duration
        Ok(self.calculate_metrics())
    }

    fn handle_key(&mut self, key_event: KeyEvent) -> Result<GameState> {
        match key_event.code {
            KeyCode::Esc => Ok(GameState::Exit),
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                terminal::disable_raw_mode().ok();
                std::process::exit(0);
            },
            KeyCode::Char(ch) => {
                if self.current_position < self.challenge_chars.len() {
                    let expected_char = self.challenge_chars[self.current_position];
                    let is_correct = ch == expected_char;
                    
                    // Record keystroke in scoring engine
                    self.scoring_engine.record_keystroke(ch, self.current_position);
                    
                    if is_correct {
                        self.current_mistake_position = None;
                        self.current_position += 1;
                        // Skip over any non-typeable characters (comments, whitespace)
                        self.advance_to_next_typeable_character();
                        if self.current_position >= self.challenge_chars.len() {
                            return Ok(GameState::Complete);
                        }
                    } else {
                        self.mistakes += 1;
                        self.mistake_positions.push(self.current_position);
                        self.current_mistake_position = Some(self.current_position);
                    }
                }
                Ok(GameState::Continue)
            },
            KeyCode::Tab => {
                // Handle tab character
                if self.current_position < self.challenge_text.len() {
                    let expected_char = self.challenge_text.chars().nth(self.current_position).unwrap();
                    let is_correct = expected_char == '\t';
                    
                    // Record keystroke in scoring engine
                    self.scoring_engine.record_keystroke('\t', self.current_position);
                    
                    if is_correct {
                        self.current_mistake_position = None;
                        self.current_position += 1;
                        // Skip over any non-typeable characters (comments, whitespace)
                        self.advance_to_next_typeable_character();
                        if self.current_position >= self.challenge_text.len() {
                            return Ok(GameState::Complete);
                        }
                    } else {
                        self.mistakes += 1;
                        self.mistake_positions.push(self.current_position);
                        self.current_mistake_position = Some(self.current_position);
                    }
                }
                Ok(GameState::Continue)
            },
            KeyCode::Enter => {
                // Auto-advance when reaching end of line (after last code character)
                if self.current_position < self.challenge_text.len() {
                    let is_correct = self.is_at_end_of_line_content();
                    
                    // Record keystroke in scoring engine
                    self.scoring_engine.record_keystroke('\n', self.current_position);
                    
                    // Check if we're at a newline or at the end of code content on a line
                    if is_correct {
                        // Skip over the newline and any following whitespace/comments to next typeable character
                        self.current_mistake_position = None;
                        self.advance_to_next_line()?;
                        if self.current_position >= self.challenge_text.len() {
                            return Ok(GameState::Complete);
                        }
                    } else {
                        self.mistakes += 1;
                        self.mistake_positions.push(self.current_position);
                        self.current_mistake_position = Some(self.current_position);
                    }
                }
                Ok(GameState::Continue)
            },
            _ => Ok(GameState::Continue),
        }
    }

    fn update_display(&mut self) -> Result<()> {
        self.display.display_challenge_with_info(
            &self.challenge_text,
            self.current_position,
            self.mistakes,
            &self.start_time,
            &self.line_starts,
            &self.comment_ranges,
            self.challenge.as_ref(),
            self.current_mistake_position,
        )
    }

    fn is_at_end_of_line_content(&self) -> bool {
        TextProcessor::is_at_end_of_line_content(
            &self.challenge_text, 
            self.current_position, 
            &self.line_starts, 
            &self.comment_ranges
        )
    }

    fn advance_to_next_line(&mut self) -> Result<()> {
        let chars: Vec<char> = self.challenge_text.chars().collect();
        
        // Skip current position if it's a newline
        if self.current_position < chars.len() && chars[self.current_position] == '\n' {
            self.current_position += 1;
        }
        
        // Skip to next typeable character
        self.current_position = TextProcessor::find_first_non_whitespace_or_comment(
            &self.challenge_text, 
            self.current_position, 
            &self.comment_ranges
        );
        
        Ok(())
    }

    fn calculate_metrics(&self) -> TypingMetrics {
        self.scoring_engine.calculate_metrics().unwrap()
    }

    pub fn get_scoring_engine(&self) -> &ScoringEngine {
        &self.scoring_engine
    }

    fn advance_to_next_typeable_character(&mut self) {
        while self.current_position < self.challenge_chars.len() {
            // Check if current position should be skipped (comment, whitespace, etc.)
            if TextProcessor::should_skip_character(
                &self.challenge_text,
                self.current_position,
                &self.line_starts,
                &self.comment_ranges,
            ) {
                self.current_position += 1;
            } else {
                // Found a typeable character
                break;
            }
        }
    }
}
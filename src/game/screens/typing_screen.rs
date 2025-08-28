use crate::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal,
};
use crate::scoring::TypingMetrics;
use super::{
    super::{
        comment_parser::CommentParser,
        text_processor::TextProcessor,
        display::GameDisplay,
        challenge::Challenge,
    },
    {TitleScreen, ResultScreen, CountdownScreen, TitleAction, ResultAction},
};

pub struct TypingScreen {
    challenge: Option<Challenge>,
    challenge_text: String,
    current_position: usize,
    mistakes: usize,
    start_time: std::time::Instant,
    line_starts: Vec<usize>,
    comment_ranges: Vec<(usize, usize)>,
    mistake_positions: Vec<usize>,
    current_mistake_position: Option<usize>,
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
    pub fn new(challenge_text: String) -> Self {
        let processed_text = TextProcessor::process_challenge_text(&challenge_text);
        let line_starts = TextProcessor::calculate_line_starts(&processed_text);
        let comment_ranges = CommentParser::detect_comments(&processed_text);
        let initial_position = TextProcessor::find_first_non_whitespace_or_comment(&processed_text, 0, &comment_ranges);
        Self {
            challenge: None,
            challenge_text: processed_text,
            current_position: initial_position,
            mistakes: 0,
            start_time: std::time::Instant::now(),
            line_starts,
            comment_ranges,
            mistake_positions: Vec::new(),
            current_mistake_position: None,
        }
    }

    pub fn new_with_challenge(challenge: &Challenge) -> Self {
        let processed_text = TextProcessor::process_challenge_text(&challenge.code_content);
        let line_starts = TextProcessor::calculate_line_starts(&processed_text);
        let comment_ranges = CommentParser::detect_comments(&processed_text);
        let initial_position = TextProcessor::find_first_non_whitespace_or_comment(&processed_text, 0, &comment_ranges);
        Self {
            challenge: Some(challenge.clone()),
            challenge_text: processed_text,
            current_position: initial_position,
            mistakes: 0,
            start_time: std::time::Instant::now(),
            line_starts,
            comment_ranges,
            mistake_positions: Vec::new(),
            current_mistake_position: None,
        }
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
                        TitleAction::Start => {
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

        GameDisplay::display_challenge_with_info(
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

        terminal::disable_raw_mode()?;
        Ok(self.calculate_metrics())
    }

    pub fn show(&mut self) -> Result<TypingMetrics> {
        // For stage manager - assumes raw mode is already enabled
        self.start_time = std::time::Instant::now();

        GameDisplay::display_challenge_with_info(
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
                if self.current_position < self.challenge_text.len() {
                    let expected_char = self.challenge_text.chars().nth(self.current_position).unwrap();
                    if ch == expected_char {
                        self.current_mistake_position = None;
                        self.current_position += 1;
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
            KeyCode::Tab => {
                // Handle tab character
                if self.current_position < self.challenge_text.len() {
                    let expected_char = self.challenge_text.chars().nth(self.current_position).unwrap();
                    if expected_char == '\t' {
                        self.current_mistake_position = None;
                        self.current_position += 1;
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
                    // Check if we're at a newline or at the end of code content on a line
                    if self.is_at_end_of_line_content() {
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

    fn update_display(&self) -> Result<()> {
        GameDisplay::display_challenge_with_info(
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
        let elapsed = self.start_time.elapsed();
        let words_typed = self.current_position as f64 / 5.0;
        let wpm = (words_typed / elapsed.as_secs_f64()) * 60.0;
        let total_chars = self.current_position.max(1);
        
        // Prevent overflow when mistakes > total_chars
        let correct_chars = if self.mistakes > total_chars {
            0
        } else {
            total_chars - self.mistakes
        };
        
        let accuracy = (correct_chars as f64 / total_chars as f64) * 100.0;

        TypingMetrics {
            wpm,
            accuracy,
            mistakes: self.mistakes,
            corrections: 0,
            consistency_score: accuracy,
            completion_time: elapsed,
            challenge_score: wpm * (accuracy / 100.0),
        }
    }
}
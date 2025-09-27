use crate::domain::models::RankTier;
use ratatui::style::Color;
use std::time::{Duration, Instant};

/// Typing animation phases
#[derive(Debug, Clone)]
pub enum AnimationPhase {
    ConcentrationLines,
    Pause, // Brief pause after lines complete with dots
    Complete,
}

/// Terminal hacking line for typewriter effect
#[derive(Debug, Clone)]
pub struct HackingLine {
    pub text: String,
    pub color: Color,
    pub typed_length: usize,
    pub completed: bool,
    pub completion_time: Option<Instant>,
    pub start_time: Option<Instant>,
}

/// Typing animation view
pub struct TypingAnimationView {
    phase: AnimationPhase,
    phase_start: Instant,
    hacking_lines: Vec<HackingLine>,
    current_line: usize,
    pause_dots: usize, // Number of dots to show during pause
}

impl TypingAnimationView {
    pub fn new(_tier: RankTier, _terminal_width: u16, _terminal_height: u16) -> Self {
        Self {
            phase: AnimationPhase::ConcentrationLines,
            phase_start: Instant::now(),
            hacking_lines: Vec::new(),
            current_line: 0,
            pause_dots: 0,
        }
    }

    pub fn set_rank_messages(&mut self, rank_name: &str) {
        use crate::presentation::game::rank_messages::get_colored_messages_for_rank;
        let colored_messages = get_colored_messages_for_rank(rank_name);
        self.hacking_lines = colored_messages
            .into_iter()
            .map(|msg| HackingLine {
                text: msg.text,
                color: msg.color,
                typed_length: 0,
                completed: false,
                completion_time: None,
                start_time: None,
            })
            .collect();
    }

    pub fn update(&mut self) -> bool {
        let phase_elapsed = self.phase_start.elapsed();
        let mut phase_changed = false;

        match self.phase {
            AnimationPhase::ConcentrationLines => {
                // Slightly slower typewriter effect
                let total_lines = self.hacking_lines.len(); // Store length to avoid borrowing conflict
                if self.current_line < total_lines {
                    let line = &mut self.hacking_lines[self.current_line];

                    // Set start time for current line if not set
                    if line.start_time.is_none() {
                        line.start_time = Some(Instant::now());
                    }

                    if line.typed_length < line.text.len() {
                        // Type characters at moderate speed - one character every 40ms
                        if let Some(start_time) = line.start_time {
                            let line_elapsed = start_time.elapsed();
                            let chars_to_type = ((line_elapsed.as_millis() / 40) as usize + 1)
                                .saturating_sub(line.typed_length);
                            if chars_to_type > 0 {
                                line.typed_length =
                                    (line.typed_length + chars_to_type.min(1)).min(line.text.len());
                            }
                        }
                    } else if !line.completed {
                        line.completed = true;
                        line.completion_time = Some(Instant::now());

                        // If this is the last line, no need to wait for delay
                        if self.current_line == total_lines - 1 {
                            self.current_line += 1;
                        }
                    } else if line.completion_time.is_some() && self.current_line < total_lines - 1
                    {
                        // Check if delay period has elapsed since line completion (not for last line)
                        if let Some(completion_time) = line.completion_time {
                            if completion_time.elapsed() >= Duration::from_millis(500) {
                                self.current_line += 1;
                            }
                        }
                    }
                }

                // Animation moves to pause when all lines are typed
                let all_completed = self.hacking_lines.iter().all(|line| line.completed);
                if all_completed {
                    self.phase = AnimationPhase::Pause;
                    self.phase_start = Instant::now();
                    phase_changed = true;
                }
            }
            AnimationPhase::Pause => {
                // Show dots at moderate pace during pause
                let dot_interval = 500; // 500ms per dot
                let new_dots = (phase_elapsed.as_millis() / dot_interval as u128) as usize;
                if new_dots != self.pause_dots {
                    self.pause_dots = new_dots.min(7);
                    phase_changed = true;
                }

                // Complete after showing 7 dots and brief wait
                if self.pause_dots >= 7 && phase_elapsed >= Duration::from_millis(2800) {
                    self.phase = AnimationPhase::Complete;
                    phase_changed = true;
                }
            }
            AnimationPhase::Complete => {
                // Animation is complete
            }
        }

        phase_changed || matches!(self.phase, AnimationPhase::ConcentrationLines)
    }

    pub fn get_current_phase(&self) -> &AnimationPhase {
        &self.phase
    }

    pub fn get_hacking_lines(&self) -> &Vec<HackingLine> {
        &self.hacking_lines
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.phase, AnimationPhase::Complete)
    }

    pub fn get_current_line(&self) -> usize {
        self.current_line
    }

    pub fn get_pause_dots(&self) -> usize {
        self.pause_dots
    }
}

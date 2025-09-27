use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Keystroke {
    pub character: char,
    pub position: usize,
    pub is_correct: bool,
    pub timestamp: Instant,
}

/// Stage level raw data tracking
#[derive(Clone)]
pub struct StageTracker {
    pub start_time: Option<Instant>,
    keystrokes: Vec<Keystroke>,
    target_text: String,
    current_streak: usize,
    streaks: Vec<usize>,
    recorded_duration: Option<std::time::Duration>,
    paused_time: Option<Instant>,
    total_paused_duration: std::time::Duration,
    challenge_path: String,
    was_skipped: bool,
    was_failed: bool,
}

impl StageTracker {
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
            challenge_path: String::new(),
            was_skipped: false,
            was_failed: false,
        }
    }

    pub fn new_with_path(target_text: String, challenge_path: String) -> Self {
        Self {
            start_time: None,
            keystrokes: Vec::new(),
            target_text,
            current_streak: 0,
            streaks: Vec::new(),
            recorded_duration: None,
            paused_time: None,
            total_paused_duration: std::time::Duration::ZERO,
            challenge_path,
            was_skipped: false,
            was_failed: false,
        }
    }

    /// Set the start time manually for precise timing control
    pub fn set_start_time(&mut self, start_time: Instant) {
        self.start_time = Some(start_time);
    }

    pub fn record(&mut self, input: StageInput) {
        match input {
            StageInput::Start => {
                // Only set start_time if not already set (to preserve manually set time)
                if self.start_time.is_none() {
                    self.start_time = Some(Instant::now());
                }
            }
            StageInput::Keystroke { ch, position } => {
                if self.recorded_duration.is_some() {
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
            StageInput::Finish => {
                if let Some(paused_time) = self.paused_time {
                    self.total_paused_duration += paused_time.elapsed();
                    self.paused_time = None;
                }
                if let Some(start) = self.start_time {
                    self.recorded_duration =
                        Some(start.elapsed().saturating_sub(self.total_paused_duration));
                }
            }
            StageInput::Pause => {
                if self.paused_time.is_none() {
                    self.paused_time = Some(Instant::now());
                }
            }
            StageInput::Resume => {
                if let Some(paused_time) = self.paused_time {
                    self.total_paused_duration += paused_time.elapsed();
                    self.paused_time = None;
                }
            }
            StageInput::Skip => {
                self.was_skipped = true;
                if let Some(paused_time) = self.paused_time {
                    self.total_paused_duration += paused_time.elapsed();
                    self.paused_time = None;
                }
                if let Some(start) = self.start_time {
                    self.recorded_duration =
                        Some(start.elapsed().saturating_sub(self.total_paused_duration));
                }
            }
            StageInput::Fail => {
                self.was_failed = true;
                if let Some(paused_time) = self.paused_time {
                    self.total_paused_duration += paused_time.elapsed();
                    self.paused_time = None;
                }
                if let Some(start) = self.start_time {
                    self.recorded_duration =
                        Some(start.elapsed().saturating_sub(self.total_paused_duration));
                }
            }
        }
    }

    pub fn get_data(&self) -> StageTrackerData {
        let elapsed_time = if let Some(recorded) = self.recorded_duration {
            recorded
        } else if let Some(start) = self.start_time {
            let total_elapsed = start.elapsed();
            let paused_duration = if let Some(paused_time) = self.paused_time {
                self.total_paused_duration + paused_time.elapsed()
            } else {
                self.total_paused_duration
            };
            total_elapsed.saturating_sub(paused_duration)
        } else {
            std::time::Duration::ZERO
        };

        StageTrackerData {
            start_time: self.start_time,
            keystrokes: self.keystrokes.clone(),
            is_finished: self.recorded_duration.is_some(),
            elapsed_time,
            streaks: self.streaks.clone(),
            current_streak: self.current_streak,
            target_text: self.target_text.clone(),
            challenge_path: self.challenge_path.clone(),
            was_skipped: self.was_skipped,
            was_failed: self.was_failed,
        }
    }
}

#[derive(Debug, Clone)]
pub enum StageInput {
    Start,
    Keystroke { ch: char, position: usize },
    Finish,
    Pause,
    Resume,
    Skip,
    Fail,
}

#[derive(Debug, Clone)]
pub struct StageTrackerData {
    pub start_time: Option<Instant>,
    pub keystrokes: Vec<Keystroke>,
    pub is_finished: bool,
    pub elapsed_time: std::time::Duration,
    pub streaks: Vec<usize>,
    pub current_streak: usize,
    pub target_text: String,
    pub challenge_path: String,
    pub was_skipped: bool,
    pub was_failed: bool,
}

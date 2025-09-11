use std::time::{Duration, Instant};

pub struct Countdown {
    active: bool,
    current_number: Option<u8>,
    start_time: Option<Instant>,
    pause_time: Option<Instant>,
    total_paused: Duration,
}

impl Countdown {
    pub fn new() -> Self {
        Self {
            active: false,
            current_number: None,
            start_time: None,
            pause_time: None,
            total_paused: Duration::ZERO,
        }
    }

    pub fn start_countdown(&mut self) {
        self.active = true;
        self.current_number = Some(3);
        self.start_time = Some(Instant::now());
        self.pause_time = None;
        self.total_paused = Duration::ZERO;
    }

    pub fn update_state(&mut self) -> Option<Instant> {
        if !self.active {
            return None;
        }

        if let Some(current_num) = self.current_number {
            if let Some(start_time) = self.start_time {
                // Calculate elapsed time excluding paused duration
                let total_elapsed = start_time.elapsed();
                let current_paused = if let Some(pause_time) = self.pause_time {
                    self.total_paused + pause_time.elapsed()
                } else {
                    self.total_paused
                };
                let elapsed = total_elapsed.saturating_sub(current_paused);

                let required_duration = if current_num == 0 {
                    // GO! shows for 400ms (shorter duration)
                    Duration::from_millis(400)
                } else {
                    // Numbers 3, 2, 1 show for 600ms each
                    Duration::from_millis(600)
                };

                if elapsed >= required_duration {
                    if current_num > 1 {
                        // Move to next countdown number
                        self.current_number = Some(current_num - 1);
                        self.start_time = Some(Instant::now());
                        self.pause_time = None;
                        self.total_paused = Duration::ZERO;
                    } else if current_num == 1 {
                        // Show "GO!" for a brief moment
                        self.current_number = Some(0); // 0 represents "GO!"
                        self.start_time = Some(Instant::now());
                        self.pause_time = None;
                        self.total_paused = Duration::ZERO;
                    } else {
                        // Countdown finished, start typing
                        self.active = false;
                        self.current_number = None;
                        self.start_time = None;
                        self.pause_time = None;
                        self.total_paused = Duration::ZERO;

                        // Return the timestamp for when typing should start
                        return Some(Instant::now());
                    }
                }
            }
        }

        None
    }

    pub fn get_current_count(&self) -> Option<u8> {
        if self.active {
            self.current_number
        } else {
            None
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn pause(&mut self) {
        if self.active && self.pause_time.is_none() {
            self.pause_time = Some(Instant::now());
        }
    }

    pub fn resume(&mut self) {
        if let Some(pause_time) = self.pause_time.take() {
            self.total_paused += pause_time.elapsed();
        }
    }
}

impl Default for Countdown {
    fn default() -> Self {
        Self::new()
    }
}

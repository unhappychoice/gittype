pub trait ProgressReporter {
    fn set_phase(&self, phase: String);
    fn set_progress(&self, progress: f64);
    fn set_current_file(&self, file: Option<String>);
    fn set_file_counts(&self, processed: usize, total: usize);
    fn update_spinner(&self);
    fn finish(&self) -> crate::Result<()> {
        Ok(())
    }
}

pub struct NoOpProgressReporter;

impl ProgressReporter for NoOpProgressReporter {
    fn set_phase(&self, _phase: String) {}
    fn set_progress(&self, _progress: f64) {}
    fn set_current_file(&self, _file: Option<String>) {}
    fn set_file_counts(&self, _processed: usize, _total: usize) {}
    fn update_spinner(&self) {}
}

pub struct ConsoleProgressReporter {
    last_phase: std::sync::Mutex<String>,
    last_progress: std::sync::Mutex<f64>,
    spinner_chars: &'static [char],
    spinner_index: std::sync::Mutex<usize>,
}

impl Default for ConsoleProgressReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsoleProgressReporter {
    pub fn new() -> Self {
        Self {
            last_phase: std::sync::Mutex::new(String::new()),
            last_progress: std::sync::Mutex::new(0.0),
            spinner_chars: &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            spinner_index: std::sync::Mutex::new(0),
        }
    }

    fn get_spinner_char(&self) -> char {
        let index = *self.spinner_index.lock().unwrap();
        self.spinner_chars[index % self.spinner_chars.len()]
    }

    fn create_progress_bar(&self, progress: f64, width: usize) -> String {
        let filled = (progress * width as f64) as usize;
        let empty = width - filled;
        format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
    }
}

impl ProgressReporter for ConsoleProgressReporter {
    fn set_phase(&self, phase: String) {
        let mut last_phase = self.last_phase.lock().unwrap();
        if *last_phase != phase {
            if !last_phase.is_empty() {
                println!(); // Only add newline if this isn't the first phase
            }
            print!("🔄 {}... ", phase);
            std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());
            *last_phase = phase;
        }
    }

    fn set_progress(&self, progress: f64) {
        *self.last_progress.lock().unwrap() = progress;
    }

    fn set_current_file(&self, file: Option<String>) {
        if let Some(file_path) = file {
            let spinner = self.get_spinner_char();
            let progress = *self.last_progress.lock().unwrap();
            let progress_bar = self.create_progress_bar(progress, 20);
            let filename = std::path::Path::new(&file_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&file_path);
            // Clear the line and show current file being processed
            print!(
                "\r{} {} {:.1}% {:<50}\r",
                spinner,
                progress_bar,
                progress * 100.0,
                filename
            );
            std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());
        }
    }

    fn set_file_counts(&self, processed: usize, total: usize) {
        if total > 0 {
            let progress = processed as f64 / total as f64;
            let spinner = self.get_spinner_char();
            let progress_bar = self.create_progress_bar(progress, 20);

            print!(
                "\r{} {} {:.1}% ({}/{})                    ",
                spinner,
                progress_bar,
                progress * 100.0,
                processed,
                total
            );
            std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());
        }
    }

    fn update_spinner(&self) {
        let mut index = self.spinner_index.lock().unwrap();
        *index = (*index + 1) % self.spinner_chars.len();
    }
}

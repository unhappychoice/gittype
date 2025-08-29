use std::io;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::CrosstermBackend, Terminal};
use crate::extractor::{ConsoleProgressReporter, LoadingProgress, ProgressReporter, RepositoryLoader, ExtractionOptions};
use crate::game::Challenge;
use crate::{Result, GitTypeError};
use super::{LoadingScreen, LoadingPhase};

pub struct LoadingRunner {
    progress: Arc<LoadingProgress>,
    loading_screen: LoadingScreen,
}

impl LoadingRunner {
    pub fn new() -> Self {
        Self {
            progress: Arc::new(LoadingProgress::new()),
            loading_screen: LoadingScreen::new(),
        }
    }

    pub fn run_with_loading<F, T>(&mut self, operation: F) -> Result<T>
    where
        F: FnOnce(&LoadingProgress) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        // Try to setup terminal - if it fails, run without loading screen
        let terminal_result = self.setup_terminal();
        
        if let Ok(mut terminal) = terminal_result {
            self.run_with_terminal(&mut terminal, operation)
        } else {
            // Fallback: run without UI
            println!("🚀 GitType Loading...");
            let progress_clone = Arc::clone(&self.progress);
            let result = operation(&progress_clone);
            println!("✅ Loading complete!");
            result
        }
    }

    fn setup_terminal(&self) -> Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
        crossterm::terminal::enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        terminal.clear()?;
        Ok(terminal)
    }

    fn run_with_terminal<F, T>(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        operation: F,
    ) -> Result<T>
    where
        F: FnOnce(&LoadingProgress) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let progress_clone = Arc::clone(&self.progress);
        let (tx, rx) = std::sync::mpsc::channel();

        let operation_thread = thread::spawn(move || {
            let result = operation(&progress_clone);
            tx.send(result).unwrap();
        });

        let mut last_spinner_update = Instant::now();
        let spinner_interval = Duration::from_millis(100);

        let result = loop {
            if let Err(e) = terminal.draw(|frame| {
                self.update_loading_screen_from_progress();
                self.loading_screen.render(frame);
            }) {
                eprintln!("Warning: Terminal draw error: {}", e);
                break self.fallback_operation_wait(rx, operation_thread);
            }

            if last_spinner_update.elapsed() >= spinner_interval {
                self.loading_screen.update_spinner();
                last_spinner_update = Instant::now();
            }

            if event::poll(Duration::from_millis(50)).unwrap_or(false) {
                if let Ok(Event::Key(key)) = event::read() {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            break Err(GitTypeError::UserCancelled);
                        }
                        _ => {}
                    }
                }
            }

            if let Ok(result) = rx.try_recv() {
                operation_thread.join().unwrap();
                break result;
            }

            thread::sleep(Duration::from_millis(50));
        };

        let _ = crossterm::terminal::disable_raw_mode();
        let _ = terminal.show_cursor();
        result
    }

    fn fallback_operation_wait<T>(
        &self,
        rx: std::sync::mpsc::Receiver<Result<T>>,
        operation_thread: thread::JoinHandle<()>,
    ) -> Result<T> {
        // Wait for operation to complete without terminal UI
        let result = rx.recv().unwrap();
        operation_thread.join().unwrap();
        result
    }

    fn update_loading_screen_from_progress(&mut self) {
        let phase_str = self.progress.get_phase();
        let phase = match phase_str.as_str() {
            "Initializing" => LoadingPhase::Initializing,
            "Scanning repository" => LoadingPhase::Scanning,
            "Parsing AST" => LoadingPhase::ParsingAST,
            "Generating challenges" => LoadingPhase::GeneratingChallenges,
            "Finalizing" => LoadingPhase::Finalizing,
            _ => LoadingPhase::Initializing,
        };

        self.loading_screen.update_phase(phase);
        
        let (processed, total) = self.progress.get_file_counts();
        if total > 0 {
            self.loading_screen.set_file_counts(processed, total);
        }
        
        if let Some(file) = self.progress.get_current_file() {
            self.loading_screen.set_current_file(Some(file));
        }
    }

    pub fn load_challenges_with_loading(
        &mut self,
        repo_path: &Path,
        unit: &str,
        options: Option<ExtractionOptions>,
    ) -> Result<Vec<Challenge>> {
        let repo_path = repo_path.to_path_buf();
        let unit = unit.to_string();
        
        // Use the standard loading system, but show console progress for now
        println!("🚀 GitType Loading...");
        std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());
        let console_progress = ConsoleProgressReporter::new();
        
        let mut loader = RepositoryLoader::new()?;
        
        console_progress.set_phase("Initializing".to_string());
        
        let challenges = match unit.as_str() {
            "function" => loader.load_functions_only_with_progress(&repo_path, options, &console_progress)?,
            "class" | "struct" => loader.load_classes_only_with_progress(&repo_path, options, &console_progress)?,
            "all" => loader.load_challenges_from_repository_with_progress(&repo_path, options, &console_progress)?,
            _ => return Err(GitTypeError::ExtractionFailed(format!("Unknown unit type: {}", unit))),
        };

        println!("\n✅ Loading complete!");
        Ok(challenges)
    }
}

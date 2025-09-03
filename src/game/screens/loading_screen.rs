use crate::extractor::{ExtractionOptions, RepositoryLoader};
use crate::game::models::loading_steps::{ExecutionContext, StepManager, StepType};
use crate::models::Challenge;
use crate::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Gauge, Paragraph},
    Frame, Terminal,
};
use std::io::stdout;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, RwLock,
};
use std::thread;
use std::time::{Duration, Instant};

pub trait ProgressReporter {
    fn set_step(&self, step_type: StepType);
    fn set_progress(&self, progress: f64);
    fn set_current_file(&self, file: Option<String>);
    fn set_file_counts(&self, processed: usize, total: usize);
    fn finish(&self) -> crate::Result<()> {
        Ok(())
    }
}

pub struct NoOpProgressReporter;

impl ProgressReporter for NoOpProgressReporter {
    fn set_step(&self, _step_type: StepType) {}
    fn set_progress(&self, _progress: f64) {}
    fn set_current_file(&self, _file: Option<String>) {}
    fn set_file_counts(&self, _processed: usize, _total: usize) {}
}

#[derive(Clone)]
pub struct LoadingScreenState {
    pub current_step: Arc<RwLock<StepType>>,
    pub progress: Arc<RwLock<f64>>,
    pub files_processed: Arc<AtomicUsize>,
    pub total_files: Arc<AtomicUsize>,
    pub spinner_index: Arc<AtomicUsize>,
    pub should_stop: Arc<AtomicBool>,
    pub repo_info: Arc<RwLock<Option<String>>>,
    pub all_steps: Arc<RwLock<Vec<StepInfo>>>,
}

#[derive(Clone, Debug)]
pub struct StepInfo {
    pub step_type: StepType,
    pub step_number: usize,
    pub step_name: String,
    pub description: String,
}

pub struct LoadingScreen {
    state: LoadingScreenState,
    render_handle: Option<thread::JoinHandle<Result<()>>>,
}

const SPINNER_CHARS: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

pub struct ProcessingResult {
    pub challenges: Vec<Challenge>,
    pub git_repository: Option<crate::models::GitRepository>,
}

impl LoadingScreen {
    pub fn new() -> Result<Self> {
        let step_manager = Arc::new(StepManager::new());

        // Initialize step info from StepManager
        let steps_info: Vec<StepInfo> = step_manager
            .get_all_steps()
            .iter()
            .map(|step| StepInfo {
                step_type: step.step_type(),
                step_number: step.step_number(),
                step_name: step.step_name().to_string(),
                description: step.description().to_string(),
            })
            .collect();

        let state = LoadingScreenState {
            current_step: Arc::new(RwLock::new(StepType::Cloning)),
            progress: Arc::new(RwLock::new(0.0)),
            files_processed: Arc::new(AtomicUsize::new(0)),
            total_files: Arc::new(AtomicUsize::new(0)),
            spinner_index: Arc::new(AtomicUsize::new(0)),
            should_stop: Arc::new(AtomicBool::new(false)),
            repo_info: Arc::new(RwLock::new(None)),
            all_steps: Arc::new(RwLock::new(steps_info)),
        };

        Ok(Self {
            state,
            render_handle: None,
        })
    }

    pub fn show_initial(&mut self) -> Result<()> {
        self.start_rendering()?;
        Ok(())
    }

    fn start_rendering(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen)?;

        let state = self.state.clone();

        let handle = thread::spawn(move || -> Result<()> {
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;

            let target_fps = 60;
            let frame_duration = Duration::from_millis(1000 / target_fps);
            let mut last_spinner_update = Instant::now();

            loop {
                let start = Instant::now();

                // Update spinner every 100ms
                if last_spinner_update.elapsed() >= Duration::from_millis(100) {
                    let current_index = state.spinner_index.load(Ordering::Relaxed);
                    state
                        .spinner_index
                        .store((current_index + 1) % SPINNER_CHARS.len(), Ordering::Relaxed);
                    last_spinner_update = Instant::now();
                }

                // Render frame
                terminal.draw(|frame| Self::draw_ui_static(frame, &state))?;

                // Check for stop signal
                if state.should_stop.load(Ordering::Relaxed) {
                    break;
                }

                // Check for Ctrl+C (optional graceful handling)
                if event::poll(Duration::from_millis(0))? {
                    if let Event::Key(key) = event::read()? {
                        if key.code == KeyCode::Char('c')
                            && key.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            // Use global session tracker to show summary
                            crate::game::stage_manager::show_session_summary_on_interrupt();
                            std::process::exit(0);
                        }
                    }
                }

                // Sleep to maintain target FPS
                let elapsed = start.elapsed();
                if elapsed < frame_duration {
                    thread::sleep(frame_duration - elapsed);
                }
            }

            disable_raw_mode()?;
            terminal.backend_mut().execute(LeaveAlternateScreen)?;
            Ok(())
        });

        self.render_handle = Some(handle);

        // Give the render thread a moment to initialize
        thread::sleep(Duration::from_millis(50));
        Ok(())
    }

    pub fn update_progress(&self, progress: f64, processed: usize, total: usize) -> Result<()> {
        // Update state atomically - no rendering needed, render thread handles it
        if let Ok(mut p) = self.state.progress.write() {
            *p = progress;
        }
        self.state
            .files_processed
            .store(processed, Ordering::Relaxed);
        self.state.total_files.store(total, Ordering::Relaxed);
        Ok(())
    }

    pub fn set_repo_info(&self, repo_info: String) -> Result<()> {
        if let Ok(mut info) = self.state.repo_info.write() {
            *info = Some(repo_info);
        }
        Ok(())
    }

    pub fn set_git_repository(&self, git_repository: &crate::models::GitRepository) -> Result<()> {
        // Build git info string in same format as title_screen
        let mut parts = vec![format!(
            "📁 {}/{}",
            git_repository.user_name, git_repository.repository_name
        )];

        if let Some(ref branch) = git_repository.branch {
            parts.push(format!("🌿 {}", branch));
        }

        if let Some(ref commit) = git_repository.commit_hash {
            parts.push(format!("📝 {}", &commit[..8]));
        }

        let status_symbol = if git_repository.is_dirty {
            "⚠️"
        } else {
            "✓"
        };
        parts.push(status_symbol.to_string());

        let git_text = parts.join(" • ");

        if let Ok(mut info) = self.state.repo_info.write() {
            *info = Some(git_text);
        }
        Ok(())
    }

    pub fn show_completion(&mut self) -> Result<()> {
        // Mark as completed
        if let Ok(mut current_step) = self.state.current_step.write() {
            *current_step = StepType::Completed;
        }

        // Wait a moment to show the completion
        thread::sleep(Duration::from_millis(800));

        // Clean up terminal state before returning
        self.cleanup()?;

        Ok(())
    }

    pub fn show_completion_without_cleanup(&self) -> Result<()> {
        // Mark as completed
        if let Ok(mut current_step) = self.state.current_step.write() {
            *current_step = StepType::Completed;
        }

        // Wait a moment to show the completion
        thread::sleep(Duration::from_millis(500));

        Ok(())
    }

    pub fn process_repository(
        &mut self,
        repo_spec: Option<&str>,
        repo_path: Option<&PathBuf>,
        options: &ExtractionOptions,
    ) -> Result<ProcessingResult> {
        // Start rendering
        self.show_initial()?;

        // Execute step system
        let step_manager = StepManager::new();
        let mut loader = RepositoryLoader::new()?;

        let mut context = ExecutionContext {
            repo_spec,
            repo_path,
            extraction_options: Some(options),
            loading_screen: Some(self),
            repository_loader: Some(&mut loader),
            current_repo_path: None,
        };

        match step_manager.execute_pipeline(&mut context) {
            Ok(challenges) => {
                // Show completion
                let _ = self.show_completion_without_cleanup();

                Ok(ProcessingResult {
                    challenges,
                    git_repository: loader.get_git_repository().clone(),
                })
            }
            Err(e) => {
                let _ = self.cleanup();
                Err(e)
            }
        }
    }

    fn draw_ui_static(frame: &mut Frame, state: &LoadingScreenState) {
        let size = frame.size();

        // Get repo info for bottom display
        let repo_info = state
            .repo_info
            .read()
            .map(|r| r.clone())
            .unwrap_or_default();

        // Calculate main content height (excluding repo info at bottom)
        let content_height = 2 + 8 + 1 + 3; // Loading message + Description + Spacing + Progress
        let vertical_margin = (size.height.saturating_sub(content_height)) / 2;

        // Create layout with main content centered and repo info at bottom
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(vertical_margin),
                Constraint::Length(2), // Loading message
                Constraint::Length(8), // Description
                Constraint::Length(1), // Spacing
                Constraint::Length(3), // Progress
                Constraint::Min(1),    // Flexible space
                Constraint::Length(1), // Repo info at bottom
            ])
            .split(size);

        // Draw loading message
        Self::draw_loading_message_static(frame, main_layout[1]);

        // Draw description
        Self::draw_description_static(frame, main_layout[2], state);

        // Skip main_layout[3] for spacing

        // Draw progress
        Self::draw_progress_static(frame, main_layout[4], state);

        // Draw repo info at bottom if available
        if let Some(ref repo_info_text) = repo_info {
            Self::draw_repo_info_at_bottom_static(frame, main_layout[6], repo_info_text);
        }
    }

    fn draw_loading_message_static(frame: &mut Frame, area: Rect) {
        let loading_msg = Line::from(vec![
            Span::styled("» ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "Loading...",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        let loading = Paragraph::new(vec![loading_msg]).alignment(Alignment::Center);

        frame.render_widget(loading, area);
    }

    fn draw_description_static(frame: &mut Frame, area: Rect, state: &LoadingScreenState) {
        let current_step_type = state
            .current_step
            .read()
            .map(|x| x.clone())
            .unwrap_or(StepType::Cloning);

        let mut description_lines = vec![
            Line::from(Span::styled(
                "Analyzing your repository to create typing challenges...",
                Style::default().fg(Color::Gray),
            )),
            Line::from(Span::raw("")), // Empty line for spacing
        ];

        // Get steps from state
        if let Ok(steps) = state.all_steps.read() {
            for step_info in steps.iter() {
                let is_current = current_step_type == step_info.step_type;
                let is_completed = if current_step_type == StepType::Completed {
                    // If completed, all steps are completed
                    true
                } else {
                    // Check if this step comes before the current step in the sequence
                    step_info.step_number
                        < steps
                            .iter()
                            .find(|s| s.step_type == current_step_type)
                            .map(|s| s.step_number)
                            .unwrap_or(0)
                };

                let (icon, color) = if is_completed {
                    ("✓", Color::Green)
                } else if is_current {
                    ("⚡", Color::Yellow)
                } else {
                    ("○", Color::DarkGray)
                };

                description_lines.push(Line::from(vec![
                    Span::styled(format!("{} ", icon), Style::default().fg(color)),
                    Span::styled(
                        step_info.description.clone(),
                        Style::default().fg(if is_completed || is_current {
                            Color::Gray
                        } else {
                            Color::DarkGray
                        }),
                    ),
                ]));
            }
        }

        let description_paragraph = Paragraph::new(description_lines).alignment(Alignment::Center);

        frame.render_widget(description_paragraph, area);
    }

    fn draw_progress_static(frame: &mut Frame, area: Rect, state: &LoadingScreenState) {
        let progress = state.progress.read().map(|x| *x).unwrap_or(0.0);
        let files_processed = state.files_processed.load(Ordering::Relaxed);
        let total_files = state.total_files.load(Ordering::Relaxed);
        let current_step_type = state
            .current_step
            .read()
            .map(|x| x.clone())
            .unwrap_or(StepType::Cloning);

        if current_step_type == StepType::Completed {
            return;
        }

        // Show spinner for steps without meaningful progress data
        // Note: during scanning, total_files might be 0 but we still want to show progress
        if total_files == 0
            && files_processed == 0
            && !matches!(
                current_step_type,
                StepType::Cloning
                    | StepType::Scanning
                    | StepType::Generating
                    | StepType::Finalizing
            )
        {
            return;
        }

        // Get spinner character
        let spinner_index = state.spinner_index.load(Ordering::Relaxed);
        let spinner = SPINNER_CHARS[spinner_index % SPINNER_CHARS.len()];

        let progress_text = if total_files > 0 {
            let unit = match current_step_type {
                StepType::Generating => "challenges",
                StepType::Cloning => "", // Just show percentage for cloning
                _ => "files",
            };

            if current_step_type == StepType::Cloning {
                format!("{} {:.1}%", spinner, progress * 100.0)
            } else {
                format!(
                    "{} {:.1}% {}/{} {}",
                    spinner,
                    progress * 100.0,
                    files_processed,
                    total_files,
                    unit
                )
            }
        } else {
            format!("{} Working...", spinner)
        };

        // Progress bar
        let progress_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Progress bar
                Constraint::Length(1), // Text
            ])
            .split(area);

        // Render progress bar (only if we have meaningful progress)
        if total_files > 0 {
            let gauge = Gauge::default()
                .block(Block::default())
                .gauge_style(Style::default().fg(Color::Green))
                .ratio(progress.clamp(0.0, 1.0)); // Clamp progress to valid range

            frame.render_widget(gauge, progress_area[0]);
        }

        // Render progress text
        let progress_line = Line::from(Span::styled(
            progress_text,
            Style::default().fg(Color::Green),
        ));

        let progress_widget = Paragraph::new(vec![progress_line]).alignment(Alignment::Center);

        frame.render_widget(progress_widget, progress_area[1]);
    }

    fn draw_repo_info_at_bottom_static(frame: &mut Frame, area: Rect, repo_info: &str) {
        // Use same style as title_screen: DarkGrey color and centered
        let repo_line = Line::from(Span::styled(
            repo_info,
            Style::default().fg(Color::DarkGray),
        ));

        let repo_widget = Paragraph::new(vec![repo_line]).alignment(Alignment::Center);

        frame.render_widget(repo_widget, area);
    }

    pub fn cleanup(&mut self) -> Result<()> {
        // Signal render thread to stop
        self.state.should_stop.store(true, Ordering::Relaxed);

        // Wait for render thread to finish
        if let Some(handle) = self.render_handle.take() {
            let _ = handle.join();
        }

        // Cleanup is now handled by the render thread itself
        Ok(())
    }
}

impl Drop for LoadingScreen {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

impl ProgressReporter for LoadingScreen {
    fn set_step(&self, step_type: StepType) {
        if let Ok(mut current_step) = self.state.current_step.write() {
            *current_step = step_type;
        }
    }

    fn set_progress(&self, progress: f64) {
        // Safely get current values, use defaults if lock fails
        let processed = self.state.files_processed.load(Ordering::Relaxed);
        let total = self.state.total_files.load(Ordering::Relaxed);
        let _ = self.update_progress(progress, processed, total);
    }

    fn set_current_file(&self, _file: Option<String>) {
        // LoadingScreen doesn't display individual files
    }

    fn set_file_counts(&self, processed: usize, total: usize) {
        let progress = if total > 0 {
            processed as f64 / total as f64
        } else {
            0.0
        };
        let _ = self.update_progress(progress, processed, total);
    }
}

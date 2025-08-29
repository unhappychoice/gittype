use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Gauge, Paragraph},
    Frame, Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::stdout;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use crate::Result;
use crate::extractor::ProgressReporter;

pub struct LoadingScreen {
    terminal: Mutex<Terminal<CrosstermBackend<std::io::Stdout>>>,
    current_phase: Mutex<String>,
    current_step: Mutex<usize>,
    total_steps: usize,
    spinner_chars: Vec<char>,
    spinner_index: Mutex<usize>,
    progress: Mutex<f64>,
    files_processed: Mutex<usize>,
    total_files: Mutex<usize>,
    last_render: Mutex<Instant>,
    cleaned_up: Mutex<bool>,
}

impl LoadingScreen {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(Self {
            terminal: Mutex::new(terminal),
            current_phase: Mutex::new(String::new()),
            current_step: Mutex::new(0),
            total_steps: 5, // Initializing, Scanning, Parsing AST, Generating challenges, Finalizing
            spinner_chars: vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            spinner_index: Mutex::new(0),
            progress: Mutex::new(0.0),
            files_processed: Mutex::new(0),
            total_files: Mutex::new(0),
            last_render: Mutex::new(Instant::now()),
            cleaned_up: Mutex::new(false),
        })
    }

    pub fn show_initial(&self) -> Result<()> {
        self.render()
    }

    pub fn update_phase(&self, phase: &str) -> Result<()> {
        let mut current_phase = self.current_phase.lock().unwrap();
        if *current_phase != phase {
            *current_phase = phase.to_string();
            
            // Update step number based on phase
            let step_num = match phase {
                "Initializing" => 1,
                "Scanning repository" => 2,
                "Parsing AST" => 3,
                "Generating challenges" => 4,
                "Finalizing" => 5,
                _ => *self.current_step.lock().unwrap(),
            };
            *self.current_step.lock().unwrap() = step_num;
            drop(current_phase);
            
            // Force render for phase changes
            self.render_throttled(true)?;
        }
        Ok(())
    }

    pub fn update_progress(&self, progress: f64, processed: usize, total: usize) -> Result<()> {
        *self.progress.lock().unwrap() = progress;
        *self.files_processed.lock().unwrap() = processed;
        *self.total_files.lock().unwrap() = total;
        
        self.render()
    }

    pub fn update_spinner(&self) {
        // Check for Ctrl+C event
        if event::poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Ok(Event::Key(KeyEvent { 
                code: KeyCode::Char('c'), 
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                ..
            })) = event::read() {
                // Cleanup and exit gracefully
                let _ = self.cleanup();
                std::process::exit(0);
            }
        }
        
        let mut spinner_index = self.spinner_index.lock().unwrap();
        *spinner_index = (*spinner_index + 1) % self.spinner_chars.len();
        drop(spinner_index);
        
        // Render with throttling to prevent excessive updates
        let _ = self.render_throttled(false);
    }

    pub fn show_completion(&self) -> Result<()> {
        // Mark as completed and render
        *self.current_phase.lock().unwrap() = "Completed".to_string();
        *self.current_step.lock().unwrap() = self.total_steps;
        
        // Force render for completion
        self.render_throttled(true)?;
        
        // Wait a moment to show the completion
        std::thread::sleep(std::time::Duration::from_millis(800));
        
        // Clean up terminal state before returning
        self.cleanup()?;
        
        Ok(())
    }

    fn render(&self) -> Result<()> {
        self.render_throttled(false)
    }
    
    fn render_throttled(&self, force: bool) -> Result<()> {
        let now = Instant::now();
        
        if !force {
            let last_render = self.last_render.lock().unwrap();
            if now.duration_since(*last_render) < Duration::from_millis(50) {
                // Skip render if less than 50ms since last render
                return Ok(());
            }
        }
        
        let mut terminal = self.terminal.lock().unwrap();
        terminal.draw(|frame| self.draw_ui(frame))?;
        *self.last_render.lock().unwrap() = now;
        Ok(())
    }

    fn draw_ui(&self, frame: &mut Frame) {
        let size = frame.size();
        
        // Calculate total content height
        let content_height = 6 + 2 + 3 + 3; // Logo + Loading message + Phase + Progress
        let vertical_margin = (size.height.saturating_sub(content_height)) / 2;
        
        // Create vertical centering layout
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(vertical_margin),
                Constraint::Length(content_height),
                Constraint::Min(0),
            ])
            .split(size);
        
        // Main content layout
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),  // Logo
                Constraint::Length(2),  // Loading message
                Constraint::Length(3),  // Phase
                Constraint::Length(3),  // Progress
            ])
            .split(vertical_layout[1]);

        // Draw logo
        self.draw_logo(frame, main_layout[0]);
        
        // Draw loading message
        self.draw_loading_message(frame, main_layout[1]);
        
        // Draw phase
        self.draw_phase(frame, main_layout[2]);
        
        // Draw progress
        self.draw_progress(frame, main_layout[3]);
    }

    fn draw_logo(&self, frame: &mut Frame, area: Rect) {
        let logo_lines = vec![
            "╔═══╗─╔══╗─╔════╗────╔════╗─╔╗──╔╗─╔═══╗─╔═══╗",
            "║╔═╗║─╚╣╠╝─║╔╗╔╗║────║╔╗╔╗║─║╚╗╔╝║─║╔═╗║─║╔══╝",
            "║║─╚╝──║║──╚╝║║╚╝────╚╝║║╚╝─╚╗╚╝╔╝─║╚═╝║─║╚══╗",
            "║║╔═╗──║║────║║────────║║────╚╗╔╝──║╔══╝─║╔══╝",
            "║╚╩═║─╔╣╠╗───║║────────║║─────║║───║║────║╚══╗",
            "╚═══╝─╚══╝───╚╝────────╚╝─────╚╝───╚╝────╚═══╝",
        ];

        let logo_text: Vec<Line> = logo_lines.iter()
            .map(|line| Line::from(Span::styled(*line, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))))
            .collect();

        let logo = Paragraph::new(logo_text)
            .alignment(Alignment::Center);
        
        frame.render_widget(logo, area);
    }

    fn draw_loading_message(&self, frame: &mut Frame, area: Rect) {
        let loading_msg = Line::from(vec![
            Span::styled("🚀 ", Style::default().fg(Color::Yellow)),
            Span::styled("Loading...", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]);

        let loading = Paragraph::new(vec![loading_msg])
            .alignment(Alignment::Center);
        
        frame.render_widget(loading, area);
    }

    fn draw_phase(&self, frame: &mut Frame, area: Rect) {
        let current_phase = self.current_phase.lock().unwrap();
        let current_step = *self.current_step.lock().unwrap();
        
        if current_phase.is_empty() {
            return;
        }

        let phase_text = if *current_phase == "Completed" {
            "✅ Loading complete!"
        } else {
            &format!("🔄 {}/{} {}...", current_step, self.total_steps, current_phase)
        };

        let phase_line = Line::from(Span::styled(
            phase_text,
            if *current_phase == "Completed" {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Blue)
            }
        ));

        let phase_widget = Paragraph::new(vec![phase_line])
            .alignment(Alignment::Center);
        
        frame.render_widget(phase_widget, area);
    }

    fn draw_progress(&self, frame: &mut Frame, area: Rect) {
        let progress = *self.progress.lock().unwrap();
        let files_processed = *self.files_processed.lock().unwrap();
        let total_files = *self.total_files.lock().unwrap();
        let current_phase = self.current_phase.lock().unwrap();
        
        if total_files == 0 || *current_phase == "Completed" {
            return;
        }

        // Get spinner character
        let spinner_index = *self.spinner_index.lock().unwrap();
        let spinner = self.spinner_chars[spinner_index % self.spinner_chars.len()];

        let progress_text = format!("{} {:.1}% {}/{} files", 
            spinner, progress * 100.0, files_processed, total_files);

        // Progress bar
        let progress_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Progress bar
                Constraint::Length(1), // Text
            ])
            .split(area);

        // Render progress bar
        let gauge = Gauge::default()
            .block(Block::default())
            .gauge_style(Style::default().fg(Color::Green))
            .ratio(progress);
        
        frame.render_widget(gauge, progress_area[0]);

        // Render progress text
        let progress_line = Line::from(Span::styled(
            progress_text,
            Style::default().fg(Color::Green)
        ));

        let progress_widget = Paragraph::new(vec![progress_line])
            .alignment(Alignment::Center);
        
        frame.render_widget(progress_widget, progress_area[1]);
    }

    pub fn cleanup(&self) -> Result<()> {
        let mut cleaned_up = self.cleaned_up.lock().unwrap();
        if *cleaned_up {
            return Ok(());
        }
        
        disable_raw_mode()?;
        let mut stdout = stdout();
        stdout.execute(LeaveAlternateScreen)?;
        *cleaned_up = true;
        Ok(())
    }
}

impl Drop for LoadingScreen {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

impl ProgressReporter for LoadingScreen {
    fn set_phase(&self, phase: String) {
        let _ = self.update_phase(&phase);
    }
    
    fn set_progress(&self, progress: f64) {
        let processed = *self.files_processed.lock().unwrap();
        let total = *self.total_files.lock().unwrap();
        let _ = self.update_progress(progress, processed, total);
    }
    
    fn set_current_file(&self, _file: Option<String>) {
        // LoadingScreen doesn't display individual files
    }
    
    fn set_file_counts(&self, processed: usize, total: usize) {
        let progress = if total > 0 { processed as f64 / total as f64 } else { 0.0 };
        let _ = self.update_progress(progress, processed, total);
    }
    
    fn update_spinner(&self) {
        self.update_spinner();
    }
}
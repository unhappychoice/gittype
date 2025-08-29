use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use std::time::Instant;

pub struct LoadingScreen {
    start_time: Instant,
    current_phase: LoadingPhase,
    phase_progress: f64,
    overall_progress: f64,
    current_file: Option<String>,
    files_processed: usize,
    total_files: usize,
    spinner_state: usize,
}

#[derive(Clone, Debug)]
pub enum LoadingPhase {
    Initializing,
    Scanning,
    ParsingAST,
    GeneratingChallenges,
    Finalizing,
}

impl LoadingPhase {
    fn display_name(&self) -> &'static str {
        match self {
            LoadingPhase::Initializing => "Initializing",
            LoadingPhase::Scanning => "Scanning repository",
            LoadingPhase::ParsingAST => "Parsing AST",
            LoadingPhase::GeneratingChallenges => "Generating challenges",
            LoadingPhase::Finalizing => "Finalizing",
        }
    }

    fn phase_weight(&self) -> f64 {
        match self {
            LoadingPhase::Initializing => 0.1,
            LoadingPhase::Scanning => 0.2,
            LoadingPhase::ParsingAST => 0.4,
            LoadingPhase::GeneratingChallenges => 0.25,
            LoadingPhase::Finalizing => 0.05,
        }
    }
}

impl LoadingScreen {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            current_phase: LoadingPhase::Initializing,
            phase_progress: 0.0,
            overall_progress: 0.0,
            current_file: None,
            files_processed: 0,
            total_files: 0,
            spinner_state: 0,
        }
    }

    pub fn update_phase(&mut self, phase: LoadingPhase) {
        self.current_phase = phase;
        self.phase_progress = 0.0;
        self.update_overall_progress();
    }

    pub fn update_progress(&mut self, progress: f64) {
        self.phase_progress = progress.clamp(0.0, 1.0);
        self.update_overall_progress();
    }

    pub fn set_current_file(&mut self, file: Option<String>) {
        self.current_file = file;
    }

    pub fn set_file_counts(&mut self, processed: usize, total: usize) {
        self.files_processed = processed;
        self.total_files = total;
        if total > 0 {
            self.phase_progress = processed as f64 / total as f64;
            self.update_overall_progress();
        }
    }

    pub fn update_spinner(&mut self) {
        self.spinner_state = (self.spinner_state + 1) % 4;
    }

    fn update_overall_progress(&mut self) {
        let phase_weights = [
            LoadingPhase::Initializing.phase_weight(),
            LoadingPhase::Scanning.phase_weight(),
            LoadingPhase::ParsingAST.phase_weight(),
            LoadingPhase::GeneratingChallenges.phase_weight(),
            LoadingPhase::Finalizing.phase_weight(),
        ];

        let current_phase_index = match self.current_phase {
            LoadingPhase::Initializing => 0,
            LoadingPhase::Scanning => 1,
            LoadingPhase::ParsingAST => 2,
            LoadingPhase::GeneratingChallenges => 3,
            LoadingPhase::Finalizing => 4,
        };

        let completed_weight: f64 = phase_weights.iter().take(current_phase_index).sum();
        let current_phase_contribution = phase_weights[current_phase_index] * self.phase_progress;
        
        self.overall_progress = (completed_weight + current_phase_contribution).clamp(0.0, 1.0);
    }

    fn get_spinner_char(&self) -> &'static str {
        match self.spinner_state {
            0 => "⠋",
            1 => "⠙",
            2 => "⠹",
            3 => "⠸",
            _ => "⠋",
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let size = frame.size();
        
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Length(1),  // Spacer
                Constraint::Length(3),  // Overall progress
                Constraint::Length(1),  // Spacer
                Constraint::Length(3),  // Phase progress
                Constraint::Length(1),  // Spacer
                Constraint::Min(4),     // Details
                Constraint::Length(3),  // Stats
            ])
            .split(size);

        // Title
        let title = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("GitType ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Loading", Style::default().fg(Color::White)),
            ]),
        ])
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
        
        frame.render_widget(title, main_layout[0]);

        // Overall progress
        let overall_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Overall Progress"))
            .gauge_style(Style::default().fg(Color::Cyan))
            .ratio(self.overall_progress)
            .label(format!("{:.1}%", self.overall_progress * 100.0));
        
        frame.render_widget(overall_gauge, main_layout[2]);

        // Phase progress
        let phase_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Current Phase"))
            .gauge_style(Style::default().fg(Color::Green))
            .ratio(self.phase_progress)
            .label(format!("{} - {:.1}%", self.current_phase.display_name(), self.phase_progress * 100.0));
        
        frame.render_widget(phase_gauge, main_layout[4]);

        // Details
        let mut details = vec![
            Line::from(vec![
                Span::styled(self.get_spinner_char(), Style::default().fg(Color::Yellow)),
                Span::raw(" "),
                Span::styled(self.current_phase.display_name(), Style::default().fg(Color::White)),
            ]),
        ];

        if let Some(ref file) = self.current_file {
            details.push(Line::from(vec![
                Span::raw("Processing: "),
                Span::styled(file, Style::default().fg(Color::Green)),
            ]));
        }

        if self.total_files > 0 {
            details.push(Line::from(vec![
                Span::raw("Files: "),
                Span::styled(
                    format!("{} / {}", self.files_processed, self.total_files),
                    Style::default().fg(Color::Cyan)
                ),
            ]));
        }

        let details_paragraph = Paragraph::new(details)
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .alignment(Alignment::Left);
        
        frame.render_widget(details_paragraph, main_layout[6]);

        // Stats
        let elapsed = self.start_time.elapsed();
        let stats = vec![
            Line::from(vec![
                Span::raw("Elapsed: "),
                Span::styled(
                    format!("{:.1}s", elapsed.as_secs_f64()),
                    Style::default().fg(Color::Yellow)
                ),
            ]),
        ];

        let stats_paragraph = Paragraph::new(stats)
            .block(Block::default().borders(Borders::ALL).title("Statistics"))
            .alignment(Alignment::Left);
        
        frame.render_widget(stats_paragraph, main_layout[7]);
    }
}
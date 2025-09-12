use crate::game::models::loading_steps::StepType;
use crate::game::screens::loading_screen::LoadingScreenState;
use crate::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Gauge, Paragraph},
    Frame,
};
use std::sync::atomic::Ordering;

const SPINNER_CHARS: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

pub struct LoadingProgressView;

impl LoadingProgressView {
    pub fn render(frame: &mut Frame, area: Rect, state: &LoadingScreenState) {
        let current_step_type = state
            .current_step
            .read()
            .map(|x| x.clone())
            .unwrap_or(StepType::Cloning);

        if current_step_type == StepType::Completed {
            return;
        }

        // Get current step progress
        let (progress, files_processed, total_files) =
            if let Ok(step_progress) = state.step_progress.read() {
                if let Some(step_prog) = step_progress.get(&current_step_type) {
                    (step_prog.progress, step_prog.processed, step_prog.total)
                } else {
                    log::info!(
                        "UI: No progress found for {:?}, available steps: {:?}",
                        current_step_type,
                        step_progress.keys().collect::<Vec<_>>()
                    );
                    (0.0, 0, 0)
                }
            } else {
                log::warn!("UI: Failed to read step_progress");
                (0.0, 0, 0)
            };

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
                .gauge_style(Style::default().fg(Colors::SUCCESS))
                .ratio(progress.clamp(0.0, 1.0)); // Clamp progress to valid range

            frame.render_widget(gauge, progress_area[0]);
        }

        // Render progress text
        let progress_line = Line::from(Span::styled(
            progress_text,
            Style::default().fg(Colors::SUCCESS),
        ));

        let progress_widget = Paragraph::new(vec![progress_line]).alignment(Alignment::Center);

        frame.render_widget(progress_widget, progress_area[1]);
    }
}

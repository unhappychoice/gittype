use crate::presentation::game::models::StepType;
use crate::presentation::tui::screens::loading_screen::LoadingScreenState;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct LoadingDescriptionView;

impl LoadingDescriptionView {
    pub fn render(frame: &mut Frame, area: Rect, state: &LoadingScreenState, colors: &Colors) {
        let current_step_type = state
            .current_step
            .read()
            .map(|x| x.clone())
            .unwrap_or(StepType::Cloning);

        let mut description_lines = vec![
            Line::from(Span::styled(
                "Analyzing your repository to create typing challenges...",
                Style::default().fg(colors.text_secondary()),
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
                    ("✓", colors.success())
                } else if is_current {
                    ("⚡", colors.warning())
                } else {
                    ("○", colors.text_secondary())
                };

                description_lines.push(Line::from(vec![
                    Span::styled(format!("{} ", icon), Style::default().fg(color)),
                    Span::styled(
                        step_info.description.clone(),
                        Style::default().fg(colors.text_secondary()),
                    ),
                ]));
            }
        }

        let description_paragraph = Paragraph::new(description_lines).alignment(Alignment::Center);

        frame.render_widget(description_paragraph, area);
    }
}

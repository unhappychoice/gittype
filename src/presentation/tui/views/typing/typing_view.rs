use super::{
    TypingContentView, TypingCountdownView, TypingDialogView, TypingFooterView, TypingHeaderView,
};
use crate::domain::models::typing::CodeContext;
use crate::domain::models::{Challenge, GitRepository};
use crate::domain::services::typing_core::TypingCore;
use crate::domain::services::SessionManager;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct TypingView {
    content_view: TypingContentView,
}

impl Default for TypingView {
    fn default() -> Self {
        Self::new()
    }
}
impl TypingView {
    pub fn new() -> Self {
        Self {
            content_view: TypingContentView::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &mut self,
        frame: &mut Frame,
        challenge: Option<&Challenge>,
        git_repository: Option<&GitRepository>,
        typing_core: &TypingCore,
        chars: &[char],
        code_context: &CodeContext,
        waiting_to_start: bool,
        countdown_number: Option<u8>,
        skips_remaining: usize,
        dialog_shown: bool,
        session_manager: &std::sync::Arc<dyn crate::domain::services::session_manager_service::SessionManagerInterface>,
        colors: &Colors,
    ) {
        let countdown_active = countdown_number.is_some();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(frame.area());

        // Header
        TypingHeaderView::render(frame, chunks[0], challenge, git_repository, colors);

        // Content
        let show_code = !(waiting_to_start || countdown_active);
        self.content_view.render(
            frame,
            chunks[1],
            show_code,
            challenge,
            typing_core,
            chars,
            code_context,
            colors,
        );

        // Metrics
        if let Some(instance) = session_manager.as_any().downcast_ref::<SessionManager>() {
            if let Some(stage_tracker) = instance.get_current_stage_tracker() {
                TypingFooterView::render_metrics(
                    frame,
                    chunks[2],
                    waiting_to_start,
                    countdown_active,
                    skips_remaining,
                    &stage_tracker,
                    typing_core,
                    colors,
                );
            }
        }

        // Progress bar
        TypingFooterView::render_progress(
            frame,
            chunks[3],
            waiting_to_start,
            countdown_active,
            typing_core,
            typing_core.text_to_display().chars().count(),
            colors,
        );

        // ESC Options
        let esc_area = ratatui::layout::Rect {
            x: 1,
            y: frame.area().height.saturating_sub(1),
            width: 15,
            height: 1,
        };
        let esc_text = Paragraph::new(vec![Line::from(vec![
            Span::styled("[ESC]", Style::default().fg(colors.key_action())),
            Span::styled(" Options", Style::default().fg(colors.text())),
        ])]);
        frame.render_widget(esc_text, esc_area);

        // Center messages and countdown
        let center_x = frame.area().width / 2;
        let center_y = frame.area().height / 2;

        if waiting_to_start {
            let start_line = vec![
                Span::styled("Press ", Style::default().fg(colors.text())),
                Span::styled(
                    "[SPACE]",
                    Style::default()
                        .fg(colors.success())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" to start", Style::default().fg(colors.text())),
            ];

            let total_width = "Press [SPACE] to start".len() as u16;
            let start_area = ratatui::layout::Rect {
                x: center_x.saturating_sub(total_width / 2),
                y: center_y,
                width: total_width,
                height: 1,
            };
            let start_text = Paragraph::new(vec![Line::from(start_line)]);
            frame.render_widget(start_text, start_area);
        } else if let Some(count) = countdown_number {
            TypingCountdownView::render(frame, count, colors);
        }

        // Dialog
        if dialog_shown {
            TypingDialogView::render(frame, skips_remaining, colors);
        }
    }
}

use crate::application::service::analytics_service::AnalyticsData;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    Frame,
};

pub struct TrendsView;

impl TrendsView {
    pub fn render(f: &mut Frame, area: Rect, data: &AnalyticsData) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), // CPM trend
                Constraint::Percentage(50), // Accuracy trend (placeholder)
            ])
            .split(area);

        Self::render_cpm_trend(f, chunks[0], data);
        Self::render_accuracy_trend(f, chunks[1], data);
    }

    fn render_cpm_trend(f: &mut Frame, area: Rect, data: &AnalyticsData) {
        if data.cpm_trend.is_empty() {
            let empty_msg = Paragraph::new(vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::raw(
                        "No trend data available - keep typing to build your performance history!",
                    ),
                ]),
            ])
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("CPM Trend"),
            );
            f.render_widget(empty_msg, area);
            return;
        }

        // Convert trend data to chart points
        let mut chart_data: Vec<(f64, f64)> = Vec::new();
        for (i, (_date, cpm)) in data.cpm_trend.iter().enumerate() {
            chart_data.push((i as f64, *cpm));
        }

        // Calculate bounds
        let max_cpm = data
            .cpm_trend
            .iter()
            .map(|(_, cpm)| *cpm)
            .fold(0.0, f64::max);
        let min_cpm = data
            .cpm_trend
            .iter()
            .map(|(_, cpm)| *cpm)
            .fold(max_cpm, f64::min);
        let cpm_range = (max_cpm - min_cpm).max(10.0); // Minimum range of 10

        let datasets = vec![Dataset::default()
            .name("CPM")
            .marker(ratatui::symbols::Marker::Braille)
            .style(Style::default().fg(Colors::cpm_wpm()))
            .graph_type(GraphType::Line)
            .data(&chart_data)];

        // Create x-axis labels from dates
        let x_labels: Vec<Span> = data
            .cpm_trend
            .iter()
            .step_by((data.cpm_trend.len().max(1) / 8).max(1)) // Show ~8 labels max
            .map(|(date, _)| {
                let day = if date.len() >= 5 { &date[3..] } else { date };
                Span::styled(day, Style::default().fg(Colors::text()))
            })
            .collect();

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("CPM Performance Trend"),
            )
            .x_axis(
                Axis::default()
                    .title("Date")
                    .style(Style::default().fg(Colors::text_secondary()))
                    .labels(x_labels)
                    .bounds([0.0, (data.cpm_trend.len().saturating_sub(1)) as f64]),
            )
            .y_axis(
                Axis::default()
                    .title("CPM")
                    .style(Style::default().fg(Colors::text_secondary()))
                    .bounds([min_cpm - cpm_range * 0.1, max_cpm + cpm_range * 0.1])
                    .labels(vec![
                        Span::styled(
                            format!("{:.0}", min_cpm),
                            Style::default().fg(Colors::text()),
                        ),
                        Span::styled(
                            format!("{:.0}", (min_cpm + max_cpm) / 2.0),
                            Style::default().fg(Colors::text()),
                        ),
                        Span::styled(
                            format!("{:.0}", max_cpm),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
            );

        f.render_widget(chart, area);
    }

    fn render_accuracy_trend(f: &mut Frame, area: Rect, data: &AnalyticsData) {
        if data.accuracy_trend.is_empty() {
            let empty_msg = Paragraph::new(vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::raw("No accuracy trend data available - keep typing to build your accuracy history!"),
                ]),
            ])
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("Accuracy Trend"),
            );
            f.render_widget(empty_msg, area);
            return;
        }

        // Convert trend data to chart points
        let mut chart_data: Vec<(f64, f64)> = Vec::new();
        for (i, (_date, accuracy)) in data.accuracy_trend.iter().enumerate() {
            chart_data.push((i as f64, *accuracy));
        }

        // Calculate bounds for accuracy (should be between 0-100)
        let max_accuracy = data
            .accuracy_trend
            .iter()
            .map(|(_, acc)| *acc)
            .fold(0.0, f64::max);
        let min_accuracy = data
            .accuracy_trend
            .iter()
            .map(|(_, acc)| *acc)
            .fold(max_accuracy, f64::min);
        let accuracy_range = (max_accuracy - min_accuracy).max(10.0);

        let datasets = vec![Dataset::default()
            .name("Accuracy")
            .marker(ratatui::symbols::Marker::Braille)
            .style(Style::default().fg(Colors::accuracy()))
            .graph_type(GraphType::Line)
            .data(&chart_data)];

        // Create x-axis labels from dates
        let x_labels: Vec<Span> = data
            .accuracy_trend
            .iter()
            .step_by((data.accuracy_trend.len().max(1) / 8).max(1))
            .map(|(date, _)| {
                let day = if date.len() >= 5 { &date[3..] } else { date };
                Span::styled(day, Style::default().fg(Colors::text()))
            })
            .collect();

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("Accuracy Performance Trend"),
            )
            .x_axis(
                Axis::default()
                    .title("Date")
                    .style(Style::default().fg(Colors::text_secondary()))
                    .labels(x_labels)
                    .bounds([0.0, (data.accuracy_trend.len().saturating_sub(1)) as f64]),
            )
            .y_axis(
                Axis::default()
                    .title("Accuracy (%)")
                    .style(Style::default().fg(Colors::text_secondary()))
                    .bounds([
                        min_accuracy - accuracy_range * 0.1,
                        max_accuracy + accuracy_range * 0.1,
                    ])
                    .labels(vec![
                        Span::styled(
                            format!("{:.1}%", min_accuracy),
                            Style::default().fg(Colors::text()),
                        ),
                        Span::styled(
                            format!("{:.1}%", (min_accuracy + max_accuracy) / 2.0),
                            Style::default().fg(Colors::text()),
                        ),
                        Span::styled(
                            format!("{:.1}%", max_accuracy),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
            );

        f.render_widget(chart, area);
    }
}

use crate::storage::repositories::session_repository::SessionRepository;
use crate::ui::Colors;
use crate::{models::GitRepository, Result};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub struct DetailsDialog;

impl DetailsDialog {
    pub fn show_details(
        session_result: &crate::models::SessionResult,
        repo_info: &Option<GitRepository>,
    ) -> Result<()> {
        let mut stdout = stdout();

        // Clear screen
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, ResetColor)?;

        let (terminal_width, _terminal_height) = terminal::size()?;
        let center_col = terminal_width / 2;
        let mut current_row = 3u16;

        // Dialog title
        let title = "=== SESSION DETAILS ===";
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, current_row))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
        )?;
        execute!(stdout, Print(title))?;
        execute!(stdout, ResetColor)?;
        current_row += 3;

        // Best Records Section
        if let Ok(Some(best_records)) = SessionRepository::get_best_records_global() {
            let best_records_title = "BEST RECORDS";
            let title_col = center_col.saturating_sub(best_records_title.len() as u16 / 2);
            execute!(stdout, MoveTo(title_col, current_row))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Yellow)
            )?;
            execute!(stdout, Print(best_records_title))?;
            execute!(stdout, ResetColor)?;
            current_row += 2;

            let records = [
                ("Today's Best", &best_records.todays_best),
                ("Weekly Best", &best_records.weekly_best),
                ("All time Best", &best_records.all_time_best),
            ];

            // Find the longest label for alignment
            let max_label_width = records
                .iter()
                .map(|(label, _)| label.len())
                .max()
                .unwrap_or(0);

            for (label, record_data) in records.iter() {
                if let Some(record) = record_data {
                    let is_new_pb = session_result.session_score > record.score;

                    // Create aligned record line with proper colors
                    let record_content = format!(
                        "Score {:.0} | CPM {:.0} | Acc {:.1}%",
                        record.score, record.cpm, record.accuracy
                    );

                    let prefix = if is_new_pb { "*** NEW PB! " } else { "" };

                    let full_line = format!(
                        "{}{:>width$}: {}",
                        prefix,
                        label,
                        record_content,
                        width = max_label_width
                    );

                    let line_col = center_col.saturating_sub(full_line.len() as u16 / 2);
                    execute!(stdout, MoveTo(line_col, current_row))?;

                    if is_new_pb {
                        execute!(
                            stdout,
                            SetForegroundColor(Colors::to_crossterm(Colors::WARNING))
                        )?;
                        execute!(stdout, Print("*** NEW PB! "))?;
                        execute!(
                            stdout,
                            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
                        )?;
                        execute!(
                            stdout,
                            Print(&format!("{:>width$}: ", label, width = max_label_width))
                        )?;
                    } else {
                        execute!(
                            stdout,
                            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
                        )?;
                        execute!(
                            stdout,
                            Print(&format!("{:>width$}: ", label, width = max_label_width))
                        )?;
                    }

                    // Score with color
                    execute!(
                        stdout,
                        SetForegroundColor(Colors::to_crossterm(Colors::SCORE))
                    )?;
                    execute!(stdout, Print("Score "))?;
                    execute!(
                        stdout,
                        SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
                    )?;
                    execute!(stdout, Print(format!("{:.0}", record.score)))?;
                    execute!(stdout, Print(" | "))?;

                    // CPM with color
                    execute!(
                        stdout,
                        SetForegroundColor(Colors::to_crossterm(Colors::CPM_WPM))
                    )?;
                    execute!(stdout, Print("CPM "))?;
                    execute!(
                        stdout,
                        SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
                    )?;
                    execute!(stdout, Print(format!("{:.0}", record.cpm)))?;
                    execute!(stdout, Print(" | "))?;

                    // Accuracy with color
                    execute!(
                        stdout,
                        SetForegroundColor(Colors::to_crossterm(Colors::ACCURACY))
                    )?;
                    execute!(stdout, Print("Acc "))?;
                    execute!(
                        stdout,
                        SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
                    )?;
                    execute!(stdout, Print(format!("{:.1}%", record.accuracy)))?;

                    // Always show score difference
                    let diff = session_result.session_score - record.score;
                    if diff > 0.0 {
                        execute!(
                            stdout,
                            SetForegroundColor(Colors::to_crossterm(Colors::SUCCESS))
                        )?;
                        execute!(stdout, Print(&format!(" (+{:.0})", diff)))?;
                    } else if diff < 0.0 {
                        execute!(
                            stdout,
                            SetForegroundColor(Colors::to_crossterm(Colors::ERROR))
                        )?;
                        execute!(stdout, Print(&format!(" ({:.0})", diff)))?; // diff is already negative
                    }

                    execute!(stdout, ResetColor)?;
                    current_row += 1;
                } else {
                    let no_record_line = format!(
                        "{:>width$}: No previous record",
                        label,
                        width = max_label_width
                    );
                    let line_col = center_col.saturating_sub(no_record_line.len() as u16 / 2);
                    execute!(stdout, MoveTo(line_col, current_row))?;
                    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
                    execute!(stdout, Print(&no_record_line))?;
                    execute!(stdout, ResetColor)?;
                    current_row += 1;
                }
            }
            current_row += 2;
        }

        // Stage Results Section
        if !session_result.stage_results.is_empty() {
            let stage_label = if let Some(repo) = repo_info {
                format!(
                    "Stage Results: [{}/{}]",
                    repo.user_name, repo.repository_name
                )
            } else {
                "Stage Results:".to_string()
            };
            let stage_label_col = center_col.saturating_sub(stage_label.len() as u16 / 2);
            execute!(stdout, MoveTo(stage_label_col, current_row))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Cyan)
            )?;
            execute!(stdout, Print(&stage_label))?;
            execute!(stdout, ResetColor)?;
            current_row += 2;

            // Calculate maximum stage name width for alignment
            let max_stage_name_width = session_result
                .stage_results
                .iter()
                .enumerate()
                .map(|(i, stage)| {
                    if !stage.challenge_path.is_empty() {
                        stage.challenge_path.len()
                    } else {
                        format!("Stage {}", i + 1).len()
                    }
                })
                .max()
                .unwrap_or(20);

            for (i, stage_result) in session_result.stage_results.iter().enumerate() {
                let stage_name = if !stage_result.challenge_path.is_empty() {
                    stage_result.challenge_path.clone()
                } else {
                    format!("Stage {}", i + 1)
                };

                let stage_content = format!(
                    "Score {:.0} | CPM {:.0} | Acc {:.1}%",
                    stage_result.challenge_score, stage_result.cpm, stage_result.accuracy
                );
                let result_line = format!(
                    "{:>width$}: {}",
                    stage_name,
                    stage_content,
                    width = max_stage_name_width
                );
                let result_col = center_col.saturating_sub(result_line.len() as u16 / 2);
                execute!(stdout, MoveTo(result_col, current_row))?;
                execute!(
                    stdout,
                    SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
                )?;
                execute!(
                    stdout,
                    Print(&format!(
                        "{:>width$}: ",
                        stage_name,
                        width = max_stage_name_width
                    ))
                )?;

                // Score with color
                execute!(
                    stdout,
                    SetForegroundColor(Colors::to_crossterm(Colors::SCORE))
                )?;
                execute!(stdout, Print("Score "))?;
                execute!(
                    stdout,
                    SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
                )?;
                execute!(
                    stdout,
                    Print(format!("{:.0}", stage_result.challenge_score))
                )?;
                execute!(stdout, Print(" | "))?;

                // CPM with color
                execute!(
                    stdout,
                    SetForegroundColor(Colors::to_crossterm(Colors::CPM_WPM))
                )?;
                execute!(stdout, Print("CPM "))?;
                execute!(
                    stdout,
                    SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
                )?;
                execute!(stdout, Print(format!("{:.0}", stage_result.cpm)))?;
                execute!(stdout, Print(" | "))?;

                // Accuracy with color
                execute!(
                    stdout,
                    SetForegroundColor(Colors::to_crossterm(Colors::ACCURACY))
                )?;
                execute!(stdout, Print("Acc "))?;
                execute!(
                    stdout,
                    SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
                )?;
                execute!(stdout, Print(format!("{:.1}%", stage_result.accuracy)))?;
                execute!(stdout, ResetColor)?;
                current_row += 1;
            }
            current_row += 2;
        }

        // Instructions
        let instruction = "[ESC] Return";
        let instruction_col = center_col.saturating_sub(instruction.len() as u16 / 2);
        execute!(stdout, MoveTo(instruction_col, current_row))?;
        execute!(stdout, SetForegroundColor(Color::Red))?;
        execute!(stdout, Print("[ESC]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Return"))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;

        // Disable cursor visibility
        execute!(stdout, Hide)?;

        // Wait for ESC key only
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.code == KeyCode::Esc {
                        break;
                    }
                }
            }
        }

        // Restore cursor visibility
        execute!(stdout, Show)?;

        Ok(())
    }
}

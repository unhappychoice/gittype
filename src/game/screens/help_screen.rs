use crate::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::models::rank::{Rank, RankTier};
use crate::ui::Colors;
use crate::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Tabs, Wrap,
    },
    Frame,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HelpSection {
    Scoring,
    Ranks,
    GameHelp,
    CLI,
    About,
    Community,
}

impl HelpSection {
    pub fn title(&self) -> &'static str {
        match self {
            HelpSection::Scoring => "Scoring System",
            HelpSection::Ranks => "Rank System",
            HelpSection::GameHelp => "Game Help",
            HelpSection::CLI => "CLI Usage",
            HelpSection::About => "About & Credits",
            HelpSection::Community => "Community",
        }
    }

    pub fn all() -> Vec<HelpSection> {
        vec![
            HelpSection::CLI,
            HelpSection::Scoring,
            HelpSection::Ranks,
            HelpSection::GameHelp,
            HelpSection::About,
            HelpSection::Community,
        ]
    }
}

pub struct HelpScreen {
    current_section: HelpSection,
    github_fallback: Option<String>,
    scroll_position: u16,
    content_height: u16,
    viewport_height: u16,
}

impl Default for HelpScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl HelpScreen {
    pub fn new() -> Self {
        Self {
            current_section: HelpSection::CLI,
            github_fallback: None,
            scroll_position: 0,
            content_height: 0,
            viewport_height: 0,
        }
    }

    fn get_scoring_content() -> Text<'static> {
        Text::from(vec![
            Line::from(vec![Span::styled(
                "Score Calculation Formula:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("Base Score = CPM × (Accuracy / 100) × 10"),
            Line::from(""),
            Line::from(vec![
                Span::styled("CPM", Style::default().fg(Colors::cpm_wpm())),
                Span::styled(
                    " (Characters Per Minute): ",
                    Style::default().fg(Colors::text()),
                ),
                Span::styled(
                    "Total characters typed / minutes",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled("WPM", Style::default().fg(Colors::cpm_wpm())),
                Span::styled(" (Words Per Minute): ", Style::default().fg(Colors::text())),
                Span::styled(
                    "CPM / 5 (average word length)",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Accuracy", Style::default().fg(Colors::accuracy())),
                Span::styled(": ", Style::default().fg(Colors::text())),
                Span::styled(
                    "(Total chars - Mistakes) / Total chars × 100%",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Bonuses & Penalties:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("• Consistency Bonus: Up to 70% extra for high accuracy"),
            Line::from("• Time Bonus: Extra points for fast completion"),
            Line::from("• Mistake Penalty: -5 points per error"),
            Line::from(""),
            Line::from("Final Score = (Base + Consistency + Time - Penalties) × 2 + 100"),
        ])
    }

    fn get_ranks_content() -> Text<'static> {
        let ranks = Rank::all_ranks();
        let mut lines = vec![
            Line::from(vec![Span::styled(
                "Rank Tiers:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
        ];

        let tiers = [
            (RankTier::Beginner, "Fresh developers learning the basics"),
            (
                RankTier::Intermediate,
                "Solid developers with growing skills",
            ),
            (
                RankTier::Advanced,
                "Senior developers mastering their craft",
            ),
            (RankTier::Expert, "Elite developers becoming legendary"),
            (
                RankTier::Legendary,
                "Mythical entities beyond comprehension",
            ),
        ];

        for (tier, description) in tiers.iter() {
            let tier_ranks: Vec<&Rank> = ranks.iter().filter(|r| r.tier() == tier).collect();
            let (min_score, max_score) =
                if let (Some(first), Some(last)) = (tier_ranks.first(), tier_ranks.last()) {
                    (first.min_score, last.max_score)
                } else {
                    continue;
                };

            lines.push(Line::from(vec![
                Span::styled(
                    format!("{:?}", tier),
                    Style::default()
                        .fg(Colors::from_crossterm(tier.terminal_color()))
                        .bold(),
                ),
                Span::styled(
                    format!(
                        " ({}-{})",
                        min_score,
                        if max_score == u32::MAX {
                            "∞".to_string()
                        } else {
                            max_score.to_string()
                        }
                    ),
                    Style::default().fg(Colors::secondary()),
                ),
            ]));
            lines.push(Line::from(vec![Span::styled(
                format!("  {}", description),
                Style::default().fg(Colors::text()),
            )]));
            lines.push(Line::from(""));
        }

        lines.push(Line::from(vec![Span::styled(
            "All Ranks:",
            Style::default().fg(Colors::title()).bold(),
        )]));
        lines.push(Line::from(""));

        for tier in [
            RankTier::Beginner,
            RankTier::Intermediate,
            RankTier::Advanced,
            RankTier::Expert,
            RankTier::Legendary,
        ]
        .iter()
        {
            let tier_ranks: Vec<_> = ranks.iter().filter(|r| r.tier() == tier).collect();

            // Show tier header
            lines.push(Line::from(vec![Span::styled(
                format!("{:?}:", tier),
                Style::default()
                    .fg(Colors::from_crossterm(tier.terminal_color()))
                    .bold(),
            )]));

            // Show all ranks in this tier
            for rank in tier_ranks.iter() {
                // Hide ranks above Origin (14501+) with ???
                let rank_name = if rank.min_score > 14500 {
                    "???"
                } else {
                    rank.name()
                };

                let score_range = if rank.max_score == u32::MAX {
                    format!("{}+", rank.min_score)
                } else {
                    format!("{}-{}", rank.min_score, rank.max_score)
                };

                // Hide score ranges for mysterious ranks
                let display_score = if rank.min_score > 14500 {
                    "???".to_string()
                } else {
                    score_range
                };

                lines.push(Line::from(vec![
                    Span::styled("  • ", Style::default().fg(Colors::secondary())),
                    Span::styled(
                        rank_name.to_string(),
                        Style::default().fg(Colors::from_crossterm(rank.terminal_color())),
                    ),
                    Span::styled(
                        format!(" ({})", display_score),
                        Style::default().fg(Colors::secondary()),
                    ),
                ]));
            }
            lines.push(Line::from(""));
        }

        lines.push(Line::from(vec![
            Span::styled("Note:", Style::default().fg(Colors::info())),
            Span::styled(
                " The highest rank remains mysterious until achieved!",
                Style::default().fg(Colors::text()),
            ),
        ]));

        Text::from(lines)
    }

    fn get_game_help_content() -> Text<'static> {
        Text::from(vec![
            Line::from(vec![Span::styled(
                "Game Modes:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("• Standard: Type code from popular repositories"),
            Line::from("• Difficulty: Choose Easy, Normal, Hard, Wild, or Zen"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Typing Tips:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("• Focus on accuracy over speed initially"),
            Line::from("• Use proper finger positioning"),
            Line::from("• Practice regularly to improve muscle memory"),
            Line::from("• Don't look at the keyboard while typing"),
            Line::from("• Take breaks to avoid fatigue"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Code Challenge Types:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("GitType extracts real code constructs from repositories:"),
            Line::from(""),
            Line::from("• Functions, methods, and procedures"),
            Line::from("• Classes, structs, and interfaces"),
            Line::from("• Enums, traits, and type definitions"),
            Line::from("• Variables, constants, and modules"),
            Line::from("• React components and namespaces"),
            Line::from("• Control flow (loops, conditionals)"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Advanced Typing Tips:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("• Use simultaneous key presses for efficiency:"),
            Line::from("  - For 'knock': press 'kno' with right hand almost simultaneously,"),
            Line::from("    then 'ck' together"),
            Line::from("  - Practice common letter combinations as single motions"),
            Line::from(""),
            Line::from("• Master Shift key timing:"),
            Line::from("  - Press Shift slightly before the target letter"),
            Line::from("  - Use the opposite hand's Shift when possible"),
            Line::from("  - Release Shift immediately after the letter"),
            Line::from(""),
            Line::from("• Optimize hand movement:"),
            Line::from("  - Keep wrists straight and hands relaxed"),
            Line::from("  - Use minimal finger movement"),
            Line::from("  - Practice chord-like movements for common patterns"),
            Line::from(""),
            Line::from("• Code-specific techniques:"),
            Line::from("  - Learn bracket/brace patterns as single motions"),
            Line::from("  - Practice common variable naming conventions"),
            Line::from("  - Master punctuation placement without looking"),
        ])
    }

    fn get_cli_content() -> Text<'static> {
        Text::from(vec![
            Line::from(vec![Span::styled(
                "Basic Usage:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype"),
                    Style::default().fg(Colors::text()),
                ),
                Span::styled(
                    "# Start with current directory",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype /path/to/repo"),
                    Style::default().fg(Colors::text()),
                ),
                Span::styled(
                    "# Use specific repository path",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype --repo owner/repo"),
                    Style::default().fg(Colors::text()),
                ),
                Span::styled(
                    "# Clone and use GitHub repository",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype --langs rust,python"),
                    Style::default().fg(Colors::text()),
                ),
                Span::styled(
                    "# Filter by programming languages",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Repository Commands:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype repo list"),
                    Style::default().fg(Colors::text()),
                ),
                Span::styled(
                    "# List all cached repositories",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype repo clear"),
                    Style::default().fg(Colors::text()),
                ),
                Span::styled(
                    "# Clear all cached repositories",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype repo clear --force"),
                    Style::default().fg(Colors::text()),
                ),
                Span::styled(
                    "# Force clear without confirmation",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype repo play"),
                    Style::default().fg(Colors::text()),
                ),
                Span::styled(
                    "# Play a cached repository interactively",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Cache Management:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype cache stats"),
                    Style::default().fg(Colors::text()),
                ),
                Span::styled(
                    "# Show cache statistics",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype cache clear"),
                    Style::default().fg(Colors::text()),
                ),
                Span::styled(
                    "# Clear all cached challenges",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype cache list"),
                    Style::default().fg(Colors::text()),
                ),
                Span::styled(
                    "# List cached repository keys",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Cache Locations:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "~/.gittype/"),
                    Style::default().fg(Colors::info()),
                ),
                Span::styled(
                    "# Main cache directory",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "~/.gittype/repos/"),
                    Style::default().fg(Colors::info()),
                ),
                Span::styled(
                    "# Repository data cache",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "~/.gittype/cache/"),
                    Style::default().fg(Colors::info()),
                ),
                Span::styled("# Challenge cache", Style::default().fg(Colors::secondary())),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "~/.gittype/gittype.db"),
                    Style::default().fg(Colors::info()),
                ),
                Span::styled(
                    "# Session history database",
                    Style::default().fg(Colors::secondary()),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Examples:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "gittype --repo rust-lang/rust",
                Style::default().fg(Colors::text()),
            )]),
            Line::from(vec![Span::styled(
                "gittype --repo facebook/react",
                Style::default().fg(Colors::text()),
            )]),
            Line::from(vec![Span::styled(
                "gittype --repo microsoft/vscode",
                Style::default().fg(Colors::text()),
            )]),
            Line::from(vec![Span::styled(
                "gittype --langs rust,typescript,python",
                Style::default().fg(Colors::text()),
            )]),
        ])
    }

    fn get_about_content() -> Text<'static> {
        Text::from(vec![
            Line::from(vec![Span::styled(
                "GitType",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("A CLI code-typing game that turns your source code into typing challenges"),
            Line::from(""),
            Line::from("Practice typing with your own code repositories -"),
            Line::from("improve your speed and accuracy while working with"),
            Line::from("real functions, classes, and methods from your actual projects."),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Development Team:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "• Creator & Lead Developer: ",
                    Style::default().fg(Colors::text()),
                ),
                Span::styled("unhappychoice", Style::default().fg(Colors::success())),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Special Thanks:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("• All open-source repository maintainers"),
            Line::from("• The Rust community for excellent tooling"),
            Line::from("• Tree-sitter for code parsing capabilities"),
            Line::from("• Ratatui for terminal UI framework"),
            Line::from("• All contributors and users providing feedback"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Built with:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("• Rust - Systems programming language"),
            Line::from("• Ratatui - Terminal user interface library"),
            Line::from("• Tree-sitter - Code parsing and syntax highlighting"),
            Line::from("• SQLite - Local data storage"),
            Line::from("• Git2 - Repository cloning and management"),
        ])
    }

    fn get_community_content() -> Text<'static> {
        Text::from(vec![
            Line::from(vec![Span::styled(
                "Join the Community!",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "GitHub Repository:",
                Style::default().fg(Colors::success()).bold(),
            )]),
            Line::from("https://github.com/unhappychoice/gittype"),
            Line::from(""),
            Line::from("⭐ Star the repository if you enjoy GitType!"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Contributing:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("• Report bugs and suggest features via GitHub Issues"),
            Line::from("• Submit pull requests for improvements"),
            Line::from("• Add support for new programming languages"),
            Line::from("• Improve code extraction algorithms"),
            Line::from("• Enhance UI/UX design"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Bug Reporting:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("When reporting bugs, please include:"),
            Line::from("• Operating system and terminal details"),
            Line::from("• Steps to reproduce the issue"),
            Line::from("• Expected vs actual behavior"),
            Line::from("• Any error messages or logs"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Social Media:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("Share your progress with #gittype"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "License:",
                Style::default().fg(Colors::title()).bold(),
            )]),
            Line::from(""),
            Line::from("GitType is open-source software."),
            Line::from("Check the LICENSE file for details."),
        ])
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let sections = HelpSection::all();
        let titles: Vec<Line> = sections
            .iter()
            .map(|section| Line::from(section.title()))
            .collect();

        let selected_index = sections
            .iter()
            .position(|&s| s == self.current_section)
            .unwrap_or(0);

        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("Help"),
            )
            .highlight_style(Style::default().fg(Colors::highlight()).bold())
            .select(selected_index);

        frame.render_widget(tabs, area);
    }

    fn render_content(&mut self, frame: &mut Frame, area: Rect) {
        let content = match self.current_section {
            HelpSection::Scoring => Self::get_scoring_content(),
            HelpSection::Ranks => Self::get_ranks_content(),
            HelpSection::GameHelp => Self::get_game_help_content(),
            HelpSection::CLI => Self::get_cli_content(),
            HelpSection::About => Self::get_about_content(),
            HelpSection::Community => Self::get_community_content(),
        };

        // Update viewport and content height for scrolling
        self.viewport_height = area.height.saturating_sub(2); // Account for borders
        self.content_height = content.lines.len() as u16;

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .padding(Padding::horizontal(2)),
            )
            .wrap(Wrap { trim: true })
            .scroll((self.scroll_position, 0));

        frame.render_widget(paragraph, area);

        // Render scrollbar if content is longer than viewport
        if self.content_height > self.viewport_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
            let mut scrollbar_state = ScrollbarState::new(
                self.content_height.saturating_sub(self.viewport_height) as usize,
            )
            .position(self.scroll_position as usize);

            frame.render_stateful_widget(
                scrollbar,
                area.inner(Margin {
                    vertical: 1,
                    horizontal: 0,
                }), // Inside the border
                &mut scrollbar_state,
            );
        }
    }

    fn render_github_fallback(&self, frame: &mut Frame, url: &str) {
        let width = std::cmp::max(60, url.len() + 4) as u16;
        let area = Self::centered_rect(width, 8, frame.area());

        frame.render_widget(Clear, area);

        let block = Block::default()
            .title("Cannot open GitHub")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Colors::error()));

        frame.render_widget(block, area);

        let inner = area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        });

        let message = Paragraph::new("Please copy and paste the URL below:")
            .style(Style::default().fg(Colors::warning()))
            .alignment(Alignment::Center);

        let message_area = Rect {
            x: inner.x,
            y: inner.y + 1,
            width: inner.width,
            height: 1,
        };

        frame.render_widget(message, message_area);

        let url_para = Paragraph::new(url)
            .style(Style::default().fg(Colors::info()).bold())
            .alignment(Alignment::Center);

        let url_area = Rect {
            x: inner.x,
            y: inner.y + 2,
            width: inner.width,
            height: 1,
        };

        frame.render_widget(url_para, url_area);

        let back_instructions = vec![
            Span::styled("[ESC]", Style::default().fg(Colors::success())),
            Span::styled(" Back", Style::default().fg(Colors::text())),
        ];

        let back_para = Paragraph::new(Line::from(back_instructions)).alignment(Alignment::Center);

        let back_area = Rect {
            x: inner.x,
            y: inner.y + 4,
            width: inner.width,
            height: 1,
        };

        frame.render_widget(back_para, back_area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Empty line
                Constraint::Length(1), // Star message
                Constraint::Length(1), // Empty line
                Constraint::Length(1), // Instructions
            ])
            .split(area);

        // Star message
        let star_message = vec![
            Span::styled("★ ", Style::default().fg(Colors::warning())),
            Span::styled("Star us on GitHub (", Style::default().fg(Colors::text())),
            Span::styled(
                "https://github.com/unhappychoice/gittype",
                Style::default().fg(Colors::secondary()),
            ),
            Span::styled(
                ") if you enjoy GitType! ",
                Style::default().fg(Colors::text()),
            ),
            Span::styled("★", Style::default().fg(Colors::warning())),
        ];
        let star_para = Paragraph::new(Line::from(star_message)).alignment(Alignment::Center);
        frame.render_widget(star_para, chunks[1]);

        // Instructions
        let instructions = vec![
            Span::styled("[←→/HL]", Style::default().fg(Colors::info())),
            Span::styled(" Switch tabs ", Style::default().fg(Colors::text())),
            Span::styled("[↑↓/JK]", Style::default().fg(Colors::info())),
            Span::styled(" Scroll ", Style::default().fg(Colors::text())),
            Span::styled("[G]", Style::default().fg(Colors::success())),
            Span::styled(" GitHub ", Style::default().fg(Colors::text())),
            Span::styled("[ESC]", Style::default().fg(Colors::error())),
            Span::styled(" Close", Style::default().fg(Colors::text())),
        ];
        let instructions_para =
            Paragraph::new(Line::from(instructions)).alignment(Alignment::Center);
        frame.render_widget(instructions_para, chunks[3]);
    }

    fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length((r.height.saturating_sub(height)) / 2),
                Constraint::Length(height),
                Constraint::Length((r.height.saturating_sub(height)) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((r.width.saturating_sub(width)) / 2),
                Constraint::Length(width),
                Constraint::Length((r.width.saturating_sub(width)) / 2),
            ])
            .split(popup_layout[1])[1]
    }

    fn try_open_github() -> Result<bool> {
        let url = "https://github.com/unhappychoice/gittype";
        Ok(open::that(url).is_ok())
    }
}

impl Screen for HelpScreen {
    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> crate::Result<ScreenTransition> {
        if self.github_fallback.is_some() {
            match key_event.code {
                KeyCode::Esc => {
                    self.github_fallback = None;
                    Ok(ScreenTransition::None)
                }
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    Ok(ScreenTransition::Exit)
                }
                _ => Ok(ScreenTransition::None),
            }
        } else {
            let sections = HelpSection::all();
            let current_index = sections
                .iter()
                .position(|&s| s == self.current_section)
                .unwrap_or(0);

            match key_event.code {
                KeyCode::Left | KeyCode::Char('h') => {
                    let new_index = if current_index == 0 {
                        sections.len() - 1
                    } else {
                        current_index - 1
                    };
                    self.current_section = sections[new_index];
                    self.scroll_position = 0; // Reset scroll when changing sections
                    Ok(ScreenTransition::None)
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    let new_index = (current_index + 1) % sections.len();
                    self.current_section = sections[new_index];
                    self.scroll_position = 0; // Reset scroll when changing sections
                    Ok(ScreenTransition::None)
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.scroll_position = self.scroll_position.saturating_sub(1);
                    Ok(ScreenTransition::None)
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    // Calculate actual maximum scroll based on content and viewport
                    let max_scroll = if self.content_height > self.viewport_height {
                        self.content_height.saturating_sub(self.viewport_height)
                    } else {
                        0
                    };

                    if self.scroll_position < max_scroll {
                        self.scroll_position = self.scroll_position.saturating_add(1);
                    }
                    Ok(ScreenTransition::None)
                }
                KeyCode::Char('g') => {
                    if Self::try_open_github()? {
                        Ok(ScreenTransition::None)
                    } else {
                        self.github_fallback =
                            Some("https://github.com/unhappychoice/gittype".to_string());
                        Ok(ScreenTransition::None)
                    }
                }
                KeyCode::Esc => Ok(ScreenTransition::Pop),
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    Ok(ScreenTransition::Exit)
                }
                _ => Ok(ScreenTransition::None),
            }
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut std::io::Stdout,
        _session_result: Option<&crate::models::SessionResult>,
        _total_result: Option<&crate::scoring::TotalResult>,
    ) -> Result<()> {
        // HelpScreen only supports ratatui rendering
        Ok(())
    }

    fn render_ratatui(&mut self, frame: &mut Frame) -> Result<()> {
        if let Some(url) = &self.github_fallback {
            self.render_github_fallback(frame, url);
            return Ok(());
        }

        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(4),
            ])
            .split(area);

        self.render_tabs(frame, chunks[0]);
        self.render_content(frame, chunks[1]);
        self.render_footer(frame, chunks[2]);

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> crate::Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

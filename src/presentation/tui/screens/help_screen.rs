use crate::domain::events::presentation_events::NavigateTo;
use crate::domain::events::EventBusInterface;
use crate::domain::models::rank::{Rank, RankTier};
use crate::infrastructure::browser;
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Tabs, Wrap,
    },
    Frame,
};
use std::sync::Arc;
use std::sync::RwLock;

const THIRD_PARTY_LICENSES: &str = include_str!("../../../../LICENSE-THIRD-PARTY");

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum HelpSection {
    Scoring,
    Ranks,
    GameHelp,
    #[default]
    CLI,
    About,
    ThirdPartyLicenses,
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
            HelpSection::ThirdPartyLicenses => "Third-Party Licenses",
            HelpSection::Community => "Community",
        }
    }

    pub fn all() -> Vec<HelpSection> {
        vec![
            HelpSection::CLI,
            HelpSection::Scoring,
            HelpSection::Ranks,
            HelpSection::GameHelp,
            HelpSection::Community,
            HelpSection::About,
            HelpSection::ThirdPartyLicenses,
        ]
    }
}

pub trait HelpScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = HelpScreenInterface)]
pub struct HelpScreen {
    #[shaku(default)]
    current_section: RwLock<HelpSection>,

    #[shaku(default)]
    github_fallback: RwLock<Option<String>>,

    #[shaku(default)]
    scroll_position: RwLock<u16>,

    #[shaku(default)]
    content_height: RwLock<u16>,

    #[shaku(default)]
    viewport_height: RwLock<u16>,

    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
}

impl HelpScreen {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
    ) -> Self {
        Self {
            current_section: RwLock::new(HelpSection::CLI),
            github_fallback: RwLock::new(None),
            scroll_position: RwLock::new(0),
            content_height: RwLock::new(0),
            viewport_height: RwLock::new(0),
            event_bus,
            theme_service,
        }
    }

    fn get_scoring_content(colors: &Colors) -> Text<'static> {
        Text::from(vec![
            Line::from(vec![Span::styled(
                "Score Calculation Formula:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "Base Score = CPM × (Accuracy / 100) × 10",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("CPM", Style::default().fg(colors.cpm_wpm())),
                Span::styled(
                    " (Characters Per Minute): ",
                    Style::default().fg(colors.text()),
                ),
                Span::styled(
                    "Total characters typed / minutes",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled("WPM", Style::default().fg(colors.cpm_wpm())),
                Span::styled(" (Words Per Minute): ", Style::default().fg(colors.text())),
                Span::styled(
                    "CPM / 5 (average word length)",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Accuracy", Style::default().fg(colors.accuracy())),
                Span::styled(": ", Style::default().fg(colors.text())),
                Span::styled(
                    "(Total chars - Mistakes) / Total chars × 100%",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Bonuses & Penalties:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "• Consistency Bonus: Up to 70% extra for high accuracy",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Time Bonus: Extra points for fast completion",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Mistake Penalty: -5 points per error",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Final Score = (Base + Consistency + Time - Penalties) × 2 + 100",
                Style::default().fg(colors.text()),
            )),
        ])
    }

    fn get_ranks_content(colors: &Colors) -> Text<'static> {
        let ranks = Rank::all_ranks();
        let mut lines = vec![
            Line::from(vec![Span::styled(
                "Rank Tiers:",
                Style::default().fg(colors.title()).bold(),
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
                    Style::default().fg(colors.text_secondary()),
                ),
            ]));
            lines.push(Line::from(vec![Span::styled(
                format!("  {}", description),
                Style::default().fg(colors.text()),
            )]));
            lines.push(Line::from(""));
        }

        lines.push(Line::from(vec![Span::styled(
            "All Ranks:",
            Style::default().fg(colors.title()).bold(),
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
                    Span::styled("  • ", Style::default().fg(colors.text_secondary())),
                    Span::styled(
                        rank_name.to_string(),
                        Style::default().fg(Colors::from_crossterm(rank.terminal_color())),
                    ),
                    Span::styled(
                        format!(" ({})", display_score),
                        Style::default().fg(colors.text_secondary()),
                    ),
                ]));
            }
            lines.push(Line::from(""));
        }

        lines.push(Line::from(vec![
            Span::styled("Note:", Style::default().fg(colors.info())),
            Span::styled(
                " The highest rank remains mysterious until achieved!",
                Style::default().fg(colors.text()),
            ),
        ]));

        Text::from(lines)
    }

    fn get_game_help_content(colors: &Colors) -> Text<'static> {
        Text::from(vec![
            Line::from(vec![Span::styled(
                "Game Modes:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "• Standard: Type code from popular repositories",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Difficulty: Choose Easy, Normal, Hard, Wild, or Zen",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Code Challenge Types:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "GitType extracts real code constructs from repositories:",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "• Functions, methods, and procedures",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Classes, structs, and interfaces",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Enums, traits, and type definitions",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Variables, constants, and modules",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• React components and namespaces",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Control flow (loops, conditionals)",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Typing Tips:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "• Focus on accuracy over speed initially",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Use proper finger positioning",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Practice regularly to improve muscle memory",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Don't look at the keyboard while typing",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Take breaks to avoid fatigue",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Advanced Typing Tips:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "• Use simultaneous key presses for efficiency:",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "  - For 'knock': press 'kno' with right hand almost simultaneously,",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "    then 'ck' together",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "  - Practice common letter combinations as single motions",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "• Master Shift key timing:",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "  - Press Shift slightly before the target letter",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "  - Use the opposite hand's Shift when possible",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "  - Release Shift immediately after the letter",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "• Optimize hand movement:",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "  - Keep wrists straight and hands relaxed",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "  - Use minimal finger movement",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "  - Practice chord-like movements for common patterns",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "• Code-specific techniques:",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "  - Learn bracket/brace patterns as single motions",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "  - Practice common variable naming conventions",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "  - Master punctuation placement without looking",
                Style::default().fg(colors.text()),
            )),
        ])
    }

    fn get_cli_content(colors: &Colors) -> Text<'static> {
        Text::from(vec![
            Line::from(vec![Span::styled(
                "Basic Usage:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype"),
                    Style::default().fg(colors.text()),
                ),
                Span::styled(
                    "# Start with current directory",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype /path/to/repo"),
                    Style::default().fg(colors.text()),
                ),
                Span::styled(
                    "# Use specific repository path",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype --repo owner/repo"),
                    Style::default().fg(colors.text()),
                ),
                Span::styled(
                    "# Clone and use GitHub repository",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype --langs rust,python"),
                    Style::default().fg(colors.text()),
                ),
                Span::styled(
                    "# Filter by programming languages",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Repository Commands:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype repo list"),
                    Style::default().fg(colors.text()),
                ),
                Span::styled(
                    "# List all cached repositories",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype repo clear"),
                    Style::default().fg(colors.text()),
                ),
                Span::styled(
                    "# Clear all cached repositories",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype repo clear --force"),
                    Style::default().fg(colors.text()),
                ),
                Span::styled(
                    "# Force clear without confirmation",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype repo play"),
                    Style::default().fg(colors.text()),
                ),
                Span::styled(
                    "# Play a cached repository interactively",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Cache Management:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype cache stats"),
                    Style::default().fg(colors.text()),
                ),
                Span::styled(
                    "# Show cache statistics",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype cache clear"),
                    Style::default().fg(colors.text()),
                ),
                Span::styled(
                    "# Clear all cached challenges",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "gittype cache list"),
                    Style::default().fg(colors.text()),
                ),
                Span::styled(
                    "# List cached repository keys",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Cache Locations:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "~/.gittype/"),
                    Style::default().fg(colors.info()),
                ),
                Span::styled(
                    "# Main cache directory",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "~/.gittype/repos/"),
                    Style::default().fg(colors.info()),
                ),
                Span::styled(
                    "# Repository data cache",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "~/.gittype/cache/"),
                    Style::default().fg(colors.info()),
                ),
                Span::styled(
                    "# Challenge cache",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("{:<33}", "~/.gittype/gittype.db"),
                    Style::default().fg(colors.info()),
                ),
                Span::styled(
                    "# Session history database",
                    Style::default().fg(colors.text_secondary()),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Examples:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "gittype --repo rust-lang/rust",
                Style::default().fg(colors.text()),
            )]),
            Line::from(vec![Span::styled(
                "gittype --repo facebook/react",
                Style::default().fg(colors.text()),
            )]),
            Line::from(vec![Span::styled(
                "gittype --repo microsoft/vscode",
                Style::default().fg(colors.text()),
            )]),
            Line::from(vec![Span::styled(
                "gittype --langs rust,typescript,python",
                Style::default().fg(colors.text()),
            )]),
        ])
    }

    fn get_about_content(colors: &Colors) -> Text<'static> {
        Text::from(vec![
            Line::from(vec![Span::styled(
                "GitType",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "A CLI code-typing game that turns your source code into typing challenges",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Practice typing with your own code repositories -",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "improve your speed and accuracy while working with",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "real functions, classes, and methods from your actual projects.",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Development Team:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "• Creator & Lead Developer: ",
                    Style::default().fg(colors.text()),
                ),
                Span::styled("unhappychoice", Style::default().fg(colors.success())),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Special Thanks:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "• All open-source repository maintainers",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• The Rust community for excellent tooling",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Tree-sitter for code parsing capabilities",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Ratatui for terminal UI framework",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• All contributors and users providing feedback",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Built with:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "• Rust - Systems programming language",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Ratatui - Terminal user interface library",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Tree-sitter - Code parsing and syntax highlighting",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• SQLite - Local data storage",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Git2 - Repository cloning and management",
                Style::default().fg(colors.text()),
            )),
        ])
    }

    fn get_third_party_licenses_content() -> Text<'static> {
        let lines = THIRD_PARTY_LICENSES
            .lines()
            .map(Line::from)
            .collect::<Vec<_>>();
        Text::from(lines)
    }

    fn get_community_content(colors: &Colors) -> Text<'static> {
        Text::from(vec![
            Line::from(vec![Span::styled(
                "Social Media:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "Share your progress with #gittype",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "https://x.com/search?q=%23gittype",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Join the Community!",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "GitHub Repository:",
                Style::default().fg(colors.success()).bold(),
            )]),
            Line::from(Span::styled(
                "https://github.com/unhappychoice/gittype",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "✨ Star the repository if you enjoy GitType!",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Contributing:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "• Report bugs and suggest features via GitHub Issues",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Submit pull requests for improvements",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Add support for new programming languages",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Improve code extraction algorithms",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Enhance UI/UX design",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Bug Reporting:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "When reporting bugs, please include:",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Operating system and terminal details",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Steps to reproduce the issue",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Expected vs actual behavior",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "• Any error messages or logs",
                Style::default().fg(colors.text()),
            )),
            Line::from(""),
            Line::from(vec![Span::styled(
                "License:",
                Style::default().fg(colors.title()).bold(),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                "GitType is open-source software.",
                Style::default().fg(colors.text()),
            )),
            Line::from(Span::styled(
                "Check the LICENSE file for details.",
                Style::default().fg(colors.text()),
            )),
        ])
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect, colors: &Colors) {
        let sections = HelpSection::all();
        let titles: Vec<Line> = sections
            .iter()
            .map(|section| Line::from(section.title()))
            .collect();

        let current_section = *self.current_section.read().unwrap();
        let selected_index = sections
            .iter()
            .position(|&s| s == current_section)
            .unwrap_or(0);

        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors.border()))
                    .title("Help"),
            )
            .highlight_style(Style::default().fg(colors.text()).bold())
            .select(selected_index);

        frame.render_widget(tabs, area);
    }

    fn render_content(&self, frame: &mut Frame, area: Rect, colors: &Colors) {
        let current_section = *self.current_section.read().unwrap();
        let content = match current_section {
            HelpSection::Scoring => Self::get_scoring_content(colors),
            HelpSection::Ranks => Self::get_ranks_content(colors),
            HelpSection::GameHelp => Self::get_game_help_content(colors),
            HelpSection::CLI => Self::get_cli_content(colors),
            HelpSection::About => Self::get_about_content(colors),
            HelpSection::ThirdPartyLicenses => Self::get_third_party_licenses_content(),
            HelpSection::Community => Self::get_community_content(colors),
        };

        // Update viewport and content height for scrolling
        let viewport_height = area.height.saturating_sub(2); // Account for borders
        let content_height = content.lines.len() as u16;
        *self.viewport_height.write().unwrap() = viewport_height;
        *self.content_height.write().unwrap() = content_height;

        let scroll_position = *self.scroll_position.read().unwrap();
        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors.border()))
                    .padding(Padding::horizontal(2)),
            )
            .wrap(Wrap { trim: true })
            .scroll((scroll_position, 0));

        frame.render_widget(paragraph, area);

        // Render scrollbar if content is longer than viewport
        if content_height > viewport_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
            let mut scrollbar_state =
                ScrollbarState::new(content_height.saturating_sub(viewport_height) as usize)
                    .position(scroll_position as usize);

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

    fn render_github_fallback(&self, frame: &mut Frame, url: &str, colors: &Colors) {
        let width = std::cmp::max(60, url.len() + 4) as u16;
        let area = Self::centered_rect(width, 8, frame.area());

        frame.render_widget(Clear, area);

        let block = Block::default()
            .title("Cannot open GitHub")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.error()));

        frame.render_widget(block, area);

        let inner = area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        });

        let message = Paragraph::new("Please copy and paste the URL below:")
            .style(Style::default().fg(colors.warning()))
            .alignment(Alignment::Center);

        let message_area = Rect {
            x: inner.x,
            y: inner.y + 1,
            width: inner.width,
            height: 1,
        };

        frame.render_widget(message, message_area);

        let url_para = Paragraph::new(url)
            .style(Style::default().fg(colors.info()).bold())
            .alignment(Alignment::Center);

        let url_area = Rect {
            x: inner.x,
            y: inner.y + 2,
            width: inner.width,
            height: 1,
        };

        frame.render_widget(url_para, url_area);

        let back_instructions = vec![
            Span::styled("[ESC]", Style::default().fg(colors.key_action())),
            Span::styled(" Back", Style::default().fg(colors.text())),
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

    fn render_footer(&self, frame: &mut Frame, area: Rect, colors: &Colors) {
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
            Span::styled("★ ", Style::default().fg(colors.warning())),
            Span::styled("Star us on GitHub (", Style::default().fg(colors.text())),
            Span::styled(
                "https://github.com/unhappychoice/gittype",
                Style::default().fg(colors.text_secondary()),
            ),
            Span::styled(
                ") if you enjoy GitType! ",
                Style::default().fg(colors.text()),
            ),
            Span::styled("★", Style::default().fg(colors.warning())),
        ];
        let star_para = Paragraph::new(Line::from(star_message)).alignment(Alignment::Center);
        frame.render_widget(star_para, chunks[1]);

        // Instructions
        let instructions = vec![
            Span::styled("[←→/HL]", Style::default().fg(colors.info())),
            Span::styled(" Switch tabs ", Style::default().fg(colors.text())),
            Span::styled("[↑↓/JK]", Style::default().fg(colors.info())),
            Span::styled(" Scroll ", Style::default().fg(colors.text())),
            Span::styled("[G]", Style::default().fg(colors.key_action())),
            Span::styled(" GitHub ", Style::default().fg(colors.text())),
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(" Close", Style::default().fg(colors.text())),
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
        Ok(browser::open_url(url).is_ok())
    }
}

pub struct HelpScreenDataProvider;

impl ScreenDataProvider for HelpScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

impl Screen for HelpScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::Help
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(HelpScreenDataProvider)
    }

    fn init_with_data(&self, _data: Box<dyn std::any::Any>) -> crate::Result<()> {
        Ok(())
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) -> crate::Result<()> {
        if self.github_fallback.read().unwrap().is_some() {
            match key_event.code {
                KeyCode::Esc => {
                    *self.github_fallback.write().unwrap() = None;
                    Ok(())
                }
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                    Ok(())
                }
                _ => Ok(()),
            }
        } else {
            let sections = HelpSection::all();
            let current_section = *self.current_section.read().unwrap();
            let current_index = sections
                .iter()
                .position(|&s| s == current_section)
                .unwrap_or(0);

            match key_event.code {
                KeyCode::Left | KeyCode::Char('h') => {
                    let new_index = if current_index == 0 {
                        sections.len() - 1
                    } else {
                        current_index - 1
                    };
                    *self.current_section.write().unwrap() = sections[new_index];
                    *self.scroll_position.write().unwrap() = 0; // Reset scroll when changing sections
                    Ok(())
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    let new_index = (current_index + 1) % sections.len();
                    *self.current_section.write().unwrap() = sections[new_index];
                    *self.scroll_position.write().unwrap() = 0; // Reset scroll when changing sections
                    Ok(())
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    let scroll_position = *self.scroll_position.read().unwrap();
                    *self.scroll_position.write().unwrap() = scroll_position.saturating_sub(1);
                    Ok(())
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let content_height = *self.content_height.read().unwrap();
                    let viewport_height = *self.viewport_height.read().unwrap();
                    let scroll_position = *self.scroll_position.read().unwrap();

                    // Calculate actual maximum scroll based on content and viewport
                    let max_scroll = if content_height > viewport_height {
                        content_height.saturating_sub(viewport_height)
                    } else {
                        0
                    };

                    if scroll_position < max_scroll {
                        *self.scroll_position.write().unwrap() = scroll_position.saturating_add(1);
                    }
                    Ok(())
                }
                KeyCode::Char('g') => {
                    if Self::try_open_github()? {
                        Ok(())
                    } else {
                        *self.github_fallback.write().unwrap() =
                            Some("https://github.com/unhappychoice/gittype".to_string());
                        Ok(())
                    }
                }
                KeyCode::Esc => {
                    self.event_bus.as_event_bus().publish(NavigateTo::Pop);
                    Ok(())
                }
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                    Ok(())
                }
                _ => Ok(()),
            }
        }
    }

    fn render_ratatui(&self, frame: &mut Frame) -> Result<()> {
        let colors = self.theme_service.get_colors();
        let github_fallback = self.github_fallback.read().unwrap().clone();
        if let Some(url) = &github_fallback {
            self.render_github_fallback(frame, url, &colors);
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

        self.render_tabs(frame, chunks[0], &colors);
        self.render_content(frame, chunks[1], &colors);
        self.render_footer(frame, chunks[2], &colors);

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&self) -> crate::Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl HelpScreenInterface for HelpScreen {}

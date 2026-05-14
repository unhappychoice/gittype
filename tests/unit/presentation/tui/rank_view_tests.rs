use gittype::domain::models::{Rank, RankTier};
use gittype::presentation::tui::views::session_summary::RankView;
use ratatui::backend::TestBackend;
use ratatui::Terminal;

#[test]
fn render_unknown_rank_returns_zero_height() {
    let rank = Rank::new("Unlisted Rank", RankTier::Beginner, 0, 0);
    let backend = TestBackend::new(80, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut height = usize::MAX;

    terminal
        .draw(|frame| {
            height = RankView::render(frame, frame.area(), &rank, 0.0);
        })
        .unwrap();

    assert_eq!(height, 0);
}

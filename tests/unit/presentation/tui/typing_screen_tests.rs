use std::sync::Arc;

use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::session_manager_service::SessionManagerInterface;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::stores::{RepositoryStore, RepositoryStoreInterface};
use gittype::presentation::di::AppModule;
use gittype::presentation::tui::screens::typing_screen::{TypingScreen, TypingScreenProvider};
use gittype::presentation::tui::{Screen, UpdateStrategy};
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use shaku::Provider;

struct FakeSessionManager;

impl SessionManagerInterface for FakeSessionManager {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn create_screen() -> TypingScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let repository_store =
        Arc::new(RepositoryStore::new_for_test()) as Arc<dyn RepositoryStoreInterface>;
    let session_manager = Arc::new(FakeSessionManager) as Arc<dyn SessionManagerInterface>;

    TypingScreen::new(
        Arc::new(EventBus::new()),
        theme_service,
        repository_store,
        session_manager,
    )
}

#[test]
fn init_with_non_concrete_session_manager_keeps_default_state() {
    let screen = create_screen();

    screen.init_with_data(Box::new(())).unwrap();

    assert!(matches!(
        screen.get_update_strategy(),
        UpdateStrategy::InputOnly
    ));
    assert!(screen.as_any().downcast_ref::<TypingScreen>().is_some());
    assert!(screen.cleanup().is_ok());
}

#[test]
fn render_with_non_concrete_session_manager_uses_fallback_skips() {
    let screen = create_screen();
    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| screen.render_ratatui(frame).unwrap())
        .unwrap();
}

#[test]
fn update_strategy_is_hybrid_after_typing_starts() {
    let screen = create_screen();

    screen.set_waiting_to_start(false);

    assert!(matches!(
        screen.get_update_strategy(),
        UpdateStrategy::Hybrid {
            interval,
            input_priority: true
        } if interval == std::time::Duration::from_millis(33)
    ));
}

#[test]
fn provider_resolves_typing_screen_from_app_module() {
    let module = AppModule::builder().build();

    let provided = <TypingScreenProvider as Provider<AppModule>>::provide(&module);

    assert!(provided.is_ok());
}

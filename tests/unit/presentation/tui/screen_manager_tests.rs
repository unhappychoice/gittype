use crossterm::event::KeyEvent;
use gittype::domain::events::EventBus;
use gittype::domain::services::{SessionManager, stage_builder_service::StageRepository};
use gittype::domain::services::session_manager_service::SessionManagerInterface;
use gittype::domain::services::stage_builder_service::StageRepositoryInterface;
use gittype::domain::stores::{ChallengeStore, RepositoryStore, SessionStore};
use gittype::domain::stores::{ChallengeStoreInterface, RepositoryStoreInterface, SessionStoreInterface};
use gittype::infrastructure::terminal::TerminalInterface;
use gittype::presentation::tui::{
    Screen, ScreenDataProvider, ScreenManagerImpl, ScreenTransition, ScreenType, UpdateStrategy,
};
use ratatui::backend::CrosstermBackend;
use ratatui::backend::TestBackend;
use ratatui::Frame;
use ratatui::Terminal;
use std::any::Any;
use std::io::Stdout;
use std::sync::Arc;

// Mock TerminalInterface for testing
#[allow(dead_code)]
struct MockTerminalInterface;

impl TerminalInterface for MockTerminalInterface {
    fn get(&self) -> Terminal<CrosstermBackend<Stdout>> {
        // This won't actually be called in these tests
        // The tests directly construct ScreenManagerImpl with a terminal
        panic!("MockTerminalInterface::get should not be called in tests")
    }
}

// Helper function to create a ScreenManager for testing
// Note: These tests are designed to work without a real terminal
// They test the ScreenManager logic, not terminal I/O
#[cfg(test)]
fn create_test_screen_manager() -> ScreenManagerImpl<TestBackend> {
    let event_bus = Arc::new(EventBus::new());

    // Create stores for DI
    let challenge_store = Arc::new(ChallengeStore::default()) as Arc<dyn ChallengeStoreInterface>;
    let repository_store = Arc::new(RepositoryStore::default()) as Arc<dyn RepositoryStoreInterface>;
    let session_store_arc = Arc::new(SessionStore::default()) as Arc<dyn SessionStoreInterface>;

    // Create StageRepository
    let stage_repository = StageRepository::new(
        None,
        challenge_store.clone(),
        repository_store.clone(),
        session_store_arc.clone(),
    );
    let stage_repository: Arc<dyn StageRepositoryInterface> = Arc::new(stage_repository);

    // Create SessionManager with dependencies
    let session_manager = SessionManager::new_with_dependencies(
        event_bus.clone(),
        stage_repository.clone(),
    );
    let session_manager: Arc<dyn SessionManagerInterface> = Arc::new(session_manager);

    let backend = TestBackend::new(80, 24);
    let terminal = Terminal::new(backend).expect("Failed to create test terminal");

    ScreenManagerImpl::new(event_bus, session_store_arc, session_manager, stage_repository, terminal)
}

// Mock screen for testing
struct MockScreen {
    screen_type: ScreenType,
}

// Mock data provider for testing
struct MockDataProvider;

impl ScreenDataProvider for MockDataProvider {
    fn provide(&self) -> gittype::Result<Box<dyn Any>> {
        Ok(Box::new(()))
    }
}

impl MockScreen {
    fn new(screen_type: ScreenType) -> Self {
        Self { screen_type }
    }
}

impl Screen for MockScreen {
    fn get_type(&self) -> ScreenType {
        self.screen_type.clone()
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(MockDataProvider)
    }

    fn init_with_data(&self, _data: Box<dyn Any>) -> gittype::Result<()> {
        Ok(())
    }

    fn update(&self) -> gittype::Result<bool> {
        Ok(false)
    }

    fn render_ratatui(&self, _frame: &mut Frame) -> gittype::Result<()> {
        Ok(())
    }

    fn handle_key_event(&self, _key_event: KeyEvent) -> gittype::Result<()> {
        Ok(())
    }

    fn cleanup(&self) -> gittype::Result<()> {
        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
fn test_new() {
    let manager = create_test_screen_manager();

    assert_eq!(*manager.get_current_screen_type(), ScreenType::Title);
    assert!(manager.get_screen_stack().is_empty());
    assert!(!manager.is_terminal_initialized());
}

#[test]
fn test_register_screen() {
    let mut manager = create_test_screen_manager();

    let mock_screen = MockScreen::new(ScreenType::Help);
    manager.register_screen(mock_screen);

    assert!(manager.get_screen(&ScreenType::Help).is_some());
}

#[test]
fn test_get_screen() {
    let mut manager = create_test_screen_manager();

    manager.register_screen(MockScreen::new(ScreenType::Settings));

    let screen = manager.get_screen(&ScreenType::Settings);
    assert!(screen.is_some());
    assert_eq!(screen.unwrap().get_type(), ScreenType::Settings);
}

#[test]
fn test_get_screen_mut() {
    let mut manager = create_test_screen_manager();

    manager.register_screen(MockScreen::new(ScreenType::Help));

    let screen = manager.get_screen_mut(&ScreenType::Help);
    assert!(screen.is_some());
    assert_eq!(screen.unwrap().get_type(), ScreenType::Help);
}

#[test]
fn test_get_screen_not_registered() {
    let manager = create_test_screen_manager();

    assert!(manager.get_screen(&ScreenType::Help).is_none());
}

#[test]
fn test_get_current_screen_type() {
    let manager = create_test_screen_manager();

    assert_eq!(*manager.get_current_screen_type(), ScreenType::Title);
}

#[test]
fn test_get_screen_stack() {
    let manager = create_test_screen_manager();

    let stack = manager.get_screen_stack();
    assert!(stack.is_empty());
}

#[test]
fn test_register_multiple_screens() {
    let mut manager = create_test_screen_manager();

    manager.register_screen(MockScreen::new(ScreenType::Help));
    manager.register_screen(MockScreen::new(ScreenType::Settings));
    manager.register_screen(MockScreen::new(ScreenType::Analytics));

    assert!(manager.get_screen(&ScreenType::Help).is_some());
    assert!(manager.get_screen(&ScreenType::Settings).is_some());
    assert!(manager.get_screen(&ScreenType::Analytics).is_some());
}

#[test]
fn test_screen_type_variants() {
    let screen_type = ScreenType::Title;
    let debug_str = format!("{:?}", screen_type);
    assert!(debug_str.contains("Title"));
}

#[test]
fn test_screen_transition_variants() {
    let transition_none = ScreenTransition::None;
    let transition_push = ScreenTransition::Push(ScreenType::Help);
    let transition_pop = ScreenTransition::Pop;
    let transition_replace = ScreenTransition::Replace(ScreenType::Settings);
    let transition_exit = ScreenTransition::Exit;

    assert!(matches!(transition_none, ScreenTransition::None));
    assert!(matches!(transition_push, ScreenTransition::Push(_)));
    assert!(matches!(transition_pop, ScreenTransition::Pop));
    assert!(matches!(transition_replace, ScreenTransition::Replace(_)));
    assert!(matches!(transition_exit, ScreenTransition::Exit));
}

#[test]
fn test_update_strategy_variants() {
    let input_only = UpdateStrategy::InputOnly;
    let time_based = UpdateStrategy::TimeBased(std::time::Duration::from_millis(100));
    let hybrid = UpdateStrategy::Hybrid {
        interval: std::time::Duration::from_millis(50),
        input_priority: true,
    };

    assert!(matches!(input_only, UpdateStrategy::InputOnly));
    assert!(matches!(time_based, UpdateStrategy::TimeBased(_)));
    assert!(matches!(hybrid, UpdateStrategy::Hybrid { .. }));
}

#[test]
fn test_cleanup_terminal_static() {
    // This test verifies the function can be called
    ScreenManagerImpl::<TestBackend>::cleanup_terminal_static();
}

#[test]
fn test_render_current_screen_with_test_backend() {
    let mut manager = create_test_screen_manager();

    manager.register_screen(MockScreen::new(ScreenType::Title));

    // render_current_screen should work even without initialize_terminal
    // because ratatui_terminal is None and it handles that case
    let result = manager.render_current_screen();
    assert!(result.is_ok());
}

#[test]
fn test_is_terminal_initialized() {
    let manager = create_test_screen_manager();

    assert!(!manager.is_terminal_initialized());
}

#[test]
fn test_get_event_bus() {
    let manager = create_test_screen_manager();

    let retrieved_bus = manager.get_event_bus();
    // EventBus should be cloneable and retrievable
    // Just verify we can call the method
    drop(retrieved_bus);
}

#[test]
fn test_set_current_screen() {
    let mut manager = create_test_screen_manager();

    manager.register_screen(MockScreen::new(ScreenType::Help));

    let result = manager.set_current_screen(ScreenType::Help);
    assert!(result.is_ok());
    assert_eq!(*manager.get_current_screen_type(), ScreenType::Help);
}

#[test]
fn test_set_current_screen_not_registered() {
    let mut manager = create_test_screen_manager();

    let result = manager.set_current_screen(ScreenType::Help);
    assert!(result.is_err());
}

#[test]
fn test_push_screen() {
    let mut manager = create_test_screen_manager();

    manager.register_screen(MockScreen::new(ScreenType::Title));
    manager.register_screen(MockScreen::new(ScreenType::Help));

    assert_eq!(*manager.get_current_screen_type(), ScreenType::Title);

    let result = manager.push_screen(ScreenType::Help);
    assert!(result.is_ok());
    assert_eq!(*manager.get_current_screen_type(), ScreenType::Help);
    assert_eq!(manager.get_screen_stack().len(), 1);
    assert_eq!(manager.get_screen_stack()[0], ScreenType::Title);
}

#[test]
fn test_handle_transition_push() {
    let mut manager = create_test_screen_manager();

    manager.register_screen(MockScreen::new(ScreenType::Title));
    manager.register_screen(MockScreen::new(ScreenType::Help));

    let result = manager.handle_transition(ScreenTransition::Push(ScreenType::Help));
    assert!(result.is_ok());
    assert_eq!(*manager.get_current_screen_type(), ScreenType::Help);
}

#[test]
fn test_handle_transition_exit() {
    let mut manager = create_test_screen_manager();

    let result = manager.handle_transition(ScreenTransition::Exit);
    assert!(result.is_ok());
    // Exit transition should succeed
}

#[test]
fn test_handle_transition_none() {
    let mut manager = create_test_screen_manager();

    manager.register_screen(MockScreen::new(ScreenType::Title));

    let current = manager.get_current_screen_type().clone();
    let result = manager.handle_transition(ScreenTransition::None);
    assert!(result.is_ok());
    assert_eq!(*manager.get_current_screen_type(), current);
}

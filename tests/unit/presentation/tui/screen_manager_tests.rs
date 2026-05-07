use crossterm::event::KeyEvent;
use gittype::domain::events::EventBus;
use gittype::domain::services::scoring::{
    SessionTracker, SessionTrackerInterface, TotalTracker, TotalTrackerInterface,
};
use gittype::domain::services::session_manager_service::SessionManagerInterface;
use gittype::domain::services::stage_builder_service::StageRepositoryInterface;
use gittype::domain::services::{stage_builder_service::StageRepository, SessionManager};
use gittype::domain::stores::{ChallengeStore, RepositoryStore, SessionStore};
use gittype::domain::stores::{
    ChallengeStoreInterface, RepositoryStoreInterface, SessionStoreInterface,
};
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
use std::sync::{Arc, Mutex};

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
    let repository_store =
        Arc::new(RepositoryStore::default()) as Arc<dyn RepositoryStoreInterface>;
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
    let session_tracker: Arc<dyn SessionTrackerInterface> = Arc::new(SessionTracker::default());
    let total_tracker: Arc<dyn TotalTrackerInterface> = Arc::new(TotalTracker::default());
    let session_manager = SessionManager::new_with_dependencies(
        event_bus.clone(),
        stage_repository.clone(),
        session_tracker,
        total_tracker,
    );
    let session_manager: Arc<dyn SessionManagerInterface> = Arc::new(session_manager);

    let backend = TestBackend::new(80, 24);
    let terminal = Terminal::new(backend).expect("Failed to create test terminal");

    ScreenManagerImpl::new(
        event_bus,
        session_store_arc,
        session_manager,
        stage_repository,
        terminal,
    )
}

// Mock screen for testing
struct MockScreen {
    screen_type: ScreenType,
}

struct ExitableScreen {
    screen_type: ScreenType,
}

struct PushAwareScreen {
    screen_type: ScreenType,
    pushed_from: Arc<Mutex<Option<ScreenType>>>,
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

impl ExitableScreen {
    fn new(screen_type: ScreenType) -> Self {
        Self { screen_type }
    }
}

impl PushAwareScreen {
    fn new(screen_type: ScreenType, pushed_from: Arc<Mutex<Option<ScreenType>>>) -> Self {
        Self {
            screen_type,
            pushed_from,
        }
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

impl Screen for ExitableScreen {
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

    fn is_exitable(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Screen for PushAwareScreen {
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

    fn on_pushed_from(&self, source_screen: &dyn Screen) -> gittype::Result<()> {
        *self.pushed_from.lock().unwrap() = Some(source_screen.get_type());
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
fn test_register_screen_interface() {
    let mut manager = create_test_screen_manager();

    let screen = Arc::new(MockScreen::new(ScreenType::Help)) as Arc<dyn Screen>;
    manager.register_screen_interface(screen);

    assert!(manager.get_screen(&ScreenType::Help).is_some());
}

#[test]
fn test_register_screen_arc_delegates_screen_methods() {
    let mut manager = create_test_screen_manager();

    manager.register_screen_arc(Arc::new(MockScreen::new(ScreenType::Help)));

    let screen = manager.get_screen(&ScreenType::Help).unwrap();
    assert_eq!(screen.get_type(), ScreenType::Help);
    assert!(screen.init_with_data(Box::new(())).is_ok());
    assert!(screen
        .handle_key_event(KeyEvent::from(crossterm::event::KeyCode::Enter))
        .is_ok());
    assert!(screen.update().is_ok());
    assert!(screen.cleanup().is_ok());
    assert!(matches!(
        screen.get_update_strategy(),
        UpdateStrategy::InputOnly
    ));
    assert!(!screen.is_exitable());
    assert!(screen.as_any().downcast_ref::<MockScreen>().is_some());
}

#[test]
fn test_register_screen_arc_delegates_render_and_push_source() {
    let mut manager = create_test_screen_manager();
    let pushed_from = Arc::new(Mutex::new(None));

    manager.register_screen(MockScreen::new(ScreenType::Title));
    manager.register_screen_arc(Arc::new(PushAwareScreen::new(
        ScreenType::Help,
        Arc::clone(&pushed_from),
    )));

    manager.push_screen(ScreenType::Help).unwrap();
    manager.render_current_screen().unwrap();

    assert_eq!(*manager.get_current_screen_type(), ScreenType::Help);
    assert_eq!(*pushed_from.lock().unwrap(), Some(ScreenType::Title));
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
fn test_initialize_terminal_rejects_non_tty_stdout() {
    if atty::is(atty::Stream::Stdout) {
        return;
    }

    let mut manager = create_test_screen_manager();

    let result = manager.initialize_terminal();

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Not running in a terminal environment"));
    assert!(!manager.is_terminal_initialized());
}

#[test]
fn test_default_constructs_title_screen_manager() {
    if !atty::is(atty::Stream::Stdout) {
        return;
    }

    let mut manager = ScreenManagerImpl::<CrosstermBackend<Stdout>>::default();
    manager.skip_cleanup_on_drop();

    assert_eq!(*manager.get_current_screen_type(), ScreenType::Title);
    assert!(manager.get_screen_stack().is_empty());
    assert!(!manager.is_terminal_initialized());
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
fn test_set_current_screen_uses_default_providers_for_registered_screen_types() {
    let screen_types = [
        ScreenType::Loading,
        ScreenType::Typing,
        ScreenType::StageSummary,
        ScreenType::SessionSummary,
        ScreenType::SessionFailure,
        ScreenType::Records,
        ScreenType::Analytics,
        ScreenType::SessionDetail,
        ScreenType::SessionSharing,
        ScreenType::Animation,
        ScreenType::VersionCheck,
        ScreenType::InfoDialog,
        ScreenType::DetailsDialog,
        ScreenType::Panic,
        ScreenType::TrendingLanguageSelection,
        ScreenType::TrendingRepositorySelection,
    ];

    screen_types.into_iter().for_each(|screen_type| {
        let mut manager = create_test_screen_manager();
        manager.register_screen(MockScreen::new(screen_type.clone()));

        let result = manager.set_current_screen(screen_type.clone());

        assert!(result.is_ok(), "expected {:?} to initialize", screen_type);
        assert_eq!(*manager.get_current_screen_type(), screen_type);
    });
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
fn test_push_screen_calls_on_pushed_from() {
    let mut manager = create_test_screen_manager();
    let pushed_from = Arc::new(Mutex::new(None));

    manager.register_screen(MockScreen::new(ScreenType::Title));
    manager.register_screen(PushAwareScreen::new(
        ScreenType::Help,
        Arc::clone(&pushed_from),
    ));

    manager.push_screen(ScreenType::Help).unwrap();

    assert_eq!(*manager.get_current_screen_type(), ScreenType::Help);
    assert_eq!(*pushed_from.lock().unwrap(), Some(ScreenType::Title));
}

#[test]
fn test_pop_screen_restores_previous_screen() {
    let mut manager = create_test_screen_manager();

    manager.register_screen(MockScreen::new(ScreenType::Title));
    manager.register_screen(MockScreen::new(ScreenType::Help));
    manager.push_screen(ScreenType::Help).unwrap();

    manager.pop_screen().unwrap();

    assert_eq!(*manager.get_current_screen_type(), ScreenType::Title);
    assert!(manager.get_screen_stack().is_empty());
}

#[test]
fn test_pop_screen_without_history_is_noop() {
    let mut manager = create_test_screen_manager();

    manager.register_screen(MockScreen::new(ScreenType::Title));

    manager.pop_screen().unwrap();

    assert_eq!(*manager.get_current_screen_type(), ScreenType::Title);
    assert!(manager.get_screen_stack().is_empty());
}

#[test]
fn test_pop_to_screen_discards_intermediate_stack_entries() {
    let mut manager = create_test_screen_manager();

    [ScreenType::Title, ScreenType::Settings, ScreenType::Help]
        .into_iter()
        .map(MockScreen::new)
        .for_each(|screen| manager.register_screen(screen));

    manager.push_screen(ScreenType::Settings).unwrap();
    manager.push_screen(ScreenType::Help).unwrap();
    manager.pop_to_screen(ScreenType::Title).unwrap();

    assert_eq!(*manager.get_current_screen_type(), ScreenType::Title);
    assert!(manager.get_screen_stack().is_empty());
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
fn test_handle_transition_pop() {
    let mut manager = create_test_screen_manager();

    manager.register_screen(MockScreen::new(ScreenType::Title));
    manager.register_screen(MockScreen::new(ScreenType::Help));
    manager.push_screen(ScreenType::Help).unwrap();

    manager.handle_transition(ScreenTransition::Pop).unwrap();

    assert_eq!(*manager.get_current_screen_type(), ScreenType::Title);
    assert!(manager.get_screen_stack().is_empty());
}

#[test]
fn test_handle_transition_pop_to() {
    let mut manager = create_test_screen_manager();

    [ScreenType::Title, ScreenType::Settings, ScreenType::Help]
        .into_iter()
        .map(MockScreen::new)
        .for_each(|screen| manager.register_screen(screen));

    manager.push_screen(ScreenType::Settings).unwrap();
    manager.push_screen(ScreenType::Help).unwrap();

    manager
        .handle_transition(ScreenTransition::PopTo(ScreenType::Title))
        .unwrap();

    assert_eq!(*manager.get_current_screen_type(), ScreenType::Title);
    assert!(manager.get_screen_stack().is_empty());
}

#[test]
fn test_handle_transition_exit() {
    let mut manager = create_test_screen_manager();

    let result = manager.handle_transition(ScreenTransition::Exit);
    assert!(result.is_ok());
    // Exit transition should succeed
}

#[test]
fn test_handle_transition_exit_replaces_with_total_summary() {
    let mut manager = create_test_screen_manager();

    [
        ScreenType::Title,
        ScreenType::Help,
        ScreenType::TotalSummary,
    ]
    .into_iter()
    .map(MockScreen::new)
    .for_each(|screen| manager.register_screen(screen));

    manager.set_current_screen(ScreenType::Help).unwrap();
    manager.handle_transition(ScreenTransition::Exit).unwrap();

    assert_eq!(*manager.get_current_screen_type(), ScreenType::TotalSummary);
}

#[test]
fn test_handle_transition_exit_keeps_exitable_screen_current() {
    let mut manager = create_test_screen_manager();

    manager.register_screen(ExitableScreen::new(ScreenType::Help));
    manager.register_screen(MockScreen::new(ScreenType::TotalSummary));

    manager.set_current_screen(ScreenType::Help).unwrap();
    manager.handle_transition(ScreenTransition::Exit).unwrap();

    assert_eq!(*manager.get_current_screen_type(), ScreenType::Help);
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

#[test]
fn test_cleanup_terminal_without_initialization_is_noop() {
    let mut manager = create_test_screen_manager();

    assert!(manager.cleanup_terminal().is_ok());
    assert!(!manager.is_terminal_initialized());
}

#[test]
fn test_mark_terminal_initialized_and_skip_cleanup_on_drop_toggle_flag() {
    let mut manager = create_test_screen_manager();

    manager.mark_terminal_initialized();
    assert!(manager.is_terminal_initialized());

    manager.skip_cleanup_on_drop();
    assert!(!manager.is_terminal_initialized());
}

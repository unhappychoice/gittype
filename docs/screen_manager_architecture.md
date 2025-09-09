# ScreenManager Architecture

## Overview

The ScreenManager is a centralized system for managing screen transitions, rendering loops, input handling, and terminal lifecycle in GitType. This architecture provides a unified approach for handling all screen-related operations and supports both crossterm and ratatui rendering backends.

## Key Components

### ScreenManager

The core component that coordinates all screen operations:

```rust
pub struct ScreenManager {
    screens: HashMap<ScreenType, Box<dyn Screen>>,
    screen_stack: Vec<ScreenType>,
    current_screen_type: ScreenType,
    should_exit: bool,
    terminal_initialized: bool,
    last_update: Instant,
    render_backend: RenderBackend,
}
```

Features:
- **Centralized Rendering Loop**: ScreenManager owns and manages the main rendering loop
- **Input Handling**: ScreenManager handles all keyboard input events and dispatches to current screen
- **Screen Management**: Uses screen stack for nested screens/dialogs
- **Terminal Lifecycle**: Manages raw mode, cursor visibility, and alternate screen
- **Dual Rendering Support**: Supports both crossterm and ratatui backends

### Screen Trait

All screens implement the `Screen` trait:

```rust
pub trait Screen: Send {
    fn init(&mut self) -> Result<()>;
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<ScreenTransition>;
    fn render_crossterm(&self, stdout: &mut Stdout) -> Result<()>;
    fn render_ratatui(&self, _frame: &mut ratatui::Frame) -> Result<()>;
    fn cleanup(&mut self) -> Result<()>;
    fn should_exit(&self) -> bool;
    fn get_update_strategy(&self) -> UpdateStrategy;
    fn update(&mut self) -> Result<bool>;
}
```

### Update Strategy System

Screens can define how they should be updated:

```rust
pub enum UpdateStrategy {
    InputOnly,           // Update only on input events
    TimeBased(Duration), // Update at regular intervals
    Hybrid {             // Both input and time-based
        interval: Duration,
        input_priority: bool,
    },
}
```

#### Strategy Examples:
- **Title Screen**: `InputOnly` - Only updates when user presses keys
- **Loading Screen**: `TimeBased(Duration::from_millis(100))` - Updates progress regularly
- **Typing Screen**: `Hybrid { interval: Duration::from_millis(500), input_priority: true }` - Updates on input + cursor blink
- **Animation Screen**: `TimeBased(Duration::from_millis(50))` - Smooth animation updates

### Screen Types

```rust
pub enum ScreenType {
    Title, Loading, Typing, StageSummary, SessionSummary,
    ExitSummary, Cancel, Failure, History, Analytics,
    SessionDetail, Sharing, Animation, VersionCheck,
    InfoDialog, DetailsDialog,
}
```

### Screen Transitions

```rust
pub enum ScreenTransition {
    None,                    // No transition
    Push(ScreenType),       // Push new screen onto stack
    Pop,                    // Pop current screen from stack
    Replace(ScreenType),    // Replace current screen
    PopTo(ScreenType),      // Pop until reaching specific screen
    Exit,                   // Exit application
}
```

## Usage Example

```rust
use gittype::game::{BasicScreen, ScreenManager, ScreenType, UpdateStrategy};

fn main() -> gittype::Result<()> {
    let mut screen_manager = ScreenManager::new();
    
    // Create a screen with input-only updates
    let title_screen = BasicScreen::new(
        "My App".to_string(),
        vec!["Welcome!".to_string()],
        UpdateStrategy::InputOnly,
    );
    
    // Register the screen
    screen_manager.register_screen(ScreenType::Title, Box::new(title_screen));
    
    // Run the application
    screen_manager.run()?;
    
    Ok(())
}
```

## Benefits

1. **Maintainability**: Centralized control makes code easier to understand and modify
2. **Consistency**: All screens follow the same patterns and lifecycle
3. **Performance**: Single rendering loop reduces overhead and improves responsiveness
4. **Extensibility**: Adding new screens becomes straightforward with clear interfaces
5. **Testing**: Easier to unit test individual screens and transitions
6. **Error Handling**: Centralized error handling for screen-related issues
7. **Flexibility**: Support for different rendering backends and update strategies
8. **Terminal Stability**: Reduced terminal configuration overhead and more stable terminal state
9. **Power Efficiency**: Screens can optimize their update frequency based on needs

## Implementation Strategy

### Phase 1: Core Architecture ✅
- [x] Create `ScreenManager` with rendering loop and input handling
- [x] Define `Screen` trait with dual rendering support and update strategies
- [x] Implement `ScreenType` enum and transition system
- [x] Add terminal lifecycle management

### Phase 2: Update Strategy System ✅
- [x] Implement `UpdateStrategy` enum and logic
- [x] Create timer-based update mechanism for time-driven screens
- [x] Add hybrid update support for complex screens
- [x] Optimize rendering pipeline to avoid unnecessary updates

### Phase 3: Screen Migration (In Progress)
- [ ] Refactor existing screens to implement new `Screen` trait
- [ ] Move rendering logic to use centralized system
- [ ] Update input handling to use event dispatching
- [ ] Assign appropriate update strategies to each screen

### Phase 4: Integration (Planned)
- [ ] Integrate ScreenManager into StageManager
- [ ] Remove duplicate rendering and input code
- [ ] Test all screen transitions and functionality
- [ ] Optimize terminal initialization/cleanup

### Phase 5: Optimization (Planned)
- [ ] Performance tuning of rendering loop
- [ ] Add ratatui rendering implementations
- [ ] Fine-tune update strategies for optimal performance
- [ ] Documentation and testing

## Best Practices

1. **Screen Lifecycle**: Always implement proper init/cleanup in screens
2. **Update Strategy**: Choose the most appropriate update strategy for each screen
3. **Error Handling**: Handle errors gracefully in screen implementations
4. **Resource Management**: Clean up resources in the cleanup method
5. **Performance**: Avoid unnecessary renders by returning `false` from `update()` when no changes occur
6. **Input Handling**: Use screen transitions to control navigation flow
7. **Testing**: Test screens individually and integration with ScreenManager

## Migration Guide

To migrate existing screens to the new architecture:

1. Implement the `Screen` trait for your screen struct
2. Move rendering logic to `render_crossterm()` method
3. Move input handling to `handle_key_event()` method
4. Define appropriate `UpdateStrategy` in `get_update_strategy()`
5. Register screen with ScreenManager
6. Remove old direct terminal management code

This architecture provides a solid foundation for building complex terminal applications with proper separation of concerns and consistent behavior across all screens.
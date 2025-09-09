#!/bin/bash

# Add Screen trait implementation to remaining screens

screens=(
  "failure_screen.rs"
  "cancel_screen.rs" 
  "exit_summary_screen.rs"
  "sharing_screen.rs"
  "animation_screen.rs"
  "version_check_screen.rs"
  "session_detail_screen.rs"
  "info_dialog.rs"
  "details_dialog.rs"
)

for screen in "${screens[@]}"; do
  file="src/game/screens/$screen"
  if [ -f "$file" ]; then
    echo "Processing $file..."
    
    # Add Screen trait import if not already present
    if ! grep -q "use crate::game::screen_manager::" "$file"; then
      sed -i '1i use crate::game::screen_manager::{Screen, ScreenTransition, UpdateStrategy};' "$file"
    fi
    
    # Add basic Screen trait implementation at the end
    cat >> "$file" << 'EOF'

// Basic Screen trait implementation for ScreenManager compatibility
pub struct ScreenState {
    should_exit: bool,
}

impl ScreenState {
    pub fn new() -> Self {
        Self { should_exit: false }
    }
}

impl Screen for ScreenState {
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> crate::Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Esc => {
                self.should_exit = true;
                Ok(ScreenTransition::None)
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_exit = true;
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm(&self, _stdout: &mut std::io::Stdout) -> crate::Result<()> {
        Ok(())
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> crate::Result<bool> {
        Ok(false)
    }
}
EOF
    
    echo "Added Screen trait to $file"
  else
    echo "File $file not found"
  fi
done
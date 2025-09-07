use crossterm::event::KeyCode;

/// Generic menu selection state manager
pub struct MenuSelector {
    selected_index: usize,
    item_count: usize,
}

impl MenuSelector {
    pub fn new(item_count: usize) -> Self {
        Self {
            selected_index: 0,
            item_count,
        }
    }

    /// Get currently selected index
    pub fn selected(&self) -> usize {
        self.selected_index
    }

    /// Set selected index (with bounds checking)
    pub fn set_selected(&mut self, index: usize) {
        self.selected_index = index.min(self.item_count.saturating_sub(1));
    }

    /// Handle key input for menu navigation
    /// Returns true if the selection changed
    pub fn handle_key(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.move_up();
                true
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.move_down();
                true
            }
            _ => false,
        }
    }

    /// Move selection up (with wrapping)
    pub fn move_up(&mut self) {
        if self.item_count == 0 {
            return;
        }
        self.selected_index = if self.selected_index == 0 {
            self.item_count - 1
        } else {
            self.selected_index - 1
        };
    }

    /// Move selection down (with wrapping)
    pub fn move_down(&mut self) {
        if self.item_count == 0 {
            return;
        }
        self.selected_index = (self.selected_index + 1) % self.item_count;
    }

    /// Move selection to the left (for horizontal menus)
    pub fn move_left(&mut self) {
        self.move_up(); // Same logic as up for circular navigation
    }

    /// Move selection to the right (for horizontal menus)
    pub fn move_right(&mut self) {
        self.move_down(); // Same logic as down for circular navigation
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessingOptions {
    pub preserve_empty_lines: bool,
    pub add_newline_symbols: bool,
    pub highlight_special_chars: bool,
}

impl Default for ProcessingOptions {
    fn default() -> Self {
        Self {
            preserve_empty_lines: true,
            add_newline_symbols: true,
            highlight_special_chars: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputResult {
    Correct,   // Input was correct, continue
    Incorrect, // Input was incorrect (mistake)
    Completed, // Input was correct and typing is complete
    NoAction,  // No input accepted (already completed)
}

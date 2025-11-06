/// ASCII art digit patterns using oh-my-logo style with simple block font
/// Each digit has consistent width within its own pattern for proper alignment
pub fn get_digit_patterns() -> [Vec<&'static str>; 10] {
    [
        // 0 - width 7
        vec!["   __  ", "  /  \\ ", " | () |", "  \\__/ "],
        // 1 - width 4
        vec!["  _ ", " / |", " | |", " |_|"],
        // 2 - width 6
        vec!["  ___ ", " |_  )", "  / / ", " /___|"],
        // 3 - width 6
        vec!["  ___ ", " |__ /", "  |_ \\", " |___/"],
        // 4 - width 7
        vec!["  _ _  ", " | | | ", " |_  _|", "   |_| "],
        // 5 - width 6
        vec!["  ___ ", " | __|", " |__ \\", " |___/"],
        // 6 - width 6
        vec!["   __ ", "  / / ", " / _ \\", " \\___/"],
        // 7 - width 7
        vec!["  ____ ", " |__  |", "   / / ", "  /_/  "],
        // 8 - width 6
        vec!["  ___ ", " ( _ )", " / _ \\", " \\___/"],
        // 9 - width 6
        vec!["  ___ ", " / _ \\", " \\_, /", "  /_/ "],
    ]
}

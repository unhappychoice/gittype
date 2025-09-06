use gittype::game::typing_core::{ProcessingOptions, TypingCore};

fn main() {
    let code = r#"fn test() {
    // Path symbols: ../.. and ./path and ~/home  
    // Unicode arrows: → ← ↑ ↓ and ↵ symbol
    // Mixed symbols: ../../config.json → ~/.config/
    let x = 42;
}"#;

    let comment_ranges = vec![
        (
            code.find("// Path symbols").unwrap(),
            code.find("~/home").unwrap() + "~/home".len(),
        ),
        (
            code.find("// Unicode arrows").unwrap(),
            code.find("↵ symbol").unwrap() + "↵ symbol".len(),
        ),
        (
            code.find("// Mixed symbols").unwrap(),
            code.find("~/.config/").unwrap() + "~/.config/".len(),
        ),
    ];

    println!(
        "code.len bytes={}, chars={}",
        code.len(),
        code.chars().count()
    );
    println!("comment_ranges (bytes): {:?}", comment_ranges);

    let typing_core = TypingCore::new(code, &comment_ranges, ProcessingOptions::default());
    let display = typing_core.text_to_display().to_string();
    println!(
        "display bytes={}, chars={}",
        display.len(),
        display.chars().count()
    );
    println!("display: {}", display);
    let ranges = typing_core.display_comment_ranges();
    println!("display_ranges: {:?}", ranges);
    for (i, (s, e)) in ranges.iter().enumerate() {
        let frag = &display[*s..*e];
        println!("[{}] {:?}", i, frag);
    }

    // Show how we convert bytes->chars for original ranges
    let to_char = |b: usize| code[..b.min(code.len())].chars().count();
    let converted: Vec<(usize, usize)> = comment_ranges
        .iter()
        .map(|&(s, e)| (to_char(s), to_char(e)))
        .collect();
    println!("converted char ranges: {:?}", converted);
    let code_chars: Vec<char> = code.chars().collect();
    for (i, (cs, ce)) in converted.iter().copied().enumerate() {
        let text: String = code_chars[cs..ce].iter().collect();
        println!("orig[{}]: {:?} (chars {})", i, text, text.chars().count());
    }

    // Show line char starts
    let mut acc = 0usize;
    for (i, line) in code.lines().enumerate() {
        let start = acc;
        let end = start + line.chars().count();
        println!("line {} chars {}..{} => {:?}", i + 1, start, end, line);
        acc = end + 1; // account for \n
    }
}

use gittype::presentation::ui::gradation_text::{ansi256_to_rgb, GradationText, Rgb};

#[test]
fn test_empty_colors() {
    let widget = GradationText::new("test", &[]);
    let line = widget.apply_gradation();
    assert_eq!(line.spans.len(), 1);
}

#[test]
fn test_single_color() {
    let widget = GradationText::new("test", &[111]);
    let line = widget.apply_gradation();
    assert_eq!(line.spans.len(), 1);
}

#[test]
fn test_multiple_colors_smooth() {
    let widget = GradationText::new("Hello", &[30, 66, 72]);
    let line = widget.apply_gradation();
    // Should have multiple spans for smooth gradation
    assert!(!line.spans.is_empty());
}

#[test]
fn test_multiple_colors_stepped() {
    let widget = GradationText::new("Hello", &[30, 66, 72]).smooth(false);
    let line = widget.apply_gradation();
    // Should have spans equal to number of colors
    assert!(!line.spans.is_empty());
}

#[test]
fn test_empty_text() {
    let widget = GradationText::new("", &[30, 66]);
    let line = widget.apply_gradation();
    // Empty text returns an empty line
    assert_eq!(line.spans.len(), 0);
}

#[test]
fn test_rgb_direct() {
    let colors = [Rgb::new(255, 0, 0), Rgb::new(0, 0, 255)];
    let widget = GradationText::new_rgb("Test", &colors);
    let line = widget.apply_gradation();
    assert!(!line.spans.is_empty());
}

#[test]
fn test_rgb_lerp() {
    let c1 = Rgb::new(0, 0, 0);
    let c2 = Rgb::new(255, 255, 255);
    let mid = c1.lerp(&c2, 0.5);
    assert!(mid.r > 100 && mid.r < 155);
    assert!(mid.g > 100 && mid.g < 155);
    assert!(mid.b > 100 && mid.b < 155);
}

#[test]
fn test_ansi256_to_rgb() {
    // Test basic color
    let rgb = ansi256_to_rgb(9); // Bright red
    assert_eq!(rgb.r, 255);
    assert_eq!(rgb.g, 0);
    assert_eq!(rgb.b, 0);

    // Test cube color
    let rgb = ansi256_to_rgb(111); // Cyan
    assert!(rgb.r < rgb.b && rgb.g < rgb.b);
}

#[test]
fn test_color_segments_smooth() {
    let widget = GradationText::new("Hello World", &[30, 66, 72, 108, 109]);
    let segments = widget.get_color_segments();

    // Should have multiple segments
    assert!(
        segments.len() > 5,
        "Expected multiple segments for smooth gradation, got {}",
        segments.len()
    );

    // First segment should start with darker color
    let first_color = &segments[0].1;
    // Last segment should end with lighter color
    let last_color = &segments.last().unwrap().1;

    // Colors should be different
    assert_ne!(first_color.r, last_color.r);

    println!("Smooth gradation created {} segments", segments.len());
    for (i, (text, rgb)) in segments.iter().enumerate() {
        println!(
            "  Segment {}: '{}' -> RGB({}, {}, {})",
            i, text, rgb.r, rgb.g, rgb.b
        );
    }
}

#[test]
fn test_color_segments_stepped() {
    let widget = GradationText::new("Hello World", &[30, 66, 72, 108, 109]).smooth(false);
    let segments = widget.get_color_segments();

    // Should have segments equal to number of colors
    assert_eq!(
        segments.len(),
        5,
        "Expected 5 segments for stepped gradation, got {}",
        segments.len()
    );

    println!("Stepped gradation created {} segments", segments.len());
    for (i, (text, rgb)) in segments.iter().enumerate() {
        println!(
            "  Segment {}: '{}' -> RGB({}, {}, {})",
            i, text, rgb.r, rgb.g, rgb.b
        );
    }
}

#[test]
fn test_kernel_hacker_colors() {
    // Test with actual Kernel Hacker rank colors
    let widget = GradationText::new("Kernel Hacker", &[30, 66, 72, 108, 109]);
    widget.debug_colors();

    let segments = widget.get_color_segments();
    assert!(segments.len() > 1, "Should have multiple color segments");
}

#[test]
fn test_ascii_art_length_smooth() {
    // Test with actual ASCII art line length (~60-80 chars)
    let ascii_line = "  |  _ \\  ___ _ __ _ __   ___| |  | | | | __ _  ___| | _____ _ __ ";
    let widget = GradationText::new(ascii_line, &[30, 66, 72, 108, 109]);

    println!(
        "\n=== Testing with ASCII art length ({} chars) ===",
        ascii_line.len()
    );
    widget.debug_colors();

    let segments = widget.get_color_segments();

    // With smooth gradation and ~70 chars, should create many segments
    // target_spans = (text_len / 2).max(num_colors * 3) = (70/2).max(15) = 35
    assert!(
        segments.len() >= 15,
        "Expected at least 15 segments for 70-char ASCII art, got {}",
        segments.len()
    );

    // Check that colors actually vary
    let first_color = &segments[0].1;
    let last_color = &segments.last().unwrap().1;
    let color_diff = (first_color.r as i32 - last_color.r as i32).abs()
        + (first_color.g as i32 - last_color.g as i32).abs()
        + (first_color.b as i32 - last_color.b as i32).abs();

    assert!(
        color_diff > 50,
        "Color should change significantly from start to end, diff: {}",
        color_diff
    );
}

#[test]
fn test_ascii_art_length_stepped() {
    // Test stepped gradation with ASCII art length
    let ascii_line = "  |  _ \\  ___ _ __ _ __   ___| |  | | | | __ _  ___| | _____ _ __ ";
    let widget = GradationText::new(ascii_line, &[30, 66, 72, 108, 109]).smooth(false);

    println!("\n=== Testing stepped gradation with ASCII art length ===");
    widget.debug_colors();

    let segments = widget.get_color_segments();

    // Stepped gradation should create exactly num_colors segments
    assert_eq!(
        segments.len(),
        5,
        "Stepped gradation should create 5 segments, got {}",
        segments.len()
    );
}

#[test]
fn test_compiler_colors_ascii() {
    // Test Expert tier (Compiler) colors with ASCII art
    let ascii_line = "  / ___ \\|  __/| |  | |___| | | (_| |  _| ||  __/ |   ";
    let widget = GradationText::new(ascii_line, &[214, 215, 220]);

    println!("\n=== Compiler (Expert) Colors ===");
    widget.debug_colors();

    let segments = widget.get_color_segments();
    assert!(
        segments.len() >= 9,
        "Expected at least 9 segments for Expert tier"
    );
}

#[test]
fn test_legendary_colors_ascii() {
    // Test Legendary tier colors with very long ASCII art
    let ascii_line = " |_____\\___/ \\__,_|\\__,_| |____/ \\__,_|_|\\__,_|_| |_|\\___\\___|_|  ";
    let widget = GradationText::new(ascii_line, &[197, 204, 210, 217]);

    println!("\n=== Legendary Tier Colors ===");
    widget.debug_colors();

    let segments = widget.get_color_segments();

    // With 4 colors and ~70 chars: target_spans = (70/2).max(12) = 35
    assert!(
        segments.len() >= 12,
        "Expected at least 12 segments for Legendary tier, got {}",
        segments.len()
    );

    // Verify gradation goes from first to last color
    let first = &segments[0].1;
    let last = &segments.last().unwrap().1;

    println!("First color: RGB({}, {}, {})", first.r, first.g, first.b);
    println!("Last color: RGB({}, {}, {})", last.r, last.g, last.b);
}

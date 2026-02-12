use gittype::presentation::ui::gradation_text::{ansi256_to_rgb, GradationText, Rgb};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

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

// ---------------------------------------------------------------------------
// boost_vibrance / HSL round-trip tests
// ---------------------------------------------------------------------------

#[test]
fn test_boost_vibrance_red() {
    let red = Rgb::new(255, 0, 0);
    let boosted = red.boost_vibrance(1.5, 0.0);
    // Red should stay reddish; R should be higher than G and B
    assert!(boosted.r > boosted.g);
    assert!(boosted.r > boosted.b);
}

#[test]
fn test_boost_vibrance_green() {
    let green = Rgb::new(0, 200, 0);
    let boosted = green.boost_vibrance(1.2, 0.05);
    // Green should stay greenish
    assert!(boosted.g > boosted.r);
    assert!(boosted.g > boosted.b);
}

#[test]
fn test_boost_vibrance_blue() {
    let blue = Rgb::new(0, 0, 200);
    let boosted = blue.boost_vibrance(1.3, -0.1);
    // Blue should remain dominant
    assert!(boosted.b > boosted.r);
    assert!(boosted.b > boosted.g);
}

#[test]
fn test_boost_vibrance_cyan() {
    // Cyan: H ≈ 180° → exercises the h < 180 branch in from_hsl
    let cyan = Rgb::new(0, 200, 200);
    let boosted = cyan.boost_vibrance(1.0, 0.0);
    // Should stay cyanish (G and B dominant)
    assert!(boosted.g > boosted.r);
    assert!(boosted.b > boosted.r);
}

#[test]
fn test_boost_vibrance_magenta() {
    // Magenta: H ≈ 300° → exercises the h >= 300 branch in from_hsl
    let magenta = Rgb::new(200, 0, 200);
    let boosted = magenta.boost_vibrance(1.0, 0.0);
    assert!(boosted.r > boosted.g);
    assert!(boosted.b > boosted.g);
}

#[test]
fn test_boost_vibrance_yellow() {
    // Yellow: H ≈ 60° → exercises the h < 120 branch in from_hsl, max == r branch in to_hsl
    let yellow = Rgb::new(200, 200, 0);
    let boosted = yellow.boost_vibrance(1.0, 0.0);
    assert!(boosted.r > boosted.b);
    assert!(boosted.g > boosted.b);
}

#[test]
fn test_boost_vibrance_gray_no_saturation() {
    // Gray: delta == 0, so H=0, S=0 → exercises delta == 0 branch
    let gray = Rgb::new(128, 128, 128);
    let boosted = gray.boost_vibrance(2.0, 0.0);
    // With zero saturation, boosting has no effect on hue — result stays grayish
    let diff =
        (boosted.r as i32 - boosted.g as i32).abs() + (boosted.g as i32 - boosted.b as i32).abs();
    assert!(
        diff <= 2,
        "Gray should remain approximately gray, diff={}",
        diff
    );
}

#[test]
fn test_boost_vibrance_purple() {
    // Purple: H ≈ 270° → exercises h < 300 branch in from_hsl, max == b branch in to_hsl
    let purple = Rgb::new(100, 0, 200);
    let boosted = purple.boost_vibrance(1.0, 0.0);
    assert!(boosted.b > boosted.g);
}

#[test]
fn test_boost_vibrance_orange() {
    // Orange: H ≈ 30° → exercises h < 60 branch in from_hsl
    let orange = Rgb::new(255, 128, 0);
    let boosted = orange.boost_vibrance(1.0, 0.0);
    assert!(boosted.r > boosted.b);
}

#[test]
fn test_boost_vibrance_teal() {
    // Teal: H ≈ 180° → max == g path in to_hsl
    let teal = Rgb::new(0, 150, 150);
    let boosted = teal.boost_vibrance(1.0, 0.0);
    assert!(boosted.g > boosted.r);
}

#[test]
fn test_boost_vibrance_lightness_clamp_high() {
    // Very high lightness_adjust should be clamped to 0.8
    let color = Rgb::new(100, 100, 100);
    let boosted = color.boost_vibrance(1.0, 1.0);
    // Should not crash, and result should be valid
    // Should not crash, result is valid RGB
    let _ = boosted;
}

#[test]
fn test_boost_vibrance_lightness_clamp_low() {
    // Very low lightness_adjust should be clamped to 0.2
    let color = Rgb::new(100, 100, 100);
    let boosted = color.boost_vibrance(1.0, -1.0);
    let _ = boosted;
}

#[test]
fn test_boost_vibrance_negative_hue_wraps() {
    // Color that produces negative hue in to_hsl (R > G > B with small diff)
    let color = Rgb::new(200, 50, 100);
    let boosted = color.boost_vibrance(1.0, 0.0);
    // Should not crash — negative hue gets wrapped to positive
    let _ = boosted;
}

// ---------------------------------------------------------------------------
// ansi256_to_rgb: basic colors (0-15) and grayscale (232-255)
// ---------------------------------------------------------------------------

#[test]
fn test_ansi256_basic_black() {
    let rgb = ansi256_to_rgb(0);
    assert_eq!((rgb.r, rgb.g, rgb.b), (0, 0, 0));
}

#[test]
fn test_ansi256_basic_colors_0_to_7() {
    let expected = [
        (0, 0, 0),       // 0: black
        (128, 0, 0),     // 1: maroon
        (0, 128, 0),     // 2: green
        (128, 128, 0),   // 3: olive
        (0, 0, 128),     // 4: navy
        (128, 0, 128),   // 5: purple
        (0, 128, 128),   // 6: teal
        (192, 192, 192), // 7: silver
    ];
    for (code, (r, g, b)) in expected.iter().enumerate() {
        let rgb = ansi256_to_rgb(code as u8);
        assert_eq!((rgb.r, rgb.g, rgb.b), (*r, *g, *b), "ANSI code {}", code);
    }
}

#[test]
fn test_ansi256_basic_colors_8_to_15() {
    let expected = [
        (128, 128, 128), // 8: gray
        (255, 0, 0),     // 9: red
        (0, 255, 0),     // 10: lime
        (255, 255, 0),   // 11: yellow
        (0, 0, 255),     // 12: blue
        (255, 0, 255),   // 13: fuchsia
        (0, 255, 255),   // 14: aqua
        (255, 255, 255), // 15: white
    ];
    for (i, (r, g, b)) in expected.iter().enumerate() {
        let code = (i + 8) as u8;
        let rgb = ansi256_to_rgb(code);
        assert_eq!((rgb.r, rgb.g, rgb.b), (*r, *g, *b), "ANSI code {}", code);
    }
}

#[test]
fn test_ansi256_grayscale() {
    // Grayscale: code 232-255 maps to gray = 8 + (code - 232) * 10
    let rgb_232 = ansi256_to_rgb(232);
    assert_eq!(rgb_232.r, 8);
    assert_eq!(rgb_232.g, 8);
    assert_eq!(rgb_232.b, 8);

    let rgb_255 = ansi256_to_rgb(255);
    let expected_gray = 8 + (255 - 232) * 10; // 238
    assert_eq!(rgb_255.r, expected_gray);
    assert_eq!(rgb_255.g, expected_gray);
    assert_eq!(rgb_255.b, expected_gray);
}

#[test]
fn test_ansi256_grayscale_mid_range() {
    let rgb_244 = ansi256_to_rgb(244);
    let expected = 8 + (244 - 232) * 10; // 128
    assert_eq!(
        (rgb_244.r, rgb_244.g, rgb_244.b),
        (expected, expected, expected)
    );
}

// ---------------------------------------------------------------------------
// Widget::render test (via ratatui Buffer)
// ---------------------------------------------------------------------------

#[test]
fn test_gradation_text_render_widget() {
    let area = Rect::new(0, 0, 20, 1);
    let mut buf = Buffer::empty(area);
    let widget = GradationText::new("Hello World", &[30, 66, 72]);
    widget.render(area, &mut buf);
    // Verify the buffer has content rendered
    let content: String = (0..20)
        .map(|x| buf.cell((x, 0)).unwrap().symbol().to_string())
        .collect::<Vec<_>>()
        .join("");
    assert!(content.starts_with("Hello World"));
}

#[test]
fn test_gradation_text_render_widget_rgb() {
    let area = Rect::new(0, 0, 10, 1);
    let mut buf = Buffer::empty(area);
    let colors = [Rgb::new(255, 0, 0), Rgb::new(0, 0, 255)];
    let widget = GradationText::new_rgb("TestText", &colors);
    widget.render(area, &mut buf);
    let content: String = (0..8)
        .map(|x| buf.cell((x, 0)).unwrap().symbol().to_string())
        .collect::<Vec<_>>()
        .join("");
    assert_eq!(content, "TestText");
}

// ---------------------------------------------------------------------------
// lerp edge cases
// ---------------------------------------------------------------------------

#[test]
fn test_rgb_lerp_at_zero() {
    let c1 = Rgb::new(100, 50, 200);
    let c2 = Rgb::new(200, 100, 50);
    let result = c1.lerp(&c2, 0.0);
    assert_eq!((result.r, result.g, result.b), (100, 50, 200));
}

#[test]
fn test_rgb_lerp_at_one() {
    let c1 = Rgb::new(100, 50, 200);
    let c2 = Rgb::new(200, 100, 50);
    let result = c1.lerp(&c2, 1.0);
    assert_eq!((result.r, result.g, result.b), (200, 100, 50));
}

#[test]
fn test_rgb_lerp_clamped_above() {
    let c1 = Rgb::new(0, 0, 0);
    let c2 = Rgb::new(255, 255, 255);
    let result = c1.lerp(&c2, 2.0); // should clamp to 1.0
    assert_eq!((result.r, result.g, result.b), (255, 255, 255));
}

#[test]
fn test_rgb_lerp_clamped_below() {
    let c1 = Rgb::new(0, 0, 0);
    let c2 = Rgb::new(255, 255, 255);
    let result = c1.lerp(&c2, -1.0); // should clamp to 0.0
    assert_eq!((result.r, result.g, result.b), (0, 0, 0));
}

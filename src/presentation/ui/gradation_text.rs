use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// RGB color representation
#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Linear interpolation between two colors
    pub fn lerp(&self, other: &Rgb, t: f64) -> Rgb {
        let t = t.clamp(0.0, 1.0);
        Rgb {
            r: (self.r as f64 + (other.r as f64 - self.r as f64) * t) as u8,
            g: (self.g as f64 + (other.g as f64 - self.g as f64) * t) as u8,
            b: (self.b as f64 + (other.b as f64 - self.b as f64) * t) as u8,
        }
    }

    /// Convert to ratatui Color
    pub fn to_color(&self) -> Color {
        Color::Rgb(self.r, self.g, self.b)
    }

    /// Convert RGB to HSL (Hue, Saturation, Lightness)
    fn to_hsl(self) -> (f64, f64, f64) {
        let r = self.r as f64 / 255.0;
        let g = self.g as f64 / 255.0;
        let b = self.b as f64 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let l = (max + min) / 2.0;

        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        (h, s, l)
    }

    /// Convert HSL to RGB
    fn from_hsl(h: f64, s: f64, l: f64) -> Self {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Rgb {
            r: ((r + m) * 255.0).round() as u8,
            g: ((g + m) * 255.0).round() as u8,
            b: ((b + m) * 255.0).round() as u8,
        }
    }

    /// Boost saturation and adjust lightness for more vibrant colors
    pub fn boost_vibrance(&self, saturation_boost: f64, lightness_adjust: f64) -> Self {
        let (h, s, l) = self.to_hsl();

        // Boost saturation (multiply by boost factor, clamped to 1.0)
        let new_s = (s * saturation_boost).min(1.0);

        // Adjust lightness (keep it in reasonable range)
        let new_l = (l + lightness_adjust).clamp(0.2, 0.8);

        Self::from_hsl(h, new_s, new_l)
    }
}

/// A widget that renders text with horizontal color gradation
pub struct GradationText<'a> {
    text: &'a str,
    colors: Vec<Rgb>,
    alignment: Alignment,
    smooth: bool,
}

impl<'a> GradationText<'a> {
    /// Create a new GradationText widget with ANSI 256 color codes
    ///
    /// # Arguments
    /// * `text` - The text to render
    /// * `colors` - Array of ANSI 256 color codes to use for gradation (left to right)
    pub fn new(text: &'a str, colors: &'a [u8]) -> Self {
        let rgb_colors = colors.iter().map(|&c| ansi256_to_rgb(c)).collect();
        Self {
            text,
            colors: rgb_colors,
            alignment: Alignment::Left,
            smooth: true,
        }
    }

    /// Create a new GradationText widget with RGB colors
    ///
    /// # Arguments
    /// * `text` - The text to render
    /// * `colors` - Array of RGB colors to use for gradation (left to right)
    pub fn new_rgb(text: &'a str, colors: &'a [Rgb]) -> Self {
        Self {
            text,
            colors: colors.to_vec(),
            alignment: Alignment::Left,
            smooth: true,
        }
    }

    /// Set the alignment of the text
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Enable or disable smooth gradation (interpolation between colors)
    pub fn smooth(mut self, smooth: bool) -> Self {
        self.smooth = smooth;
        self
    }

    /// Get color information for debugging/testing
    /// Returns a vector of (text_segment, rgb_color) tuples
    pub fn get_color_segments(&self) -> Vec<(String, Rgb)> {
        let line = self.apply_gradation();
        line.spans
            .into_iter()
            .filter_map(|span| {
                if let Style {
                    fg: Some(Color::Rgb(r, g, b)),
                    ..
                } = span.style
                {
                    Some((span.content.to_string(), Rgb::new(r, g, b)))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Print color distribution for debugging
    pub fn debug_colors(&self) {
        let segments = self.get_color_segments();
        println!("=== Color Distribution ({} segments) ===", segments.len());
        for (i, (text, rgb)) in segments.iter().enumerate() {
            println!(
                "Segment {}: '{}' -> RGB({}, {}, {})",
                i, text, rgb.r, rgb.g, rgb.b
            );
        }
    }

    /// Apply gradation colors to the text
    fn apply_gradation(&self) -> Line<'a> {
        if self.colors.is_empty() {
            return Line::from(self.text);
        }

        if self.colors.len() == 1 {
            return Line::from(vec![Span::styled(
                self.text,
                Style::default().fg(self.colors[0].to_color()),
            )]);
        }

        let text_len = self.text.len();
        if text_len == 0 {
            return Line::from("");
        }

        if self.smooth {
            self.apply_smooth_gradation(text_len)
        } else {
            self.apply_stepped_gradation(text_len)
        }
    }

    /// Apply smooth gradation with color interpolation
    fn apply_smooth_gradation(&self, text_len: usize) -> Line<'a> {
        let mut spans = Vec::new();
        let num_colors = self.colors.len();

        // Determine target number of spans (more spans = smoother gradation)
        // Aim for about 1 span per 2-3 characters, but at least num_colors * 3
        let target_spans = ((text_len / 2).max(num_colors * 3)).min(text_len);
        let chars_per_span = if target_spans > 0 {
            (text_len as f64 / target_spans as f64).ceil() as usize
        } else {
            text_len
        };

        let mut current_segment = String::new();
        let mut segment_start_idx = 0;

        for (char_idx, ch) in self.text.chars().enumerate() {
            current_segment.push(ch);

            // Create a new span every chars_per_span characters
            let should_push = (char_idx + 1) % chars_per_span == 0 || char_idx == text_len - 1;

            if should_push && !current_segment.is_empty() {
                // Calculate the color for the middle of the segment
                let mid_idx = segment_start_idx + current_segment.len() / 2;
                let mid_position = if text_len > 1 {
                    mid_idx as f64 / (text_len - 1) as f64
                } else {
                    0.0
                };

                // Find which two colors to interpolate between
                let color_position = mid_position * (num_colors - 1) as f64;
                let color_idx = color_position.floor() as usize;
                let next_color_idx = (color_idx + 1).min(num_colors - 1);
                let t = color_position - color_idx as f64;

                // Interpolate between the two colors
                let segment_color = self.colors[color_idx].lerp(&self.colors[next_color_idx], t);

                spans.push(Span::styled(
                    current_segment.clone(),
                    Style::default().fg(segment_color.to_color()),
                ));

                current_segment.clear();
                segment_start_idx = char_idx + 1;
            }
        }

        Line::from(spans)
    }

    /// Apply stepped gradation (original behavior)
    fn apply_stepped_gradation(&self, text_len: usize) -> Line<'a> {
        let mut spans = Vec::new();
        let num_colors = self.colors.len();
        let chars_per_segment = text_len as f64 / num_colors as f64;

        let mut current_color_idx = 0;
        let mut current_segment = String::new();

        for (char_idx, ch) in self.text.chars().enumerate() {
            let target_color_idx =
                ((char_idx as f64 / chars_per_segment).floor() as usize).min(num_colors - 1);

            if target_color_idx != current_color_idx && !current_segment.is_empty() {
                spans.push(Span::styled(
                    current_segment.clone(),
                    Style::default().fg(self.colors[current_color_idx].to_color()),
                ));
                current_segment.clear();
                current_color_idx = target_color_idx;
            }

            current_segment.push(ch);
        }

        if !current_segment.is_empty() {
            spans.push(Span::styled(
                current_segment,
                Style::default().fg(self.colors[current_color_idx].to_color()),
            ));
        }

        Line::from(spans)
    }
}

impl<'a> Widget for GradationText<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let line = self.apply_gradation();
        let paragraph = Paragraph::new(line).alignment(self.alignment);
        paragraph.render(area, buf);
    }
}

/// Convert ANSI 256 color code to RGB
/// Based on the standard ANSI 256 color palette
pub fn ansi256_to_rgb(code: u8) -> Rgb {
    match code {
        // 16 basic colors (0-15)
        0 => Rgb::new(0, 0, 0),
        1 => Rgb::new(128, 0, 0),
        2 => Rgb::new(0, 128, 0),
        3 => Rgb::new(128, 128, 0),
        4 => Rgb::new(0, 0, 128),
        5 => Rgb::new(128, 0, 128),
        6 => Rgb::new(0, 128, 128),
        7 => Rgb::new(192, 192, 192),
        8 => Rgb::new(128, 128, 128),
        9 => Rgb::new(255, 0, 0),
        10 => Rgb::new(0, 255, 0),
        11 => Rgb::new(255, 255, 0),
        12 => Rgb::new(0, 0, 255),
        13 => Rgb::new(255, 0, 255),
        14 => Rgb::new(0, 255, 255),
        15 => Rgb::new(255, 255, 255),
        // 216 color cube (16-231)
        16..=231 => {
            let idx = code - 16;
            let r = (idx / 36) % 6;
            let g = (idx / 6) % 6;
            let b = idx % 6;
            Rgb::new(
                if r == 0 { 0 } else { 55 + r * 40 },
                if g == 0 { 0 } else { 55 + g * 40 },
                if b == 0 { 0 } else { 55 + b * 40 },
            )
        }
        // Grayscale (232-255)
        232..=255 => {
            let gray = 8 + (code - 232) * 10;
            Rgb::new(gray, gray, gray)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(line.spans.len() > 0);
    }

    #[test]
    fn test_multiple_colors_stepped() {
        let widget = GradationText::new("Hello", &[30, 66, 72]).smooth(false);
        let line = widget.apply_gradation();
        // Should have spans equal to number of colors
        assert!(line.spans.len() > 0);
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
        assert!(line.spans.len() > 0);
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
        let ascii_line =
            " |_____\\___/ \\__,_|\\__,_| |____/ \\__,_|_|\\__,_|_| |_|\\___\\___|_|  ";
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
}

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
    pub fn apply_gradation(&self) -> Line<'a> {
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

use std::ops::Range;

pub struct IndentProcessor;

impl IndentProcessor {
    pub fn extract_and_normalize_indentation<'a>(
        content: &str,
        source_code: &'a str,
        line_row: usize,
        indent_byte_length: usize,
        line_cache: &[usize],
    ) -> (String, &'a str) {
        let original_indent_chars = if indent_byte_length == 0 {
            ""
        } else {
            Self::extract_line_indent_chars_cached(
                source_code,
                line_row,
                indent_byte_length,
                line_cache,
            )
        };

        let normalized_content = if original_indent_chars.is_empty() {
            content.to_owned()
        } else {
            format!("{}{}", original_indent_chars, content)
        };

        (normalized_content, original_indent_chars)
    }

    fn extract_line_indent_chars_cached<'a>(
        source_code: &'a str,
        line_row: usize,
        indent_byte_length: usize,
        line_cache: &[usize],
    ) -> &'a str {
        let line_range = Self::get_line_range_from_cache(line_row, line_cache, source_code.len());

        let line_slice = source_code.get(line_range).unwrap_or("");
        let line = line_slice.strip_suffix('\n').unwrap_or(line_slice);

        line.get(..indent_byte_length).unwrap_or(line)
    }

    fn get_line_range_from_cache(
        line_row: usize,
        line_cache: &[usize],
        source_len: usize,
    ) -> Range<usize> {
        let line_start = line_cache.get(line_row).copied().unwrap_or(source_len);
        let line_end = line_cache.get(line_row + 1).copied().unwrap_or(source_len);

        line_start..line_end
    }
}

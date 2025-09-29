pub struct CacheBuilder;

impl CacheBuilder {
    pub fn build_byte_to_char_cache(source_code: &str) -> Vec<usize> {
        let char_positions: Vec<_> = source_code.char_indices().collect();
        let mut cache = vec![0; source_code.len() + 1];
        
        char_positions
            .iter()
            .enumerate()
            .for_each(|(char_idx, &(byte_pos, _))| {
                // Fill from previous position to current byte position
                (cache.len().min(byte_pos)..=byte_pos.min(cache.len() - 1))
                    .for_each(|pos| cache[pos] = char_idx);
            });
        
        // Fill remaining positions with final character count
        let final_char_count = char_positions.len();
        cache
            .iter_mut()
            .skip(char_positions.last().map(|(pos, _)| *pos + 1).unwrap_or(0))
            .for_each(|pos| *pos = final_char_count);
        
        cache
    }

    pub fn build_line_cache(source_code: &str) -> Vec<usize> {
        std::iter::once(0) // Line 0 starts at byte 0
            .chain(
                source_code
                    .bytes()
                    .enumerate()
                    .filter(|(_, byte)| *byte == b'\n')
                    .map(|(i, _)| i + 1) // Next line starts after the newline
            )
            .collect()
    }

    pub fn byte_to_char_cached(cache: &[usize], byte_pos: usize) -> usize {
        if byte_pos >= cache.len() {
            cache.last().copied().unwrap_or(0)
        } else {
            cache[byte_pos]
        }
    }
}
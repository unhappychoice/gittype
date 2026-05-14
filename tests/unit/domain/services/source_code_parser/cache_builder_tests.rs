use gittype::domain::services::source_code_parser::CacheBuilder;

#[test]
fn byte_to_char_cached_returns_last_value_for_out_of_bounds_byte() {
    let cache = CacheBuilder::build_byte_to_char_cache("éx");

    assert_eq!(CacheBuilder::byte_to_char_cached(&cache, usize::MAX), 2);
    assert_eq!(CacheBuilder::byte_to_char_cached(&[], 0), 0);
}

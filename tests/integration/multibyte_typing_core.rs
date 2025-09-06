use crate::integration::languages::typing_core_common::*;
use crate::typing_core_test_with_parser;
use gittype::game::typing_core::ProcessingOptions;

// Test multibyte characters in various positions with comments

typing_core_test_with_parser!(
    multibyte_japanese_comment_basic,
    "rust",
    r#"fn main() {
    // こんにちは世界 - Hello World in Japanese
    println!("こんにちは");
}"#
);

typing_core_test_with_parser!(
    multibyte_jemoji_in_comments,
    "rust",
    r#"fn test() {
    // 🚀 Rocket function with emoji 🎉
    // ↵ Arrow symbols in comments ↵
    let result = "success";
}"#
);

typing_core_test_with_parser!(
    multibyte_jchinese_variable_names,
    "rust",
    r#"fn 计算器() {
    // 中文注释：这是一个计算函数
    let 数值 = 42;
    // More comments with 中文字符
    return 数值;
}"#
);

typing_core_test_with_parser!(
    multibyte_jmixed,
    "javascript",
    r#"// ユーザー管理システム
function ユーザー作成(名前, 年齢) {
    // エラーチェック処理 
    if (!名前 || 年齢 < 0) {
        // 無効なパラメータエラー
        throw new Error("無効なパラメータ");
    }
    
    // 正常処理: ユーザーオブジェクト作成
    return {
        名前: 名前,
        年齢: 年齢,
        作成日時: new Date()
    };
}"#
);

typing_core_test_with_parser!(
    multibyte_jarabic_text_mixed,
    "python",
    r#"def معالج_النصوص():
    # هذه دالة لمعالجة النصوص العربية
    نص = "مرحبا بالعالم"
    # تحويل النص إلى أحرف كبيرة
    return نص.upper()
    
# اختبار الدالة
print(معالج_النصوص())"#
);

typing_core_test_with_parser!(
    multibyte_jkorean_with_symbols,
    "rust",
    r#"// 한국어 테스트 함수
fn 테스트_함수() -> String {
    // 다양한 유니코드 문자들: ★☆♥♦♣♠
    let 메시지 = "안녕하세요! 👋";
    // ↵ 기호와 함께 ↵
    format!("결과: {}", 메시지)
}"#
);

typing_core_test_with_parser!(
    multibyte_at_comment_boundaries,
    "rust",
    r#"fn boundary_test() {
    // Start with emoji 🎯 and end with unicode ✨
    let x = 42;
    // 日本語の境界テスト：コメントの始まりと終わりにマルチバイト文字
    let y = "test";
    // Mixed: Hello 世界 and symbols ↵↵↵
}"#
);

typing_core_test_with_parser!(
    multibyte_jnewline_symbols,
    "rust",
    r#"// Unicode comments with ↵ symbols
fn test() {
    // これは日本語のコメント↵です
    println!("Hello");
    // More text with 中文 characters ↵
    let value = 123;
}"#,
    ProcessingOptions {
        preserve_empty_lines: true,
        add_newline_symbols: true,
        highlight_special_chars: true,
    }
);

typing_core_test_with_parser!(
    multibyte_jtab_replacement,
    "rust",
    r#"fn tab_test() {
	// タブ文字を含む日本語コメント
	let 変数 = "値";
	// 	インデントされた中文注释
	return 変数;
}"#,
    ProcessingOptions {
        preserve_empty_lines: true,
        add_newline_symbols: true,
        highlight_special_chars: true,
    }
);

typing_core_test_with_parser!(
    multibyte_jcomplex_mixing,
    "javascript",
    r#"/**
 * 多言語対応のテスト関数
 * Multi-language test function
 * 多语言测试函数
 */
function 多言語テスト(パラメータ) {
    // ログ出力: 开始处理 with emoji 🔄
    console.log("処理開始:", パラメータ);
    
    // 各種文字の混在テスト
    const 結果 = {
        日本語: "こんにちは世界",
        中文: "你好世界", 
        العربية: "مرحبا بالعالم",
        한국어: "안녕 세계",
        emoji: "🌍🎉✨",
        symbols: "↵→←↑↓"
    };
    
    // 戻り値の返却 ↵
    return 結果;
}"#
);

typing_core_test_with_parser!(
    multibyte_jrtl_text_comments,
    "python",
    r#"def test_rtl():
    # English then Arabic: مرحبا بكم في البرنامج
    # Hebrew: שלום עולם וברוכים הבאים  
    # Mixed RTL and LTR: Hello שלום مرحبا World
    text = "testing RTL in comments"
    return text"#
);

typing_core_test_with_parser!(
    multibyte_jzero_width_characters,
    "rust",
    r#"fn zero_width_test() {
    // Comment with zero-width joiner: ‍ and non-joiner: ‌
    // Soft hyphen test: test­ing (invisible hyphen)
    let text = "normal text";
    // Combining characters: a̍b̊c̃ in comment
    return text;
}"#
);

#[cfg(test)]
mod multibyte_comment_range_tests {
    use super::*;
    use gittype::game::typing_core::TypingCore;

    #[test]
    fn test_comment_ranges_with_multibyte_boundaries() {
        // Test comment ranges when multibyte chars are at exact boundaries
        let code = r#"// 🎯これはテスト
fn main() {
    // コメント終了→
}"#;

        let comment_ranges = find_comment_ranges_with_parser(code, "rust");
        let typing_core = TypingCore::new(code, &comment_ranges, ProcessingOptions::default());

        let display_ranges = typing_core.display_comment_ranges();

        println!("Original code: {:?}", code);
        println!("Comment ranges: {:?}", comment_ranges);
        println!("Display text: {:?}", typing_core.text_to_display());
        println!("Display comment ranges: {:?}", display_ranges);

        // Verify each comment range produces valid strings
        for (i, &(start, end)) in display_ranges.iter().enumerate() {
            let display_text = typing_core.text_to_display();
            assert!(
                end <= display_text.len(),
                "Comment {} range ({}, {}) exceeds display text length {}",
                i,
                start,
                end,
                display_text.len()
            );

            let comment = &display_text[start..end];
            println!("Comment {}: {:?}", i, comment);

            // Should not panic and should contain expected content
            assert!(!comment.is_empty(), "Comment {} should not be empty", i);
        }
    }

    #[test]
    fn test_multibyte_position_mapping() {
        // Test position mapping accuracy with various multibyte characters
        let test_cases = vec![
            ("// 🚀 emoji", "rust"),
            ("// 中文注释", "rust"),
            ("// مرحبا", "rust"),
            ("// 한글 주석", "rust"),
            ("// Mix: 🎯中文👋", "rust"),
        ];

        for (code, lang) in test_cases {
            println!("\n=== Testing: {:?} ===", code);

            let comment_ranges = find_comment_ranges_with_parser(code, lang);
            let typing_core = TypingCore::new(code, &comment_ranges, ProcessingOptions::default());

            let display_ranges = typing_core.display_comment_ranges();

            for &(start, end) in &display_ranges {
                let display_text = typing_core.text_to_display();

                // Verify we can safely slice at these positions
                let comment = &display_text[start..end];
                println!("  Comment: {:?}", comment);

                // Should contain the comment marker
                assert!(comment.contains("//"), "Should contain '//' marker");
            }
        }
    }

    #[test]
    fn test_multibyte_with_newline_symbols() {
        let code = r#"// マルチバイト with ↵ symbols
fn test() {
    // 🎯 emoji comment ↵
}"#;

        let comment_ranges = find_comment_ranges_with_parser(code, "rust");
        let options = ProcessingOptions {
            preserve_empty_lines: true,
            add_newline_symbols: true,
            highlight_special_chars: true,
        };

        let typing_core = TypingCore::new(code, &comment_ranges, options);
        let display_ranges = typing_core.display_comment_ranges();

        println!("Code with newline symbols: {:?}", code);
        println!("Display text: {:?}", typing_core.text_to_display());
        println!("Comment ranges: {:?}", display_ranges);

        // Verify all comments are complete
        for &(start, end) in &display_ranges {
            let display_text = typing_core.text_to_display();
            let comment = &display_text[start..end];

            println!("Comment: {:?}", comment);

            // Should be valid UTF-8 and contain expected content
            assert!(comment.starts_with("//"), "Comment should start with //");
        }
    }
}

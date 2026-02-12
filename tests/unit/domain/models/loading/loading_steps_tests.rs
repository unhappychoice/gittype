use gittype::domain::models::loading::{
    CacheCheckStep, CloningStep, DatabaseInitStep, ExtractingStep, FinalizingStep, GeneratingStep,
    ScanningStep, Step, StepType,
};
use gittype::domain::models::theme::Theme;
use gittype::presentation::ui::Colors;

fn create_colors() -> Colors {
    Colors::new(Theme::default().dark)
}

// ============================================
// DatabaseInitStep
// ============================================

#[test]
fn database_init_step_type() {
    assert_eq!(DatabaseInitStep.step_type(), StepType::DatabaseInit);
}

#[test]
fn database_init_step_number() {
    assert_eq!(DatabaseInitStep.step_number(), 1);
}

#[test]
fn database_init_description() {
    assert!(!DatabaseInitStep.description().is_empty());
}

#[test]
fn database_init_step_name() {
    assert_eq!(DatabaseInitStep.step_name(), "Database Setup");
}

#[test]
fn database_init_no_progress() {
    assert!(!DatabaseInitStep.supports_progress());
    assert_eq!(DatabaseInitStep.progress_unit(), "");
}

#[test]
fn database_init_format_progress() {
    let result = DatabaseInitStep.format_progress(0, 0, 0.0, '⠋');
    assert!(result.contains("Initializing"));
}

#[test]
fn database_init_icon_completed() {
    let colors = create_colors();
    let (icon, color) = DatabaseInitStep.icon(false, true, &colors);
    assert_eq!(icon, "✓");
    assert_eq!(color, colors.success());
}

#[test]
fn database_init_icon_current() {
    let colors = create_colors();
    let (icon, color) = DatabaseInitStep.icon(true, false, &colors);
    assert_eq!(icon, "💾");
    assert_eq!(color, colors.warning());
}

#[test]
fn database_init_icon_pending() {
    let colors = create_colors();
    let (icon, color) = DatabaseInitStep.icon(false, false, &colors);
    assert_eq!(icon, "◦");
    assert_eq!(color, colors.text_secondary());
}

// ============================================
// CloningStep
// ============================================

#[test]
fn cloning_step_type() {
    assert_eq!(CloningStep.step_type(), StepType::Cloning);
}

#[test]
fn cloning_step_number() {
    assert_eq!(CloningStep.step_number(), 2);
}

#[test]
fn cloning_description() {
    assert!(!CloningStep.description().is_empty());
}

#[test]
fn cloning_step_name() {
    assert_eq!(CloningStep.step_name(), "Cloning repository");
}

#[test]
fn cloning_supports_progress() {
    assert!(CloningStep.supports_progress());
    assert_eq!(CloningStep.progress_unit(), "");
}

#[test]
fn cloning_format_progress() {
    let result = CloningStep.format_progress(0, 100, 0.5, '⠋');
    assert!(result.contains("50.0%"));
    assert!(result.contains('⠋'));
}

#[test]
fn cloning_icon_completed() {
    let colors = create_colors();
    let (icon, _) = CloningStep.icon(false, true, &colors);
    assert_eq!(icon, "✓");
}

#[test]
fn cloning_icon_current() {
    let colors = create_colors();
    let (icon, _) = CloningStep.icon(true, false, &colors);
    assert_eq!(icon, "⚡");
}

#[test]
fn cloning_icon_pending() {
    let colors = create_colors();
    let (icon, _) = CloningStep.icon(false, false, &colors);
    assert_eq!(icon, "◦");
}

// ============================================
// CacheCheckStep
// ============================================

#[test]
fn cache_check_step_type() {
    assert_eq!(CacheCheckStep.step_type(), StepType::CacheCheck);
}

#[test]
fn cache_check_step_number() {
    assert_eq!(CacheCheckStep.step_number(), 3);
}

#[test]
fn cache_check_description() {
    assert!(!CacheCheckStep.description().is_empty());
}

#[test]
fn cache_check_step_name() {
    assert_eq!(CacheCheckStep.step_name(), "Cache check");
}

#[test]
fn cache_check_supports_progress() {
    assert!(CacheCheckStep.supports_progress());
    assert_eq!(CacheCheckStep.progress_unit(), "challenges");
}

#[test]
fn cache_check_format_progress() {
    let result = CacheCheckStep.format_progress(5, 10, 0.5, '⠋');
    assert!(result.contains("50.0%"));
    assert!(result.contains("5/10"));
    assert!(result.contains("challenges"));
    assert!(result.contains("cache"));
}

#[test]
fn cache_check_icon_completed() {
    let colors = create_colors();
    let (icon, _) = CacheCheckStep.icon(false, true, &colors);
    assert_eq!(icon, "✓");
}

#[test]
fn cache_check_icon_current() {
    let colors = create_colors();
    let (icon, _) = CacheCheckStep.icon(true, false, &colors);
    assert_eq!(icon, "🔍");
}

// ============================================
// ScanningStep
// ============================================

#[test]
fn scanning_step_type() {
    assert_eq!(ScanningStep.step_type(), StepType::Scanning);
}

#[test]
fn scanning_step_number() {
    assert_eq!(ScanningStep.step_number(), 4);
}

#[test]
fn scanning_description() {
    assert!(!ScanningStep.description().is_empty());
}

#[test]
fn scanning_step_name() {
    assert_eq!(ScanningStep.step_name(), "Scanning repository");
}

#[test]
fn scanning_supports_progress() {
    assert!(ScanningStep.supports_progress());
    assert_eq!(ScanningStep.progress_unit(), "files");
}

#[test]
fn scanning_format_progress() {
    let result = ScanningStep.format_progress(30, 100, 0.3, '⠹');
    assert!(result.contains("30.0%"));
    assert!(result.contains("30/100"));
    assert!(result.contains("files"));
}

#[test]
fn scanning_icon_states() {
    let colors = create_colors();
    assert_eq!(ScanningStep.icon(false, true, &colors).0, "✓");
    assert_eq!(ScanningStep.icon(true, false, &colors).0, "⚡");
    assert_eq!(ScanningStep.icon(false, false, &colors).0, "◦");
}

// ============================================
// ExtractingStep
// ============================================

#[test]
fn extracting_step_type() {
    assert_eq!(ExtractingStep.step_type(), StepType::Extracting);
}

#[test]
fn extracting_step_number() {
    assert_eq!(ExtractingStep.step_number(), 5);
}

#[test]
fn extracting_description() {
    assert!(ExtractingStep.description().contains("functions"));
}

#[test]
fn extracting_step_name() {
    assert!(ExtractingStep.step_name().contains("Extracting"));
}

#[test]
fn extracting_supports_progress() {
    assert!(ExtractingStep.supports_progress());
    assert_eq!(ExtractingStep.progress_unit(), "files");
}

#[test]
fn extracting_format_progress() {
    let result = ExtractingStep.format_progress(7, 20, 0.35, '⠼');
    assert!(result.contains("35.0%"));
    assert!(result.contains("7/20"));
    assert!(result.contains("files"));
}

#[test]
fn extracting_icon_states() {
    let colors = create_colors();
    assert_eq!(ExtractingStep.icon(false, true, &colors).0, "✓");
    assert_eq!(ExtractingStep.icon(true, false, &colors).0, "⚡");
    assert_eq!(ExtractingStep.icon(false, false, &colors).0, "◦");
}

// ============================================
// GeneratingStep
// ============================================

#[test]
fn generating_step_type() {
    assert_eq!(GeneratingStep.step_type(), StepType::Generating);
}

#[test]
fn generating_step_number() {
    assert_eq!(GeneratingStep.step_number(), 7);
}

#[test]
fn generating_description() {
    assert!(GeneratingStep.description().contains("challenges"));
}

#[test]
fn generating_step_name() {
    assert_eq!(GeneratingStep.step_name(), "Generating challenges");
}

#[test]
fn generating_supports_progress() {
    assert!(GeneratingStep.supports_progress());
    assert_eq!(GeneratingStep.progress_unit(), "challenges");
}

#[test]
fn generating_format_progress() {
    let result = GeneratingStep.format_progress(50, 200, 0.25, '⠏');
    assert!(result.contains("25.0%"));
    assert!(result.contains("50/200"));
    assert!(result.contains("challenges"));
}

#[test]
fn generating_icon_states() {
    let colors = create_colors();
    assert_eq!(GeneratingStep.icon(false, true, &colors).0, "✓");
    assert_eq!(GeneratingStep.icon(true, false, &colors).0, "⚡");
    assert_eq!(GeneratingStep.icon(false, false, &colors).0, "◦");
}

// ============================================
// FinalizingStep
// ============================================

#[test]
fn finalizing_step_type() {
    assert_eq!(FinalizingStep.step_type(), StepType::Finalizing);
}

#[test]
fn finalizing_step_number() {
    assert_eq!(FinalizingStep.step_number(), 8);
}

#[test]
fn finalizing_description() {
    assert!(!FinalizingStep.description().is_empty());
}

#[test]
fn finalizing_step_name() {
    assert_eq!(FinalizingStep.step_name(), "Finalizing");
}

#[test]
fn finalizing_no_progress() {
    assert!(!FinalizingStep.supports_progress());
    assert_eq!(FinalizingStep.progress_unit(), "");
}

#[test]
fn finalizing_format_progress() {
    let result = FinalizingStep.format_progress(0, 0, 0.0, '⠋');
    assert!(result.contains("Finalizing"));
}

#[test]
fn finalizing_icon_states() {
    let colors = create_colors();
    assert_eq!(FinalizingStep.icon(false, true, &colors).0, "✓");
    assert_eq!(FinalizingStep.icon(true, false, &colors).0, "⚡");
    assert_eq!(FinalizingStep.icon(false, false, &colors).0, "◦");
}

// ============================================
// Icon color consistency across all steps
// ============================================

#[test]
fn all_steps_completed_icon_uses_success_color() {
    let colors = create_colors();
    let steps: Vec<Box<dyn Step>> = vec![
        Box::new(DatabaseInitStep),
        Box::new(CloningStep),
        Box::new(CacheCheckStep),
        Box::new(ScanningStep),
        Box::new(ExtractingStep),
        Box::new(GeneratingStep),
        Box::new(FinalizingStep),
    ];
    for step in &steps {
        let (_, color) = step.icon(false, true, &colors);
        assert_eq!(
            color,
            colors.success(),
            "Step {} should use success color when completed",
            step.step_name()
        );
    }
}

#[test]
fn all_steps_current_icon_uses_warning_color() {
    let colors = create_colors();
    let steps: Vec<Box<dyn Step>> = vec![
        Box::new(DatabaseInitStep),
        Box::new(CloningStep),
        Box::new(CacheCheckStep),
        Box::new(ScanningStep),
        Box::new(ExtractingStep),
        Box::new(GeneratingStep),
        Box::new(FinalizingStep),
    ];
    for step in &steps {
        let (_, color) = step.icon(true, false, &colors);
        assert_eq!(
            color,
            colors.warning(),
            "Step {} should use warning color when current",
            step.step_name()
        );
    }
}

#[test]
fn all_steps_pending_icon_uses_text_secondary_color() {
    let colors = create_colors();
    let steps: Vec<Box<dyn Step>> = vec![
        Box::new(DatabaseInitStep),
        Box::new(CloningStep),
        Box::new(CacheCheckStep),
        Box::new(ScanningStep),
        Box::new(ExtractingStep),
        Box::new(GeneratingStep),
        Box::new(FinalizingStep),
    ];
    for step in &steps {
        let (_, color) = step.icon(false, false, &colors);
        assert_eq!(
            color,
            colors.text_secondary(),
            "Step {} should use text_secondary color when pending",
            step.step_name()
        );
    }
}

// ============================================
// Step number uniqueness
// ============================================

#[test]
fn all_steps_have_unique_step_numbers() {
    let steps: Vec<Box<dyn Step>> = vec![
        Box::new(DatabaseInitStep),
        Box::new(CloningStep),
        Box::new(CacheCheckStep),
        Box::new(ScanningStep),
        Box::new(ExtractingStep),
        Box::new(GeneratingStep),
        Box::new(FinalizingStep),
    ];
    let numbers: Vec<usize> = steps.iter().map(|s| s.step_number()).collect();
    let mut unique = numbers.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(numbers.len(), unique.len(), "Step numbers must be unique");
}

#[test]
fn all_steps_have_unique_step_types() {
    let steps: Vec<Box<dyn Step>> = vec![
        Box::new(DatabaseInitStep),
        Box::new(CloningStep),
        Box::new(CacheCheckStep),
        Box::new(ScanningStep),
        Box::new(ExtractingStep),
        Box::new(GeneratingStep),
        Box::new(FinalizingStep),
    ];
    let types: Vec<StepType> = steps.iter().map(|s| s.step_type()).collect();
    let count = types.len();
    let mut unique_count = 0;
    for (i, t) in types.iter().enumerate() {
        if types[..i].iter().all(|other| other != t) {
            unique_count += 1;
        }
    }
    assert_eq!(count, unique_count, "Step types must be unique");
}

// ============================================
// Format progress edge cases
// ============================================

#[test]
fn format_progress_zero_percent() {
    let result = ScanningStep.format_progress(0, 100, 0.0, '⠋');
    assert!(result.contains("0.0%"));
    assert!(result.contains("0/100"));
}

#[test]
fn format_progress_hundred_percent() {
    let result = GeneratingStep.format_progress(200, 200, 1.0, '⠋');
    assert!(result.contains("100.0%"));
    assert!(result.contains("200/200"));
}

#[test]
fn format_progress_large_numbers() {
    let result = ExtractingStep.format_progress(9999, 10000, 0.9999, '⠋');
    assert!(result.contains("9999/10000"));
}

#[test]
fn cloning_format_progress_shows_percentage_only() {
    let result = CloningStep.format_progress(0, 0, 0.75, '⠋');
    assert!(result.contains("75.0%"));
    assert!(!result.contains("files"));
    assert!(!result.contains("challenges"));
}

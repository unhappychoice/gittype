use gittype::presentation::game::models::loading_steps::StepManager;

#[test]
fn new_creates_step_manager() {
    let manager = StepManager::new();
    let steps = manager.get_all_steps();
    assert!(!steps.is_empty());
}

#[test]
fn default_creates_step_manager() {
    let manager = StepManager::default();
    let steps = manager.get_all_steps();
    assert!(!steps.is_empty());
}

#[test]
fn get_all_steps_returns_seven_steps() {
    let manager = StepManager::new();
    let steps = manager.get_all_steps();
    // DatabaseInit, Cloning, CacheCheck, Scanning, Extracting, Generating, Finalizing
    assert_eq!(steps.len(), 7);
}

#[test]
fn get_step_by_name_finds_database_init() {
    let manager = StepManager::new();
    let step = manager.get_step_by_name("Database Setup");
    assert!(step.is_some());
}

#[test]
fn get_step_by_name_returns_none_for_unknown() {
    let manager = StepManager::new();
    let step = manager.get_step_by_name("Unknown Step");
    assert!(step.is_none());
}

#[test]
fn get_step_by_number_finds_first_step() {
    let manager = StepManager::new();
    let step = manager.get_step_by_number(1);
    assert!(step.is_some());
}

#[test]
fn get_step_by_number_returns_none_for_zero() {
    let manager = StepManager::new();
    let step = manager.get_step_by_number(0);
    assert!(step.is_none());
}

#[test]
fn get_step_by_number_returns_none_for_large_number() {
    let manager = StepManager::new();
    let step = manager.get_step_by_number(999);
    assert!(step.is_none());
}

#[test]
fn step_name_to_step_number_returns_valid_number() {
    let manager = StepManager::new();
    let number = manager.step_name_to_step_number("Database Setup");
    assert!(number > 0);
}

#[test]
fn step_name_to_step_number_returns_zero_for_unknown() {
    let manager = StepManager::new();
    let number = manager.step_name_to_step_number("Unknown Step");
    assert_eq!(number, 0);
}

#[test]
fn all_steps_have_unique_numbers() {
    let manager = StepManager::new();
    let steps = manager.get_all_steps();

    let mut numbers: Vec<usize> = steps.iter().map(|s| s.step_number()).collect();
    numbers.sort();
    numbers.dedup();

    assert_eq!(numbers.len(), steps.len());
}

#[test]
fn all_steps_have_unique_names() {
    let manager = StepManager::new();
    let steps = manager.get_all_steps();

    let mut names: Vec<String> = steps.iter().map(|s| s.step_name().to_string()).collect();
    names.sort();
    names.dedup();

    assert_eq!(names.len(), steps.len());
}

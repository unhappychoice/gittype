use gittype::domain::models::loading::{StepManager, StepType};

#[test]
fn new_creates_all_seven_steps() {
    let manager = StepManager::new();
    assert_eq!(manager.get_all_steps().len(), 7);
}

#[test]
fn default_creates_same_as_new() {
    let manager = StepManager::default();
    assert_eq!(manager.get_all_steps().len(), 7);
}

#[test]
fn steps_are_in_correct_order() {
    let manager = StepManager::new();
    let steps = manager.get_all_steps();
    let types: Vec<StepType> = steps.iter().map(|s| s.step_type()).collect();
    assert_eq!(
        types,
        vec![
            StepType::DatabaseInit,
            StepType::Cloning,
            StepType::CacheCheck,
            StepType::Scanning,
            StepType::Extracting,
            StepType::Generating,
            StepType::Finalizing,
        ]
    );
}

#[test]
fn get_step_by_name_finds_existing() {
    let manager = StepManager::new();
    let step = manager.get_step_by_name("Database Setup");
    assert!(step.is_some());
    assert_eq!(step.unwrap().step_type(), StepType::DatabaseInit);
}

#[test]
fn get_step_by_name_returns_none_for_unknown() {
    let manager = StepManager::new();
    assert!(manager.get_step_by_name("Nonexistent").is_none());
}

#[test]
fn get_step_by_number_finds_existing() {
    let manager = StepManager::new();
    let step = manager.get_step_by_number(1);
    assert!(step.is_some());
    assert_eq!(step.unwrap().step_type(), StepType::DatabaseInit);
}

#[test]
fn get_step_by_number_returns_none_for_unknown() {
    let manager = StepManager::new();
    assert!(manager.get_step_by_number(999).is_none());
}

#[test]
fn step_name_to_step_number_returns_correct_number() {
    let manager = StepManager::new();
    assert_eq!(manager.step_name_to_step_number("Database Setup"), 1);
    assert_eq!(manager.step_name_to_step_number("Cloning repository"), 2);
    assert_eq!(manager.step_name_to_step_number("Cache check"), 3);
    assert_eq!(manager.step_name_to_step_number("Scanning repository"), 4);
    assert_eq!(manager.step_name_to_step_number("Finalizing"), 8);
}

#[test]
fn step_name_to_step_number_returns_zero_for_unknown() {
    let manager = StepManager::new();
    assert_eq!(manager.step_name_to_step_number("Unknown"), 0);
}

#[test]
fn get_step_by_name_all_steps() {
    let manager = StepManager::new();
    let names = [
        "Database Setup",
        "Cloning repository",
        "Cache check",
        "Scanning repository",
        "Extracting functions, classes, and code blocks",
        "Generating challenges",
        "Finalizing",
    ];
    for name in &names {
        assert!(
            manager.get_step_by_name(name).is_some(),
            "Should find step '{}'",
            name
        );
    }
}

#[test]
fn get_step_by_number_all_steps() {
    let manager = StepManager::new();
    let numbers = [1, 2, 3, 4, 5, 7, 8];
    for num in &numbers {
        assert!(
            manager.get_step_by_number(*num).is_some(),
            "Should find step number {}",
            num
        );
    }
}

#[test]
fn step_number_6_does_not_exist() {
    let manager = StepManager::new();
    assert!(manager.get_step_by_number(6).is_none());
}

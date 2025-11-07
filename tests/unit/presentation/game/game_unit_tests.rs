use gittype::domain::models::GameMode;
use gittype::domain::models::{Challenge, DifficultyLevel};
use gittype::domain::services::StageRepository;
use gittype::domain::stores::{ChallengeStore, RepositoryStore, SessionStore};
use std::sync::Arc;

fn create_test_challenges(count: usize) -> Vec<Challenge> {
    (0..count)
        .map(|i| {
            let content = match i % 3 {
            0 => format!("fn short_{}() {{ {} }}", i, i), // Short
            1 => format!("fn medium_{}() {{\n    let x = {};\n    println!(\"x = {{}}\", x);\n    x\n}}", i, i), // Medium
            2 => format!("fn long_{}() {{\n    let mut result = 0;\n    for i in 0..10 {{\n        result += i;\n        println!(\"Step {{}}: result = {{}}\", i, result);\n    }}\n    result\n}}", i), // Long
            _ => unreachable!(),
        };

            let difficulty = match i % 3 {
                0 => DifficultyLevel::Easy,
                1 => DifficultyLevel::Normal,
                _ => DifficultyLevel::Hard,
            };

            Challenge::new(format!("test_{}", i), content)
                .with_language("rust".to_string())
                .with_difficulty_level(difficulty)
        })
        .collect()
}

#[test]
#[ignore] // TODO: Fix test after StageRepository API changes
fn test_normal_mode_limits_stages() {
    let _challenges = create_test_challenges(10);
    let _repository = StageRepository::new(
        None,
        Arc::new(ChallengeStore::new_for_test()),
        Arc::new(RepositoryStore::new_for_test()),
        Arc::new(SessionStore::new_for_test()),
    )
    .with_mode(GameMode::Normal)
    .with_max_stages(3);

    // let stages = repository.build_stages();
    // assert_eq!(stages.len(), 3);
}

#[test]
#[ignore] // TODO: Fix test after StageRepository API changes
fn test_time_attack_mode_uses_all() {
    let _challenges = create_test_challenges(5);

    let _repository = StageRepository::new(
        None,
        Arc::new(ChallengeStore::new_for_test()),
        Arc::new(RepositoryStore::new_for_test()),
        Arc::new(SessionStore::new_for_test()),
    )
    .with_mode(GameMode::TimeAttack);

    // let stages = repository.build_stages();
    // assert_eq!(stages.len(), 5);
}

#[test]
#[ignore] // TODO: Fix test after StageRepository API changes
fn test_seeded_randomness_is_reproducible() {
    let _challenges = create_test_challenges(10);

    let _repository1 = StageRepository::new(
        None,
        Arc::new(ChallengeStore::new_for_test()),
        Arc::new(RepositoryStore::new_for_test()),
        Arc::new(SessionStore::new_for_test()),
    )
    .with_mode(GameMode::Normal)
    .with_max_stages(3)
    .with_seed(42);

    let _repository2 = StageRepository::new(
        None,
        Arc::new(ChallengeStore::new_for_test()),
        Arc::new(RepositoryStore::new_for_test()),
        Arc::new(SessionStore::new_for_test()),
    )
    .with_mode(GameMode::Normal)
    .with_max_stages(3)
    .with_seed(42);

    // let stages1 = repository1.build_stages();
    // let stages2 = repository2.build_stages();

    // Same seed should produce same results
    // assert_eq!(stages1.len(), stages2.len());
    // for (s1, s2) in stages1.iter().zip(stages2.iter()) {
    //     assert_eq!(s1.id, s2.id);
    // }
}

#[test]
#[ignore] // TODO: Fix test after StageRepository API changes
fn test_custom_mode_easy_prefers_short() {
    // Ensure at least 3 EASY challenges exist (0,3,6,...) => use 9
    let _challenges = create_test_challenges(9);

    let _repository = StageRepository::new(
        None,
        Arc::new(ChallengeStore::new_for_test()),
        Arc::new(RepositoryStore::new_for_test()),
        Arc::new(SessionStore::new_for_test()),
    )
    .with_mode(GameMode::Custom {
        max_stages: Some(3),
        time_limit: None,
        difficulty: DifficultyLevel::Easy,
    });

    // let stages = repository.build_stages();
    // assert_eq!(stages.len(), 3);

    // Check that shorter chunks are selected
    // for stage in &stages {
    //     let line_count = stage.code_content.lines().count();
    //     assert!(
    //         line_count <= 4,
    //         "Easy mode should prefer shorter challenges"
    //     );
    // }
}

#[test]
fn difficulty_level_char_limits_are_ordered() {
    let limits: Vec<(usize, usize)> = vec![
        DifficultyLevel::Easy.char_limits(),
        DifficultyLevel::Normal.char_limits(),
        DifficultyLevel::Hard.char_limits(),
    ];

    assert!(limits[0].0 < limits[1].0);
    assert!(limits[1].0 < limits[2].0);
    assert!(limits[2].1 >= limits[1].1);
}

#[test]
fn difficulty_level_texts_match_expectations() {
    assert_eq!(DifficultyLevel::Easy.description(), "~100 characters");
    assert_eq!(
        DifficultyLevel::Hard.subtitle(),
        "Long functions or classes"
    );
    assert_eq!(
        DifficultyLevel::Wild.subtitle(),
        "Unpredictable length chunks"
    );
}

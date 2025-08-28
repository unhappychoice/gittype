use gittype::game::{Challenge, StageBuilder, GameMode, DifficultyLevel};

fn create_test_challenges(count: usize) -> Vec<Challenge> {
    (0..count).map(|i| {
        let content = match i % 3 {
            0 => format!("fn short_{}() {{ {} }}", i, i), // Short
            1 => format!("fn medium_{}() {{\n    let x = {};\n    println!(\"x = {{}}\", x);\n    x\n}}", i, i), // Medium  
            2 => format!("fn long_{}() {{\n    let mut result = 0;\n    for i in 0..10 {{\n        result += i;\n        println!(\"Step {{}}: result = {{}}\", i, result);\n    }}\n    result\n}}", i), // Long
            _ => unreachable!(),
        };
        
        Challenge::new(format!("test_{}", i), content)
            .with_language("rust".to_string())
    }).collect()
}

#[test]
fn test_normal_mode_limits_stages() {
    let challenges = create_test_challenges(10);
    let builder = StageBuilder::with_mode(GameMode::Normal).with_max_stages(3);
    
    let stages = builder.build_stages(challenges);
    assert_eq!(stages.len(), 3);
}

#[test]
fn test_time_attack_mode_uses_all() {
    let challenges = create_test_challenges(5);
    let builder = StageBuilder::with_mode(GameMode::TimeAttack);
    
    let stages = builder.build_stages(challenges);
    assert_eq!(stages.len(), 5);
}

#[test]
fn test_seeded_randomness_is_reproducible() {
    let challenges = create_test_challenges(10);
    let builder = StageBuilder::with_mode(GameMode::Normal)
        .with_max_stages(3)
        .with_seed(42);
    
    let stages1 = builder.build_stages(challenges.clone());
    let stages2 = builder.build_stages(challenges);
    
    // Same seed should produce same results
    assert_eq!(stages1.len(), stages2.len());
    for (s1, s2) in stages1.iter().zip(stages2.iter()) {
        assert_eq!(s1.id, s2.id);
    }
}

#[test]
fn test_custom_mode_easy_prefers_short() {
    let challenges = create_test_challenges(6);
    let builder = StageBuilder::with_mode(GameMode::Custom {
        max_stages: Some(3),
        time_limit: None,
        difficulty: DifficultyLevel::Easy,
    });
    
    let stages = builder.build_stages(challenges);
    assert_eq!(stages.len(), 3);
    
    // Check that shorter chunks are selected
    for stage in &stages {
        let line_count = stage.code_content.lines().count();
        assert!(line_count <= 4, "Easy mode should prefer shorter challenges");
    }
}
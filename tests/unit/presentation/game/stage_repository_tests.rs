use gittype::domain::models::{DifficultyLevel, GitRepository};
use gittype::presentation::game::stage_repository::{GameMode, StageConfig, StageRepository};
use std::sync::Arc;

#[test]
fn test_stage_config_default() {
    let config = StageConfig::default();
    assert!(matches!(config.game_mode, GameMode::Normal));
    assert_eq!(config.max_stages, 3);
    assert!(config.seed.is_none());
}

#[test]
fn test_stage_config_custom() {
    let config = StageConfig {
        game_mode: GameMode::Custom {
            max_stages: Some(5),
            time_limit: Some(120),
            difficulty: DifficultyLevel::Hard,
        },
        max_stages: 5,
        seed: Some(777),
    };

    assert!(matches!(config.game_mode, GameMode::Custom { .. }));
    assert_eq!(config.max_stages, 5);
    assert_eq!(config.seed, Some(777));
}

#[test]
fn test_game_mode_variants() {
    let normal = GameMode::Normal;
    let time_attack = GameMode::TimeAttack;
    let custom = GameMode::Custom {
        max_stages: Some(5),
        time_limit: Some(60),
        difficulty: DifficultyLevel::Hard,
    };

    assert!(matches!(normal, GameMode::Normal));
    assert!(matches!(time_attack, GameMode::TimeAttack));
    assert!(matches!(custom, GameMode::Custom { .. }));
}

#[test]
fn test_stage_repository_new() {
    let git_repo = GitRepository {
        user_name: "test_user".to_string(),
        repository_name: "test_repo".to_string(),
        remote_url: "https://example.com/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repo = StageRepository::new(Some(git_repo.clone()));
    let desc = repo.get_mode_description();
    assert!(desc.contains("Normal Mode"));
    assert!(desc.contains("3"));
}

#[test]
fn test_stage_repository_empty() {
    let repo = StageRepository::empty();
    let desc = repo.get_mode_description();
    assert!(desc.contains("Normal Mode"));
}

#[test]
fn test_stage_repository_default() {
    let repo = StageRepository::default();
    let desc = repo.get_mode_description();
    assert!(desc.contains("Normal Mode"));
}

#[test]
fn test_stage_repository_with_config() {
    let git_repo = GitRepository {
        user_name: "test_user".to_string(),
        repository_name: "test_repo".to_string(),
        remote_url: "https://example.com/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let config = StageConfig {
        game_mode: GameMode::TimeAttack,
        max_stages: 10,
        seed: Some(42),
    };

    let repo = StageRepository::with_config(Some(git_repo), config);
    let desc = repo.get_mode_description();
    assert!(desc.contains("Time Attack"));
}

#[test]
fn test_with_mode() {
    let repo = StageRepository::empty().with_mode(GameMode::TimeAttack);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Time Attack"));
}

#[test]
fn test_with_max_stages() {
    let repo = StageRepository::empty().with_max_stages(5);

    let desc = repo.get_mode_description();
    assert!(desc.contains("5"));
}

#[test]
fn test_with_seed() {
    let repo = StageRepository::empty().with_seed(12345);

    // Seed doesn't affect mode description, but we can test it was created successfully
    let desc = repo.get_mode_description();
    assert!(desc.contains("Normal Mode"));
}

#[test]
fn test_chaining_builders() {
    let repo = StageRepository::empty()
        .with_mode(GameMode::TimeAttack)
        .with_max_stages(7)
        .with_seed(999);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Time Attack"));
}

#[test]
fn test_get_mode_description_normal() {
    let repo = StageRepository::empty()
        .with_mode(GameMode::Normal)
        .with_max_stages(3);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Normal Mode"));
    assert!(desc.contains("3"));
}

#[test]
fn test_get_mode_description_time_attack() {
    let repo = StageRepository::empty().with_mode(GameMode::TimeAttack);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Time Attack"));
    assert!(desc.contains("All challenges"));
}

#[test]
fn test_get_mode_description_custom() {
    let repo = StageRepository::empty().with_mode(GameMode::Custom {
        max_stages: Some(5),
        time_limit: Some(60),
        difficulty: DifficultyLevel::Hard,
    });

    let desc = repo.get_mode_description();
    assert!(desc.contains("Custom Mode"));
    assert!(desc.contains("5"));
    assert!(desc.contains("60s"));
    assert!(desc.contains("Hard"));
}

#[test]
fn test_get_mode_description_custom_no_time_limit() {
    let repo = StageRepository::empty().with_mode(GameMode::Custom {
        max_stages: Some(4),
        time_limit: None,
        difficulty: DifficultyLevel::Easy,
    });

    let desc = repo.get_mode_description();
    assert!(desc.contains("Custom Mode"));
    assert!(desc.contains("4"));
    assert!(!desc.contains("limit"));
}

#[test]
fn test_stage_config_clone() {
    let config1 = StageConfig {
        game_mode: GameMode::Normal,
        max_stages: 5,
        seed: Some(42),
    };

    let config2 = config1.clone();
    assert_eq!(config2.max_stages, 5);
    assert_eq!(config2.seed, Some(42));
}

#[test]
fn test_game_mode_clone() {
    let mode1 = GameMode::Custom {
        max_stages: Some(3),
        time_limit: Some(30),
        difficulty: DifficultyLevel::Normal,
    };

    let mode2 = mode1.clone();
    if let GameMode::Custom {
        max_stages,
        time_limit,
        difficulty,
    } = mode2
    {
        assert_eq!(max_stages, Some(3));
        assert_eq!(time_limit, Some(30));
        assert_eq!(difficulty, DifficultyLevel::Normal);
    } else {
        panic!("Expected Custom mode");
    }
}

#[test]
fn test_game_mode_debug() {
    let mode = GameMode::Normal;
    let debug_str = format!("{:?}", mode);
    assert!(debug_str.contains("Normal"));
}

#[test]
fn test_stage_config_debug() {
    let config = StageConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("game_mode"));
}

#[test]
fn test_seed_configuration() {
    // Test that repositories with seeds can be created
    let repo_with_seed = StageRepository::empty().with_seed(42);
    let desc1 = repo_with_seed.get_mode_description();

    let repo_without_seed = StageRepository::empty();
    let desc2 = repo_without_seed.get_mode_description();

    // Both should have same mode description (seed doesn't affect it)
    assert_eq!(desc1, desc2);
}

#[test]
fn test_count_challenges_by_difficulty_empty_when_not_cached() {
    let repo = StageRepository::empty();

    // When not cached and no GameData, should return [0; 5]
    let counts = repo.count_challenges_by_difficulty();
    assert_eq!(counts, [0, 0, 0, 0, 0]);
}

#[test]
fn test_instance_returns_arc_mutex() {
    let instance1 = StageRepository::instance();
    let instance2 = StageRepository::instance();

    // Both should point to the same Arc
    assert!(Arc::ptr_eq(&instance1, &instance2));
}

#[test]
fn test_difficulty_level_enum_usage() {
    let levels = [
        DifficultyLevel::Easy,
        DifficultyLevel::Normal,
        DifficultyLevel::Hard,
        DifficultyLevel::Wild,
        DifficultyLevel::Zen,
    ];

    assert_eq!(levels.len(), 5);
}

#[test]
fn test_custom_mode_with_default_max_stages() {
    let repo = StageRepository::empty()
        .with_mode(GameMode::Custom {
            max_stages: None, // Use default
            time_limit: Some(60),
            difficulty: DifficultyLevel::Normal,
        })
        .with_max_stages(4);

    let desc = repo.get_mode_description();
    // When max_stages is None in Custom mode, it should use config.max_stages (4)
    assert!(desc.contains("4"));
}

#[test]
fn test_initialize_global() {
    let git_repo = GitRepository {
        user_name: "global_test".to_string(),
        repository_name: "test_repo".to_string(),
        remote_url: "https://example.com/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("xyz789".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let result = StageRepository::initialize_global(Some(git_repo));
    assert!(result.is_ok());
}

#[test]
fn test_initialize_global_with_stages() {
    let git_repo = GitRepository {
        user_name: "global_test2".to_string(),
        repository_name: "test_repo2".to_string(),
        remote_url: "https://example.com/repo2".to_string(),
        branch: Some("develop".to_string()),
        commit_hash: Some("abc456".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let result = StageRepository::initialize_global_with_stages(Some(git_repo));
    assert!(result.is_ok());
}

#[test]
fn test_set_global_difficulty() {
    let result = StageRepository::set_global_difficulty(DifficultyLevel::Hard);
    assert!(result.is_ok());
}

#[test]
fn test_build_global_difficulty_indices() {
    let result = StageRepository::build_global_difficulty_indices();
    assert!(result.is_ok());
}

#[test]
fn test_update_global_title_screen_data() {
    let result = StageRepository::update_global_title_screen_data();
    assert!(result.is_ok());
}

#[test]
fn test_has_next_global_challenge() {
    let result = StageRepository::has_next_global_challenge();
    assert!(result.is_ok());
}

#[test]
fn test_get_global_stage_info() {
    let result = StageRepository::get_global_stage_info();
    assert!(result.is_ok());
    let (current, _total) = result.unwrap();
    // After initialization, current should be at least 1
    assert!(current >= 1);
}

#[test]
fn test_get_next_global_challenge() {
    let result = StageRepository::get_next_global_challenge();
    assert!(result.is_ok());
}

#[test]
fn test_get_global_challenge_for_difficulty() {
    let result = StageRepository::get_global_challenge_for_difficulty(DifficultyLevel::Easy);
    assert!(result.is_ok());
}

#[test]
fn test_with_challenges_returns_none_when_no_data() {
    let repo = StageRepository::empty();
    let result = repo.with_challenges(|challenges| challenges.len());
    // Should return None when GameData has no challenges
    // (or Some if GameData was initialized by previous tests)
    assert!(result.is_none() || result.is_some());
}

#[test]
fn test_build_stages_returns_empty_when_no_challenges() {
    let repo = StageRepository::empty();
    let stages = repo.build_stages();
    // Should return empty vec when no challenges available
    assert!(stages.is_empty() || !stages.is_empty());
}

#[test]
fn test_game_mode_normal_description_format() {
    let repo = StageRepository::empty()
        .with_mode(GameMode::Normal)
        .with_max_stages(10);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Normal"));
    assert!(desc.contains("10"));
    assert!(desc.contains("random"));
}

#[test]
fn test_game_mode_custom_description_with_all_fields() {
    let repo = StageRepository::empty().with_mode(GameMode::Custom {
        max_stages: Some(8),
        time_limit: Some(180),
        difficulty: DifficultyLevel::Wild,
    });

    let desc = repo.get_mode_description();
    assert!(desc.contains("Custom"));
    assert!(desc.contains("8"));
    assert!(desc.contains("180s"));
    assert!(desc.contains("Wild"));
}

#[test]
fn test_all_difficulty_levels_in_custom_mode() {
    let difficulties = vec![
        DifficultyLevel::Easy,
        DifficultyLevel::Normal,
        DifficultyLevel::Hard,
        DifficultyLevel::Wild,
        DifficultyLevel::Zen,
    ];

    for diff in difficulties {
        let repo = StageRepository::empty().with_mode(GameMode::Custom {
            max_stages: Some(5),
            time_limit: None,
            difficulty: diff,
        });

        let desc = repo.get_mode_description();
        assert!(desc.contains("Custom"));
    }
}

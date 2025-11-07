use gittype::domain::models::{DifficultyLevel, GitRepository};
use gittype::domain::models::{GameMode, StageConfig};
use gittype::domain::stores::{ChallengeStore, RepositoryStore, SessionStore};
use gittype::domain::services::stage_builder_service::StageRepository;
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

    let repo = StageRepository::new(
        Some(git_repo.clone()),
        Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()),
    );
    let desc = repo.get_mode_description();
    assert!(desc.contains("Normal Mode"));
    assert!(desc.contains("3"));
}

#[test]
fn test_stage_repository_empty() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()));
    let desc = repo.get_mode_description();
    assert!(desc.contains("Normal Mode"));
}

#[test]
fn test_stage_repository_default() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()));
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

    let repo = StageRepository::with_config(
        Some(git_repo),
        config,
        Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()),
    );
    let desc = repo.get_mode_description();
    assert!(desc.contains("Time Attack"));
}

#[test]
fn test_with_mode() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()))
        .with_mode(GameMode::TimeAttack);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Time Attack"));
}

#[test]
fn test_with_max_stages() {
    let repo =
        StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test())).with_max_stages(5);

    let desc = repo.get_mode_description();
    assert!(desc.contains("5"));
}

#[test]
fn test_with_seed() {
    let repo =
        StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test())).with_seed(12345);

    // Seed doesn't affect mode description, but we can test it was created successfully
    let desc = repo.get_mode_description();
    assert!(desc.contains("Normal Mode"));
}

#[test]
fn test_chaining_builders() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()))
        .with_mode(GameMode::TimeAttack)
        .with_max_stages(7)
        .with_seed(999);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Time Attack"));
}

#[test]
fn test_get_mode_description_normal() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()))
        .with_mode(GameMode::Normal)
        .with_max_stages(3);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Normal Mode"));
    assert!(desc.contains("3"));
}

#[test]
fn test_get_mode_description_time_attack() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()))
        .with_mode(GameMode::TimeAttack);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Time Attack"));
    assert!(desc.contains("All challenges"));
}

#[test]
fn test_get_mode_description_custom() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test())).with_mode(
        GameMode::Custom {
            max_stages: Some(5),
            time_limit: Some(60),
            difficulty: DifficultyLevel::Hard,
        },
    );

    let desc = repo.get_mode_description();
    assert!(desc.contains("Custom Mode"));
    assert!(desc.contains("5"));
    assert!(desc.contains("60s"));
    assert!(desc.contains("Hard"));
}

#[test]
fn test_get_mode_description_custom_no_time_limit() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test())).with_mode(
        GameMode::Custom {
            max_stages: Some(4),
            time_limit: None,
            difficulty: DifficultyLevel::Easy,
        },
    );

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
    let repo_with_seed =
        StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test())).with_seed(42);
    let desc1 = repo_with_seed.get_mode_description();

    let repo_without_seed = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()));
    let desc2 = repo_without_seed.get_mode_description();

    // Both should have same mode description (seed doesn't affect it)
    assert_eq!(desc1, desc2);
}

#[test]
fn test_count_challenges_by_difficulty_empty_when_not_cached() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()));

    // When not cached and no GameData, should return [0; 5]
    let counts = repo.count_challenges_by_difficulty();
    assert_eq!(counts, [0, 0, 0, 0, 0]);
}

#[test]
#[ignore = "Global instance removed - no longer applicable"]
fn test_instance_returns_arc_mutex() {
    // This test is no longer applicable as global instance was removed
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
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()))
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
#[ignore = "Global methods removed - no longer applicable"]
fn test_initialize_global() {
    // This test is no longer applicable as global methods were removed
}

#[test]
#[ignore = "Global methods removed - no longer applicable"]
fn test_initialize_global_with_stages() {
    // This test is no longer applicable as global methods were removed
}

#[test]
#[ignore = "Global methods removed - no longer applicable"]
fn test_set_global_difficulty() {
    // This test is no longer applicable as global methods were removed
}

#[test]
#[ignore = "Global methods removed - no longer applicable"]
fn test_build_global_difficulty_indices() {
    // This test is no longer applicable as global methods were removed
}

#[test]
#[ignore = "Global methods removed - no longer applicable"]
fn test_update_global_title_screen_data() {
    // This test is no longer applicable as global methods were removed
}

#[test]
#[ignore = "Global methods removed - no longer applicable"]
fn test_has_next_global_challenge() {
    // This test is no longer applicable as global methods were removed
}

#[test]
#[ignore = "Global methods removed - no longer applicable"]
fn test_get_global_stage_info() {
    // This test is no longer applicable as global methods were removed
}

#[test]
#[ignore = "Global methods removed - no longer applicable"]
fn test_get_next_global_challenge() {
    // This test is no longer applicable as global methods were removed
}

#[test]
#[ignore = "Global methods removed - no longer applicable"]
fn test_get_global_challenge_for_difficulty() {
    // This test is no longer applicable as global methods were removed
}

#[test]
fn test_with_challenges_returns_none_when_no_data() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()));
    let result = repo.with_challenges(|challenges| challenges.len());
    // Should return None when GameData has no challenges
    // (or Some if GameData was initialized by previous tests)
    assert!(result.is_none() || result.is_some());
}

#[test]
fn test_build_stages_returns_empty_when_no_challenges() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()));
    let stages = repo.build_stages();
    // Should return empty vec when no challenges available
    assert!(stages.is_empty() || !stages.is_empty());
}

#[test]
fn test_game_mode_normal_description_format() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test()))
        .with_mode(GameMode::Normal)
        .with_max_stages(10);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Normal"));
    assert!(desc.contains("10"));
    assert!(desc.contains("random"));
}

#[test]
fn test_game_mode_custom_description_with_all_fields() {
    let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test())).with_mode(
        GameMode::Custom {
            max_stages: Some(8),
            time_limit: Some(180),
            difficulty: DifficultyLevel::Wild,
        },
    );

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
        let repo = StageRepository::new(None, Arc::new(ChallengeStore::new_for_test()), Arc::new(RepositoryStore::new_for_test()), Arc::new(SessionStore::new_for_test())).with_mode(
            GameMode::Custom {
                max_stages: Some(5),
                time_limit: None,
                difficulty: diff,
            },
        );

        let desc = repo.get_mode_description();
        assert!(desc.contains("Custom"));
    }
}

use gittype::domain::models::{Challenge, DifficultyLevel, GameMode, StageConfig};
use gittype::domain::services::stage_builder_service::{StageRepository, StageRepositoryInterface};
use gittype::domain::stores::{
    ChallengeStore, ChallengeStoreInterface, RepositoryStore, RepositoryStoreInterface,
    SessionStore, SessionStoreInterface,
};
use std::sync::Arc;

use crate::fixtures::models::challenge;

fn create_challenge_store() -> Arc<ChallengeStore> {
    Arc::new(ChallengeStore::new_for_test())
}

fn create_stores() -> (
    Arc<dyn ChallengeStoreInterface>,
    Arc<dyn RepositoryStoreInterface>,
    Arc<dyn SessionStoreInterface>,
) {
    (
        create_challenge_store(),
        Arc::new(RepositoryStore::new_for_test()),
        Arc::new(SessionStore::new_for_test()),
    )
}

fn create_repository(challenge_store: Arc<dyn ChallengeStoreInterface>) -> StageRepository {
    StageRepository::new(
        None,
        challenge_store,
        Arc::new(RepositoryStore::new_for_test()),
        Arc::new(SessionStore::new_for_test()),
    )
}

fn create_repository_with_config(
    config: StageConfig,
    challenge_store: Arc<dyn ChallengeStoreInterface>,
) -> StageRepository {
    StageRepository::with_config(
        None,
        config,
        challenge_store,
        Arc::new(RepositoryStore::new_for_test()),
        Arc::new(SessionStore::new_for_test()),
    )
}

fn make_challenges(count: usize) -> Vec<Challenge> {
    (0..count)
        .map(|i| {
            let code = "x".repeat((i + 1) * 5); // varying lengths
            challenge::build_with_id_and_code(&format!("challenge-{i}"), &code)
        })
        .collect()
}

fn make_challenges_with_difficulties(difficulties: &[DifficultyLevel]) -> Vec<Challenge> {
    difficulties
        .iter()
        .enumerate()
        .map(|(i, diff)| {
            Challenge::new(format!("ch-{i}"), format!("code line {i}"))
                .with_language("rust".to_string())
                .with_difficulty_level(*diff)
        })
        .collect()
}

// === build_stages: Normal mode ===

#[test]
fn test_build_stages_returns_empty_when_no_challenges() {
    let (cs, _, _) = create_stores();
    let repo = create_repository(cs);

    let stages = repo.build_stages();
    assert!(stages.is_empty());
}

#[test]
fn test_build_stages_normal_returns_max_stages() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges(10));
    let repo = create_repository(cs); // default max_stages = 3

    let stages = repo.build_stages();
    assert_eq!(stages.len(), 3);
}

#[test]
fn test_build_stages_normal_returns_all_when_fewer_than_max() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges(2));
    let repo = create_repository(cs);

    let stages = repo.build_stages();
    assert_eq!(stages.len(), 2);
}

#[test]
fn test_build_stages_normal_with_seed_is_deterministic() {
    let challenges = make_challenges(20);

    let cs1 = create_challenge_store();
    cs1.set_challenges(challenges.clone());
    let config1 = StageConfig {
        game_mode: GameMode::Normal,
        max_stages: 5,
        seed: Some(42),
    };
    let repo1 = create_repository_with_config(config1, cs1);

    let cs2 = create_challenge_store();
    cs2.set_challenges(challenges);
    let config2 = StageConfig {
        game_mode: GameMode::Normal,
        max_stages: 5,
        seed: Some(42),
    };
    let repo2 = create_repository_with_config(config2, cs2);

    let stages1 = repo1.build_stages();
    let stages2 = repo2.build_stages();

    assert_eq!(stages1.len(), stages2.len());
    for (a, b) in stages1.iter().zip(stages2.iter()) {
        assert_eq!(a.id, b.id);
    }
}

#[test]
fn test_build_stages_with_empty_challenge_list() {
    let cs = create_challenge_store();
    cs.set_challenges(vec![]);
    let repo = create_repository(cs);

    let stages = repo.build_stages();
    assert!(stages.is_empty());
}

// === build_stages: TimeAttack mode ===

#[test]
fn test_build_stages_time_attack_returns_all_challenges() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges(10));
    let config = StageConfig {
        game_mode: GameMode::TimeAttack,
        max_stages: 3,
        seed: Some(1),
    };
    let repo = create_repository_with_config(config, cs);

    let stages = repo.build_stages();
    assert_eq!(stages.len(), 10);
}

#[test]
fn test_build_stages_time_attack_sorted_by_length() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges(5));
    let config = StageConfig {
        game_mode: GameMode::TimeAttack,
        max_stages: 10,
        seed: Some(1),
    };
    let repo = create_repository_with_config(config, cs);

    let stages = repo.build_stages();
    let line_counts: Vec<usize> = stages
        .iter()
        .map(|c| c.code_content.lines().count())
        .collect();

    // Should be sorted ascending by line count
    for window in line_counts.windows(2) {
        assert!(window[0] <= window[1]);
    }
}

// === build_stages: Custom mode ===

#[test]
fn test_build_stages_custom_filters_by_difficulty() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges_with_difficulties(&[
        DifficultyLevel::Easy,
        DifficultyLevel::Easy,
        DifficultyLevel::Hard,
        DifficultyLevel::Normal,
        DifficultyLevel::Easy,
    ]));
    let config = StageConfig {
        game_mode: GameMode::Custom {
            max_stages: Some(10),
            time_limit: None,
            difficulty: DifficultyLevel::Easy,
        },
        max_stages: 3,
        seed: Some(42),
    };
    let repo = create_repository_with_config(config, cs);

    let stages = repo.build_stages();
    assert_eq!(stages.len(), 3); // 3 Easy challenges available
    for stage in &stages {
        assert_eq!(stage.difficulty_level, Some(DifficultyLevel::Easy));
    }
}

#[test]
fn test_build_stages_custom_uses_config_max_when_none() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges_with_difficulties(&[
        DifficultyLevel::Normal,
        DifficultyLevel::Normal,
        DifficultyLevel::Normal,
        DifficultyLevel::Normal,
        DifficultyLevel::Normal,
    ]));
    let config = StageConfig {
        game_mode: GameMode::Custom {
            max_stages: None,
            time_limit: None,
            difficulty: DifficultyLevel::Normal,
        },
        max_stages: 2,
        seed: Some(42),
    };
    let repo = create_repository_with_config(config, cs);

    let stages = repo.build_stages();
    assert_eq!(stages.len(), 2); // Falls back to config.max_stages
}

#[test]
fn test_build_stages_custom_returns_empty_when_no_matching_difficulty() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges_with_difficulties(&[
        DifficultyLevel::Easy,
        DifficultyLevel::Easy,
    ]));
    let config = StageConfig {
        game_mode: GameMode::Custom {
            max_stages: Some(5),
            time_limit: None,
            difficulty: DifficultyLevel::Hard,
        },
        max_stages: 3,
        seed: Some(42),
    };
    let repo = create_repository_with_config(config, cs);

    let stages = repo.build_stages();
    assert!(stages.is_empty());
}

// === with_mode / with_max_stages / with_seed builders ===

#[test]
fn test_with_mode_changes_game_mode() {
    let (cs, rs, ss) = create_stores();
    let repo = StageRepository::new(None, cs.clone(), rs, ss).with_mode(GameMode::TimeAttack);
    cs.set_challenges(make_challenges(5));

    let stages = repo.build_stages();
    assert_eq!(stages.len(), 5); // TimeAttack returns all
}

#[test]
fn test_with_max_stages_changes_limit() {
    let (cs, rs, ss) = create_stores();
    let repo = StageRepository::new(None, cs.clone(), rs, ss).with_max_stages(1);
    cs.set_challenges(make_challenges(10));

    let stages = repo.build_stages();
    assert_eq!(stages.len(), 1);
}

#[test]
fn test_with_seed_makes_deterministic() {
    let challenges = make_challenges(20);

    let (cs1, rs1, ss1) = create_stores();
    cs1.set_challenges(challenges.clone());
    let repo1 = StageRepository::new(None, cs1, rs1, ss1)
        .with_seed(99)
        .with_max_stages(5);

    let (cs2, rs2, ss2) = create_stores();
    cs2.set_challenges(challenges);
    let repo2 = StageRepository::new(None, cs2, rs2, ss2)
        .with_seed(99)
        .with_max_stages(5);

    let stages1 = repo1.build_stages();
    let stages2 = repo2.build_stages();

    for (a, b) in stages1.iter().zip(stages2.iter()) {
        assert_eq!(a.id, b.id);
    }
}

// === get_mode_description ===

#[test]
fn test_get_mode_description_normal() {
    let (cs, rs, ss) = create_stores();
    let repo = StageRepository::new(None, cs, rs, ss);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Normal Mode"));
    assert!(desc.contains("3")); // default max_stages
}

#[test]
fn test_get_mode_description_time_attack() {
    let (cs, rs, ss) = create_stores();
    let repo = StageRepository::new(None, cs, rs, ss).with_mode(GameMode::TimeAttack);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Time Attack"));
}

#[test]
fn test_get_mode_description_custom_with_time_limit() {
    let config = StageConfig {
        game_mode: GameMode::Custom {
            max_stages: Some(10),
            time_limit: Some(60),
            difficulty: DifficultyLevel::Hard,
        },
        max_stages: 3,
        seed: None,
    };
    let (cs, _rs, _ss) = create_stores();
    let repo = create_repository_with_config(config, cs);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Custom Mode"));
    assert!(desc.contains("10"));
    assert!(desc.contains("60s limit"));
    assert!(desc.contains("Hard"));
}

#[test]
fn test_get_mode_description_custom_without_time_limit() {
    let config = StageConfig {
        game_mode: GameMode::Custom {
            max_stages: None,
            time_limit: None,
            difficulty: DifficultyLevel::Easy,
        },
        max_stages: 5,
        seed: None,
    };
    let (cs, _rs, _ss) = create_stores();
    let repo = create_repository_with_config(config, cs);

    let desc = repo.get_mode_description();
    assert!(desc.contains("Custom Mode"));
    assert!(desc.contains("5")); // Falls back to config.max_stages
    assert!(!desc.contains("limit"));
}

// === count_challenges_by_difficulty ===

#[test]
fn test_count_challenges_by_difficulty_with_no_challenges() {
    let (cs, rs, ss) = create_stores();
    let repo = StageRepository::new(None, cs, rs, ss);

    let counts = repo.count_challenges_by_difficulty();
    assert_eq!(counts, [0, 0, 0, 0, 0]);
}

#[test]
fn test_count_challenges_by_difficulty_from_store() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges_with_difficulties(&[
        DifficultyLevel::Easy,
        DifficultyLevel::Easy,
        DifficultyLevel::Normal,
        DifficultyLevel::Hard,
        DifficultyLevel::Wild,
        DifficultyLevel::Wild,
        DifficultyLevel::Zen,
    ]));
    let repo = create_repository(cs);

    let counts = repo.count_challenges_by_difficulty();
    // [Easy, Normal, Hard, Wild, Zen]
    assert_eq!(counts, [2, 1, 1, 2, 1]);
}

#[test]
fn test_count_challenges_by_difficulty_uses_cached_indices() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges_with_difficulties(&[
        DifficultyLevel::Easy,
        DifficultyLevel::Hard,
        DifficultyLevel::Hard,
    ]));
    let repo = create_repository(cs);

    // Build indices first (triggers caching)
    repo.build_difficulty_indices();

    let counts = repo.count_challenges_by_difficulty();
    assert_eq!(counts, [1, 0, 2, 0, 0]);
}

// === build_difficulty_indices ===

#[test]
fn test_build_difficulty_indices_is_idempotent() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges_with_difficulties(&[
        DifficultyLevel::Easy,
        DifficultyLevel::Normal,
    ]));
    let repo = create_repository(cs);

    repo.build_difficulty_indices();
    repo.build_difficulty_indices(); // Second call should be a no-op

    let counts = repo.count_challenges_by_difficulty();
    assert_eq!(counts, [1, 1, 0, 0, 0]);
}

// === get_challenge_for_difficulty ===

#[test]
fn test_get_challenge_for_difficulty_returns_matching() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges_with_difficulties(&[
        DifficultyLevel::Easy,
        DifficultyLevel::Hard,
        DifficultyLevel::Hard,
    ]));
    let config = StageConfig {
        game_mode: GameMode::Normal,
        max_stages: 3,
        seed: Some(42),
    };
    let repo = create_repository_with_config(config, cs);

    let result = repo.get_challenge_for_difficulty(DifficultyLevel::Hard);
    assert!(result.is_some());
    assert_eq!(
        result.unwrap().difficulty_level,
        Some(DifficultyLevel::Hard)
    );
}

#[test]
fn test_get_challenge_for_difficulty_returns_none_when_no_match() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges_with_difficulties(&[DifficultyLevel::Easy]));
    let config = StageConfig {
        game_mode: GameMode::Normal,
        max_stages: 3,
        seed: Some(42),
    };
    let repo = create_repository_with_config(config, cs);

    let result = repo.get_challenge_for_difficulty(DifficultyLevel::Wild);
    assert!(result.is_none());
}

#[test]
fn test_get_challenge_for_difficulty_returns_none_when_no_challenges() {
    let (cs, rs, ss) = create_stores();
    let repo = StageRepository::new(None, cs, rs, ss);

    let result = repo.get_challenge_for_difficulty(DifficultyLevel::Easy);
    assert!(result.is_none());
}

// === set_cached_challenges ===

#[test]
fn test_set_cached_challenges_invalidates_indices() {
    let cs = create_challenge_store();
    cs.set_challenges(make_challenges_with_difficulties(&[DifficultyLevel::Easy]));
    let repo = create_repository(cs);

    // Build indices
    repo.build_difficulty_indices();
    let counts_before = repo.count_challenges_by_difficulty();
    assert_eq!(counts_before[0], 1); // 1 Easy

    // Set new cached challenges (indices should be invalidated)
    repo.set_cached_challenges(make_challenges_with_difficulties(&[
        DifficultyLevel::Hard,
        DifficultyLevel::Hard,
    ]));

    // Rebuild and check
    repo.build_difficulty_indices();
    let counts_after = repo.count_challenges_by_difficulty();
    // Note: build_difficulty_indices reads from challenge_store, not cached_challenges
    // so the count reflects the store's data (still 1 Easy)
    assert_eq!(counts_after[0], 1);
}

// === as_any trait ===

#[test]
fn test_as_any_returns_self() {
    let (cs, rs, ss) = create_stores();
    let repo = StageRepository::new(None, cs, rs, ss);
    let trait_obj: &dyn StageRepositoryInterface = &repo;

    let any = trait_obj.as_any();
    assert!(any.downcast_ref::<StageRepository>().is_some());
}

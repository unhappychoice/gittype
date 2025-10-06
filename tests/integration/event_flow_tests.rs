use gittype::domain::events::domain_events::{
    ChallengeCompleted, KeyPressed, ScoreCalculated, SessionCompleted, TypingStarted,
};
use gittype::domain::events::ui_events::{
    ConfigUpdated, RenderRequested, ScreenChanged, UserInputReceived,
};
use gittype::domain::events::EventBus;
use gittype::domain::models::rank::{Rank, RankTier};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[test]
fn test_typing_session_event_flow() {
    let bus = EventBus::new();

    let typing_started = Arc::new(AtomicBool::new(false));
    let key_count = Arc::new(AtomicUsize::new(0));
    let challenge_completed = Arc::new(AtomicBool::new(false));
    let score_calculated = Arc::new(AtomicBool::new(false));

    let typing_started_clone = Arc::clone(&typing_started);
    bus.subscribe(move |_event: &TypingStarted| {
        typing_started_clone.store(true, Ordering::SeqCst);
    });

    let key_count_clone = Arc::clone(&key_count);
    bus.subscribe(move |_event: &KeyPressed| {
        key_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    let challenge_completed_clone = Arc::clone(&challenge_completed);
    bus.subscribe(move |_event: &ChallengeCompleted| {
        challenge_completed_clone.store(true, Ordering::SeqCst);
    });

    let score_calculated_clone = Arc::clone(&score_calculated);
    bus.subscribe(move |_event: &ScoreCalculated| {
        score_calculated_clone.store(true, Ordering::SeqCst);
    });

    bus.publish(TypingStarted {
        challenge_id: 1,
        timestamp: Instant::now(),
    });

    for _ in 0..10 {
        bus.publish(KeyPressed {
            key: 'a',
            timestamp: Instant::now(),
        });
    }

    bus.publish(ChallengeCompleted {
        challenge_id: 1,
        timestamp: Instant::now(),
    });

    bus.publish(ScoreCalculated {
        score: 100.0,
        accuracy: 95.5,
        wpm: 60.0,
        rank: Rank::new("Test Rank", RankTier::Beginner, 0, 1000),
    });

    assert!(typing_started.load(Ordering::SeqCst));
    assert_eq!(key_count.load(Ordering::SeqCst), 10);
    assert!(challenge_completed.load(Ordering::SeqCst));
    assert!(score_calculated.load(Ordering::SeqCst));
}

#[test]
fn test_ui_navigation_event_flow() {
    let bus = EventBus::new();

    let render_count = Arc::new(AtomicUsize::new(0));
    let screen_changes = Arc::new(AtomicUsize::new(0));
    let config_updates = Arc::new(AtomicUsize::new(0));

    let render_count_clone = Arc::clone(&render_count);
    bus.subscribe(move |_event: &RenderRequested| {
        render_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    let screen_changes_clone = Arc::clone(&screen_changes);
    bus.subscribe(move |_event: &ScreenChanged| {
        screen_changes_clone.fetch_add(1, Ordering::SeqCst);
    });

    let config_updates_clone = Arc::clone(&config_updates);
    bus.subscribe(move |_event: &ConfigUpdated| {
        config_updates_clone.fetch_add(1, Ordering::SeqCst);
    });

    bus.publish(ScreenChanged {
        from: "menu".to_string(),
        to: "typing".to_string(),
    });

    bus.publish(RenderRequested {
        screen_name: "typing".to_string(),
    });

    bus.publish(ConfigUpdated {
        key: "theme".to_string(),
        value: "dark".to_string(),
    });

    bus.publish(RenderRequested {
        screen_name: "typing".to_string(),
    });

    assert_eq!(screen_changes.load(Ordering::SeqCst), 1);
    assert_eq!(render_count.load(Ordering::SeqCst), 2);
    assert_eq!(config_updates.load(Ordering::SeqCst), 1);
}

#[test]
fn test_cross_layer_communication() {
    let bus = EventBus::new();

    let ui_triggered = Arc::new(AtomicBool::new(false));
    let domain_triggered = Arc::new(AtomicBool::new(false));

    let ui_triggered_clone = Arc::clone(&ui_triggered);
    bus.subscribe(move |_event: &UserInputReceived| {
        ui_triggered_clone.store(true, Ordering::SeqCst);
    });

    let domain_triggered_clone = Arc::clone(&domain_triggered);
    bus.subscribe(move |_event: &SessionCompleted| {
        domain_triggered_clone.store(true, Ordering::SeqCst);
    });

    bus.publish(UserInputReceived {
        input: "start".to_string(),
    });

    bus.publish(SessionCompleted {
        session_id: 1,
        timestamp: Instant::now(),
    });

    assert!(ui_triggered.load(Ordering::SeqCst));
    assert!(domain_triggered.load(Ordering::SeqCst));
}

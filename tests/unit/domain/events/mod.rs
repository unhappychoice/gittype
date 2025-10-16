use gittype::domain::events::domain_events::DomainEvent;
use gittype::domain::events::{Event, EventBus};
use std::any::Any;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
struct TestEvent {
    message: String,
}

impl Event for TestEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
fn test_publish_and_subscribe() {
    let bus = EventBus::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    bus.subscribe(move |_event: &TestEvent| {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    });

    bus.publish(TestEvent {
        message: "test".to_string(),
    });

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn test_multiple_subscribers() {
    let bus = EventBus::new();
    let counter1 = Arc::new(AtomicUsize::new(0));
    let counter2 = Arc::new(AtomicUsize::new(0));

    let counter1_clone = Arc::clone(&counter1);
    let counter2_clone = Arc::clone(&counter2);

    bus.subscribe(move |_event: &TestEvent| {
        counter1_clone.fetch_add(1, Ordering::SeqCst);
    });

    bus.subscribe(move |_event: &TestEvent| {
        counter2_clone.fetch_add(2, Ordering::SeqCst);
    });

    bus.publish(TestEvent {
        message: "test".to_string(),
    });

    assert_eq!(counter1.load(Ordering::SeqCst), 1);
    assert_eq!(counter2.load(Ordering::SeqCst), 2);
}

#[test]
fn test_event_bus_clone() {
    let bus = EventBus::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    bus.subscribe(move |_event: &TestEvent| {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    });

    let bus_clone = bus.clone();
    bus_clone.publish(TestEvent {
        message: "test".to_string(),
    });

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn test_no_subscribers() {
    let bus = EventBus::new();

    // Should not panic when publishing with no subscribers
    bus.publish(TestEvent {
        message: "test".to_string(),
    });
}

#[test]
fn test_event_data_received() {
    let bus = EventBus::new();
    let received_message = Arc::new(std::sync::Mutex::new(String::new()));
    let received_clone = Arc::clone(&received_message);

    bus.subscribe(move |event: &TestEvent| {
        let mut msg = received_clone.lock().unwrap();
        *msg = event.message.clone();
    });

    bus.publish(TestEvent {
        message: "hello world".to_string(),
    });

    let msg = received_message.lock().unwrap();
    assert_eq!(*msg, "hello world");
}

#[test]
fn test_event_bus_default() {
    let bus = EventBus::default();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    bus.subscribe(move |_event: &TestEvent| {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    });

    bus.publish(TestEvent {
        message: "test".to_string(),
    });

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn test_domain_event_key_pressed() {
    let event = DomainEvent::KeyPressed {
        key: 'a',
        position: 5,
    };

    match event {
        DomainEvent::KeyPressed { key, position } => {
            assert_eq!(key, 'a');
            assert_eq!(position, 5);
        }
        _ => panic!("Expected KeyPressed event"),
    }
}

#[test]
fn test_domain_event_stage_started() {
    let now = std::time::Instant::now();
    let event = DomainEvent::StageStarted { start_time: now };

    match event {
        DomainEvent::StageStarted { start_time } => {
            assert_eq!(start_time, now);
        }
        _ => panic!("Expected StageStarted event"),
    }
}

#[test]
fn test_domain_event_stage_paused() {
    let event = DomainEvent::StagePaused;
    matches!(event, DomainEvent::StagePaused);
}

#[test]
fn test_domain_event_stage_resumed() {
    let event = DomainEvent::StageResumed;
    matches!(event, DomainEvent::StageResumed);
}

#[test]
fn test_domain_event_stage_finalized() {
    let event = DomainEvent::StageFinalized;
    matches!(event, DomainEvent::StageFinalized);
}

#[test]
fn test_domain_event_stage_skipped() {
    let event = DomainEvent::StageSkipped;
    matches!(event, DomainEvent::StageSkipped);
}

#[test]
fn test_domain_event_challenge_loaded() {
    let event = DomainEvent::ChallengeLoaded {
        text: "fn main() {}".to_string(),
        source_path: "main.rs".to_string(),
    };

    match event {
        DomainEvent::ChallengeLoaded { text, source_path } => {
            assert_eq!(text, "fn main() {}");
            assert_eq!(source_path, "main.rs");
        }
        _ => panic!("Expected ChallengeLoaded event"),
    }
}

#[test]
fn test_domain_event_implements_event_trait() {
    let event = DomainEvent::StageFinalized;
    let _any: &dyn Any = event.as_any();
}

#[test]
fn test_domain_event_clone() {
    let event = DomainEvent::KeyPressed {
        key: 'x',
        position: 10,
    };
    let cloned = event.clone();

    match (event, cloned) {
        (
            DomainEvent::KeyPressed {
                key: k1,
                position: p1,
            },
            DomainEvent::KeyPressed {
                key: k2,
                position: p2,
            },
        ) => {
            assert_eq!(k1, k2);
            assert_eq!(p1, p2);
        }
        _ => panic!("Clone failed"),
    }
}

#[test]
fn test_publish_domain_event() {
    let bus = EventBus::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    bus.subscribe(move |event: &DomainEvent| {
        if let DomainEvent::KeyPressed { .. } = event {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }
    });

    bus.publish(DomainEvent::KeyPressed {
        key: 't',
        position: 0,
    });

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn test_multiple_event_types() {
    #[derive(Debug, Clone)]
    struct EventA;
    impl Event for EventA {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[derive(Debug, Clone)]
    struct EventB;
    impl Event for EventB {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    let bus = EventBus::new();
    let counter_a = Arc::new(AtomicUsize::new(0));
    let counter_b = Arc::new(AtomicUsize::new(0));

    let counter_a_clone = Arc::clone(&counter_a);
    let counter_b_clone = Arc::clone(&counter_b);

    bus.subscribe(move |_event: &EventA| {
        counter_a_clone.fetch_add(1, Ordering::SeqCst);
    });

    bus.subscribe(move |_event: &EventB| {
        counter_b_clone.fetch_add(1, Ordering::SeqCst);
    });

    bus.publish(EventA);
    bus.publish(EventB);

    assert_eq!(counter_a.load(Ordering::SeqCst), 1);
    assert_eq!(counter_b.load(Ordering::SeqCst), 1);
}

#[test]
fn test_publish_multiple_times() {
    let bus = EventBus::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    bus.subscribe(move |_event: &TestEvent| {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    });

    for i in 0..5 {
        bus.publish(TestEvent {
            message: format!("test {}", i),
        });
    }

    assert_eq!(counter.load(Ordering::SeqCst), 5);
}

#[test]
fn test_subscriber_receives_event_data() {
    let bus = EventBus::new();
    let received_key = Arc::new(std::sync::Mutex::new(' '));
    let received_position = Arc::new(std::sync::Mutex::new(0usize));

    let key_clone = Arc::clone(&received_key);
    let position_clone = Arc::clone(&received_position);

    bus.subscribe(move |event: &DomainEvent| {
        if let DomainEvent::KeyPressed { key, position } = event {
            *key_clone.lock().unwrap() = *key;
            *position_clone.lock().unwrap() = *position;
        }
    });

    bus.publish(DomainEvent::KeyPressed {
        key: 'z',
        position: 42,
    });

    assert_eq!(*received_key.lock().unwrap(), 'z');
    assert_eq!(*received_position.lock().unwrap(), 42);
}

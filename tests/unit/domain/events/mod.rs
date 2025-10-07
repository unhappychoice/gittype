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

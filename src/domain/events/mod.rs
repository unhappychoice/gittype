use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub mod domain_events;
pub mod ui_events;

pub trait Event: Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
}

pub trait EventHandler<E: Event>: Send + Sync {
    fn handle(&self, event: &E);
}

type BoxedHandler = Box<dyn Fn(&dyn Event) + Send + Sync>;

pub struct EventBus {
    subscribers: Arc<RwLock<HashMap<TypeId, Vec<BoxedHandler>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn publish<E: Event>(&self, event: E) {
        let type_id = TypeId::of::<E>();
        let subscribers = self.subscribers.read().unwrap();

        if let Some(handlers) = subscribers.get(&type_id) {
            for handler in handlers {
                handler(&event);
            }
        }
    }

    pub fn subscribe<E: Event, F>(&self, handler: F)
    where
        F: Fn(&E) + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<E>();
        let mut subscribers = self.subscribers.write().unwrap();

        let boxed_handler: BoxedHandler = Box::new(move |event: &dyn Event| {
            if let Some(concrete_event) = event.as_any().downcast_ref::<E>() {
                handler(concrete_event);
            }
        });

        subscribers
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(boxed_handler);
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for EventBus {
    fn clone(&self) -> Self {
        Self {
            subscribers: Arc::clone(&self.subscribers),
        }
    }
}

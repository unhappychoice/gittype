use shaku::{Component, Interface};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub mod domain_events;

pub trait Event: Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
}

pub trait EventHandler<E: Event>: Send + Sync {
    fn handle(&self, event: &E);
}

type BoxedHandler = Arc<dyn Fn(&dyn Event) + Send + Sync>;

pub trait EventBusInterface: Interface {
    fn as_event_bus(&self) -> &EventBus;
}

#[derive(Clone, Component)]
#[shaku(interface = EventBusInterface)]
pub struct EventBus {
    #[shaku(default)]
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

        let handlers: Vec<BoxedHandler> = {
            let subscribers = self.subscribers.read().unwrap();
            let h = subscribers.get(&type_id).cloned().unwrap_or_default();
            h
        };

        for handler in handlers.iter() {
            handler(&event);
        }
    }

    pub fn subscribe<E: Event, F>(&self, handler: F)
    where
        F: Fn(&E) + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<E>();
        let mut subscribers = self.subscribers.write().unwrap();

        let boxed_handler: BoxedHandler = Arc::new(move |event: &dyn Event| {
            if let Some(concrete_event) = event.as_any().downcast_ref::<E>() {
                handler(concrete_event);
            }
        });

        subscribers.entry(type_id).or_default().push(boxed_handler);
    }
}

impl EventBusInterface for EventBus {
    fn as_event_bus(&self) -> &EventBus {
        self
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

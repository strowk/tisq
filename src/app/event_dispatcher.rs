use std::{sync::{Arc, Mutex}, fmt::Debug};

use tuirealm::{
    listener::{ListenerResult, Poll},
    Event,
};

use crate::components::SentTree;

use super::TisqEvent;

#[derive(Clone)]
pub(crate) struct EventDispatcherPort<T>
where
    T: Eq + PartialEq + Clone + PartialOrd + 'static,
{
    pub holder: Arc<Mutex<BrowserUpdateHolder<T>>>,
}

pub(crate) struct BrowserUpdateHolder<T>
where
    T: Eq + PartialEq + Clone + PartialOrd + 'static,
{
    pub events: Vec<Event<T>>,
}

impl<T> EventDispatcherPort<T>
where
    T: Eq + PartialEq + Clone + PartialOrd + 'static,
{
    pub(crate) fn new() -> Self {
        Self {
            holder: Arc::new(Mutex::new(BrowserUpdateHolder { events: Vec::new() })),
        }
    }

    pub(crate) fn dispatch(&mut self, event: Event<T>) {
        self.holder.lock().unwrap().events.push(event);
    }
}
impl<T> Poll<T> for EventDispatcherPort<T>
where
    T: Eq + PartialEq + Clone + PartialOrd + Send + 'static + Debug,
{
    fn poll(&mut self) -> ListenerResult<Option<Event<T>>> {
        let mut holder = self.holder.lock().unwrap();
        let event = holder.events.pop();
        // if event.is_some() {
            // println!("EventDispatcherPort::poll: {:?}", event);
        // }
        Ok(event)
    }
}

impl EventDispatcherPort<TisqEvent> {
    pub(crate) fn send_tree(&mut self, tree: SentTree) {
        self.dispatch(Event::User(TisqEvent::TreeReloaded(tree)));
    }
}

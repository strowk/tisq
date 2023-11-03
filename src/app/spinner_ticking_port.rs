use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use tuirealm::{
    listener::{ListenerResult, Poll},
    Event,
};

use super::TisqEvent;

struct SpinnerTickingState {
    ticking: bool,
}

#[derive(Clone)]
pub(crate) struct SpinnerTickingPort {
    state: Arc<Mutex<SpinnerTickingState>>,
}

impl SpinnerTickingPort {
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(SpinnerTickingState { ticking: false })),
        }
    }

    pub(crate) fn set_ticking(&mut self, ticking: bool) {
        self.state.lock().unwrap().ticking = ticking;
    }
}

impl Poll<TisqEvent> for SpinnerTickingPort {
    fn poll(&mut self) -> ListenerResult<Option<Event<TisqEvent>>> {
        let state = self.state.lock().unwrap();
        if !state.ticking {
            return Ok(None);
        }
        Ok(Some(Event::User(TisqEvent::SpinnerTick)))
    }
}

use tui_realm_stdlib::Phantom;
use tuirealm::{Component, Event, MockComponent, Sub, SubClause, SubEventClause};

use crate::{
    app::{DbResponse, KeySubClause, SectionKeybindings, TisqEvent, TisqKeyboundAction},
    Id, Msg,
};

#[derive(MockComponent)]
pub struct DbResponseStatusListener {
    component: Phantom,
}

impl DbResponseStatusListener {
    pub(crate) fn new() -> Self {
        Self {
            component: Phantom::default(),
        }
    }

    pub(crate) fn subscriptions() -> Vec<Sub<Id, TisqEvent>> {
        vec![Sub::new(
            SubEventClause::User(TisqEvent::DbResponse(DbResponse::None)),
            // due to comparison of TisqEvent, it does not matter which DbResponse is used
            SubClause::Always,
        )]
    }
}

impl Component<Msg, TisqEvent> for DbResponseStatusListener {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        match ev {
            Event::User(TisqEvent::DbResponse(_)) => Some(Msg::PopDbRequestStatus),
            _ => None,
        }
    }
}

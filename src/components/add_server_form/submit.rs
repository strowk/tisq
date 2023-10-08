use tui_realm_stdlib::Phantom;
use tuirealm::{
    event::{Key, KeyEvent, KeyEventKind, KeyModifiers},
    Component, Event, MockComponent, Sub, SubClause, SubEventClause,
};

use crate::{app::TisqEvent, Id, Msg};

#[derive(Default, MockComponent)]
pub struct FormSubmitListener {
    component: Phantom,
}

impl FormSubmitListener {
    pub(crate) fn get_subscription() -> Sub<Id, TisqEvent> {
        Sub::new(
            SubEventClause::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }),
            SubClause::IsMounted(Id::ConnectionUrlInput),
        )
    }
}

impl Component<Msg, TisqEvent> for FormSubmitListener {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }) => Some(Msg::SubmitAddServerForm),
            _ => None,
        }
    }
}

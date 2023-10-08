use tui_realm_stdlib::Phantom;
use tuirealm::{MockComponent, NoUserEvent, Component, Event, event::{KeyEvent, Key, KeyModifiers, KeyEventKind}};

use crate::{Msg, app::TisqEvent};


#[derive(Default, MockComponent)]
pub struct GlobalListener {
    component: Phantom,
}

impl Component<Msg, TisqEvent> for GlobalListener {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press
            }) => Some(Msg::AppClose),
            _ => None,
        }
    }
}
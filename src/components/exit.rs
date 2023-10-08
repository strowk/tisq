use std::ops::BitOr;

use tui_realm_stdlib::Phantom;
use tuirealm::{
    event::{Key, KeyEvent, KeyEventKind, KeyModifiers},
    Component, Event, MockComponent, Sub, SubClause, SubEventClause,
};

use crate::{app::TisqEvent, Id, Msg};

#[derive(Default, MockComponent)]
pub struct GlobalListener {
    component: Phantom,
}

impl GlobalListener {
    pub(crate) fn switch_location(key: Key) -> Option<Msg> {
        match key {
            Key::Left => Some(Msg::NavigateLeft),
            Key::Right => Some(Msg::NavigateRight),
            _ => None,
        }
    }

    pub(crate) fn subscriptions() -> Vec<Sub<Id, TisqEvent>> {
        vec![
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Esc,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                }),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Left,
                    modifiers: KeyModifiers::ALT,
                    kind: KeyEventKind::Press,
                }),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Right,
                    modifiers: KeyModifiers::ALT,
                    kind: KeyEventKind::Press,
                }),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Left,
                    modifiers: KeyModifiers::ALT.bitor(KeyModifiers::CONTROL),
                    kind: KeyEventKind::Press,
                }),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent {
                    code: Key::Right,
                    modifiers: KeyModifiers::ALT.bitor(KeyModifiers::CONTROL),
                    kind: KeyEventKind::Press,
                }),
                SubClause::Always,
            ),
        ]
    }
}

impl Component<Msg, TisqEvent> for GlobalListener {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        let alt_control: KeyModifiers = KeyModifiers::ALT.bitor(KeyModifiers::CONTROL);

        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }) => Some(Msg::AppClose),

            Event::Keyboard(
                key @ KeyEvent {
                    code: Key::Left | Key::Right,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::ALT,
                },
            ) => Self::switch_location(key.code),

            Event::Keyboard(
                key @ KeyEvent {
                    code: Key::Left | Key::Right,
                    kind: KeyEventKind::Press,
                    modifiers,
                },
            ) if modifiers == alt_control => Self::switch_location(key.code),

            _ => None,
        }
    }
}

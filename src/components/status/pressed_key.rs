use tui_realm_stdlib::{Span, Spinner};
use tuirealm::{
    event::{self, KeyEvent},
    props::{Color, PropPayload, PropValue, TextSpan},
    AttrValue, Attribute, Component, Event, MockComponent, Sub, SubClause, SubEventClause,
};

use crate::{
    app::{KeybindingKeyPress, TisqEvent},
    tui::{Id, Msg},
};

#[derive(MockComponent)]
pub(crate) struct PressedKey {
    // last_pressed_key: Option<KeyEvent>,
    component: Span,
}

impl Default for PressedKey {
    fn default() -> Self {
        Self {
            // last_pressed_key: None,
            component: Span::default()
                .foreground(Color::Green)
                .spans(&[TextSpan::new("").underlined()]),
        }
    }
}

impl PressedKey {
    pub(crate) fn subscriptions(enabled_showing_pressed_key: bool) -> Vec<Sub<Id, TisqEvent>> {
        if enabled_showing_pressed_key {
            vec![Sub::new(SubEventClause::Any, SubClause::Always)]
        } else {
            vec![]
        }
    }
}

impl Component<Msg, TisqEvent> for PressedKey {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        if let Event::Keyboard(input) = ev {
            // self.last_pressed_key = Some(input);
            let key_press = &KeybindingKeyPress {
                key: input.code,
                modifiers: input.modifiers,
            };

            self.component.attr(
                Attribute::Text,
                AttrValue::Payload(PropPayload::Vec(vec![PropValue::TextSpan(TextSpan::new(
                    format!("{}", key_press),
                ))])),
            );
            return Some(Msg::None);
        }
        None
    }
}

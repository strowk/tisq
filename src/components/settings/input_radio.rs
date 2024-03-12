use tui_realm_stdlib::Radio;
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::event::KeyEventKind;
use tuirealm::props::{Alignment, BorderType, Borders, Color};
use tuirealm::{
    event::{Key, KeyEvent},
    Component, Event, MockComponent
};
use tuirealm::{State, StateValue};

use crate::app::TisqEvent;
use crate::tui::Msg;

#[derive(MockComponent)]
pub(crate) struct RadioInput {
    component: Radio,
}

impl RadioInput {
    pub(crate) fn new(enabled_showing_pressed_key: bool) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightGreen),
                )
                .foreground(Color::LightGreen)
                .title("Toggle to show the last entered key", Alignment::Center)
                .rewind(true)
                .choices(&["OFF", "ON"])
                .value(if enabled_showing_pressed_key { 1 } else { 0 }),
        }
    }
}

impl Component<Msg, TisqEvent> for RadioInput {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let res = self.perform(Cmd::Move(Direction::Left));
                match self.component.state() {
                    State::One(StateValue::Usize(choice)) => {
                        return Some(Msg::SetEnabledLastEnteredKey(choice == 1));
                    }
                    _ => {}
                };
                res
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let res = self.perform(Cmd::Move(Direction::Right));
                match self.component.state() {
                    State::One(StateValue::Usize(choice)) => {
                        return Some(Msg::SetEnabledLastEnteredKey(choice == 1));
                    }
                    _ => {}
                };
                res
            }
            // Event::Keyboard(KeyEvent {
            //     code: Key::Enter,
            //     kind: KeyEventKind::Press,
            //     ..
            // }) => self.perform(Cmd::Submit),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

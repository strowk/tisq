use tui_realm_stdlib::Input;
use tuirealm::{
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent, KeyEventKind},
    props::{Alignment, BorderType, Borders, Color, InputType, Style},
    Component, Event, MockComponent,
};

use crate::{app::TisqEvent, Msg};

#[derive(MockComponent)]
pub(crate) struct InputText {
    component: Input,
}

impl InputText {
    pub(crate) fn new(title: &str, value: &str) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightYellow),
                )
                .foreground(Color::LightYellow)
                .input_type(InputType::Text)
                .title(title, Alignment::Left)
                .value(value)
                .invalid_style(Style::default().fg(Color::Red)),
        }
    }
}

impl Component<Msg, TisqEvent> for InputText {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        tracing::debug!("InputText Event: {:?}", ev);
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                kind: KeyEventKind::Press,
                ..
            }) => return Some(Msg::FocusPreviousInput),
            Event::Keyboard(KeyEvent {
                code: Key::Down | Key::Tab,
                kind: KeyEventKind::Press,
                ..
            }) => return Some(Msg::FocusNextInput),
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                kind: KeyEventKind::Press,
                ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                kind: KeyEventKind::Press,
                ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Home,
                kind: KeyEventKind::Press,
                ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete,
                kind: KeyEventKind::Press,
                ..
            }) => self.perform(Cmd::Cancel),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                kind: KeyEventKind::Press,
                ..
            }) => self.perform(Cmd::Delete),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                kind: KeyEventKind::Press,
                // modifiers: KeyModifiers::NONE,
                ..
            }) => self.perform(Cmd::Type(ch)),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

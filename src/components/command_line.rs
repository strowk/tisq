use tui_realm_stdlib::Input;
use tuirealm::{
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent, KeyEventKind},
    props::{Alignment, BorderSides, BorderType, Borders, Color, InputType, Style},
    Component, Event, MockComponent, State, StateValue,
};

use crate::{app::TisqEvent, Msg};

#[derive(MockComponent)]
pub(crate) struct CommandLine {
    component: Input,
}

impl CommandLine {
    pub(crate) fn new(title: &str, value: &str) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Plain)
                        .sides(BorderSides::ALL)
                        .color(Color::DarkGray),
                )
                .foreground(Color::LightCyan)
                .input_type(InputType::Text)
                .title("type command and press Enter", Alignment::Left)
                .value(value)
                .invalid_style(Style::default().fg(Color::Red)),
        }
    }

    fn get_text(&self) -> Option<String> {
        match self.component.state() {
            State::One(StateValue::String(value)) => Some(value),
            _ => None,
        }
    }
}

impl Component<Msg, TisqEvent> for CommandLine {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        // tracing::debug!("InputText Event: {:?}", ev);
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
            }) => {
                if let Some(text) = self.get_text() {
                    if text.len() > 1 {
                        self.perform(Cmd::Delete);
                    }
                };
                CmdResult::None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                kind: KeyEventKind::Press,
                // modifiers: KeyModifiers::NONE,
                ..
            }) => self.perform(Cmd::Type(ch)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                kind: KeyEventKind::Press,
                // modifiers: KeyModifiers::NONE,
                ..
            }) => match self.get_text() {
                Some(value) => {
                    if value.starts_with(":") {
                        return Some(Msg::ExecuteCommand(
                            value.trim_start_matches(":").to_string(),
                        ));
                    }
                    return Some(Msg::ExecuteCommand(value));
                }
                _ => {
                    return Some(Msg::None);
                }
            },
            // Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

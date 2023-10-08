use std::time::Duration;

use tui_realm_stdlib::{Paragraph, Table};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, BorderSides, BorderType, Borders, Color, PropPayload, PropValue, TableBuilder,
    TextSpan,
};
use tuirealm::terminal::TerminalBridge;
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update,
};
use tuirealm::{AttrValue, Attribute};
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};

use crate::app::{DbResponse, TisqEvent};
use crate::Msg;

#[derive(PartialEq, PartialOrd, Clone, Eq, Debug)]
pub(crate) struct QueryResult {
    pub headers: Vec<String>,
    pub data: Vec<Vec<String>>,
}

#[derive(MockComponent)]
pub(crate) struct ErrorResult {
    component: Paragraph,
}

impl Default for ErrorResult {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                .borders(Borders::default().sides(BorderSides::NONE))
                .title("Execution Error", Alignment::Center),
        }
    }
}

impl ErrorResult {
    fn set_message(&mut self, message: String) {
        self.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(
                [message]
                    .iter()
                    .cloned()
                    .map(|msg| TextSpan::from(msg))
                    .map(PropValue::TextSpan)
                    .collect(),
            )),
        );
    }
}

impl Component<Msg, TisqEvent> for ErrorResult {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::User(TisqEvent::DbResponse(DbResponse::Error(_, message))) => {
                self.set_message(message);
                return Some(Msg::ShowErrorResult);
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

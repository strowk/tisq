use std::time::Duration;

use tui_realm_stdlib::Table;
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
pub(crate) struct ExecuteResultTable {
    component: Table,
}

impl Default for ExecuteResultTable {
    fn default() -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Thick)
                        .sides(BorderSides::NONE)
                        .color(Color::Yellow),
                )
                .title("Query Result", Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::LightYellow)
                .highlighted_str("🚀")
                .rewind(true)
                .step(4)
                .row_height(1)
                // .headers(&["Column 1"])
                .column_spacing(3)
                .widths(&[30, 20, 50]),
        }
    }
}

impl ExecuteResultTable {
    fn set_result(&mut self, result: QueryResult) {
        // println!("Setting result: {:?}", result);
        // self.component.table(TableBuilder::default().build());

        let mut builder = TableBuilder::default();

        result.data.iter().for_each(|row| {
            row.iter().for_each(|col| {
                builder.add_col(TextSpan::from(col));
            });
            builder.add_row();
        });

        self.component.attr(
            tuirealm::Attribute::Content,
            AttrValue::Table(builder.build()),
        );

        self.attr(
            Attribute::Width,
            AttrValue::Payload(PropPayload::Vec(
                self.widths(&result)
                    .iter()
                    .map(|x| PropValue::U16(*x))
                    .collect(),
            )),
        );

        self.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(
                result
                    .headers
                    .iter()
                    .map(|x| PropValue::Str(x.to_string()))
                    .collect(),
            )),
        );
    }

    fn widths(&self, result: &QueryResult) -> Vec<u16> {
        // select maximum width for each column
        let mut absolute_widths: Vec<u16> = vec![];
        fn update_widths(widths: &mut Vec<u16>, col: &str, i: usize) {
            if widths.len() <= i {
                widths.push(0);
            }
            if widths[i] < col.len() as u16 {
                widths[i] = col.len() as u16;
            }
        }
        result.data.iter().for_each(|row| {
            row.iter()
                .enumerate()
                .for_each(|(i, col)| update_widths(&mut absolute_widths, col, i));
        });
        result
            .headers
            .iter()
            .enumerate()
            .for_each(|(i, col)| update_widths(&mut absolute_widths, col, i));
        // log::debug!("widths: {:?}", widths);
        let total = absolute_widths.iter().sum::<u16>();

        // transform to percentages
        absolute_widths
            .iter()
            .map(|x| (*x as f64 / total as f64 * 100.0) as u16)
            .collect::<Vec<u16>>()

        // TODO: change underlying table to use absolute widths instead of percentages
        
        // absolute_widths

        // if result.data.is_empty() {
        //     return vec![];
        // }
        // result.data[0]
        //     .iter()
        //     .map(|x| x.len() as u16)
        //     .collect::<Vec<u16>>()
    }
}

impl Component<Msg, TisqEvent> for ExecuteResultTable {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        // println!("ExecuteResultTable event: {:?}", ev);
        let _ = match ev {
            Event::User(TisqEvent::DbResponse(DbResponse::Executed(_, headers, data))) => {
                self.set_result(QueryResult { headers, data });
                return Some(Msg::ShowFetchedTable);
            }

            Event::User(TisqEvent::DbResponse(DbResponse::Error(_, data))) => {
                // TODO: display error
                return Some(Msg::None);
            }
            // Event::User(TisqEvent::QueryResultFetched(result)) => {
            //     self.set_result(result);
            //     return Some(Msg::None);
            // }
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

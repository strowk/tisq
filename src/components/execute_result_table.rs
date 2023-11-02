use tui_realm_stdlib::Table;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{KeyEventKind, KeyModifiers};
use tuirealm::props::{
    Alignment, BorderSides, Borders, Color, PropPayload, PropValue, TableBuilder, TextSpan,
};
use tuirealm::{
    event::{Key, KeyEvent},
    Component, Event, MockComponent,
};
use tuirealm::{AttrValue, Attribute};

use crate::app::{DbResponse, SectionKeybindings, TisqEvent, TisqKeyboundAction};
use crate::Msg;

#[derive(PartialEq, PartialOrd, Clone, Eq, Debug)]
pub(crate) struct QueryResult {
    pub headers: Vec<String>,
    pub data: Vec<Vec<String>>,
}

#[derive(MockComponent)]
pub(crate) struct ExecuteResultTable {
    component: Table,
    result: Option<QueryResult>,
    column_offset: usize,
    keybindings: SectionKeybindings<TisqKeyboundAction>,
}

impl ExecuteResultTable {
    pub(crate) fn new(keybindings: SectionKeybindings<TisqKeyboundAction>) -> Self {
        Self {
            column_offset: 0,
            result: None,
            keybindings,
            component: Table::default()
                .borders(
                    Borders::default()
                        // .modifiers(BorderType::Thick)
                        // .modifiers(BorderType::Plain)
                        .sides(BorderSides::NONE), // .sides(BorderSides::TOP)
                                                   // .color(Color::LightCyan),
                )
                .title("Query Result", Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::LightYellow)
                // .highlighted_color(Color::LightCyan)
                .highlighted_str("🚀")
                .rewind(true)
                .step(4)
                .row_height(1)
                // .headers(&["Column 1"])
                .column_spacing(3)
                .widths(&[30, 20, 50]),
        }
    }

    fn update_result(&mut self) {
        if let Some(result) = self.result.as_ref() {
            let mut builder = TableBuilder::default();

            result.data.iter().for_each(|row| {
                row.iter().skip(self.column_offset).for_each(|col| {
                    builder.add_col(TextSpan::from(col));
                });
                builder.add_row();
            });

            self.component.attr(
                tuirealm::Attribute::Content,
                AttrValue::Table(builder.build()),
            );

            let widths = self.widths(&result).clone();

            self.attr(
                Attribute::Text,
                AttrValue::Payload(PropPayload::Vec(
                    result
                        .headers
                        .iter()
                        .skip(self.column_offset)
                        .map(|x| PropValue::Str(x.to_string()))
                        .collect(),
                )),
            );

            self.attr(
                Attribute::Width,
                AttrValue::Payload(PropPayload::Vec(
                    widths.iter().map(|x| PropValue::U16(*x)).collect(),
                )),
            );
        }
    }

    fn set_result(&mut self, result: QueryResult, column_offet: usize) {
        // println!("Setting result: {:?}", result);
        // self.component.table(TableBuilder::default().build());
        self.column_offset = column_offet;
        self.result = Some(result);
        self.update_result();
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
                .skip(self.column_offset)
                .enumerate()
                .for_each(|(i, col)| update_widths(&mut absolute_widths, col, i));
        });
        result
            .headers
            .iter()
            .skip(self.column_offset)
            .enumerate()
            .for_each(|(i, col)| update_widths(&mut absolute_widths, col, i));
        // tracing::debug!("widths: {:?}", widths);
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

// TODO: could be cool to support scroll "mouse" events, such as:
// https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html

impl Component<Msg, TisqEvent> for ExecuteResultTable {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        // println!("ExecuteResultTable event: {:?}", ev);
        if let Event::Keyboard(kb_event) = ev {
            match self.keybindings.get_action(&kb_event) {
                Some(TisqKeyboundAction::ResultOffsetColumnLeft) => {
                    self.column_offset = if self.column_offset > 0 {
                        self.column_offset - 1
                    } else {
                        0
                    };
                    self.update_result();
                    return Some(Msg::None);
                }
                Some(TisqKeyboundAction::ResultOffsetColumnRight) => {
                    self.column_offset = if let Some(result) = self.result.as_ref() {
                        if self.column_offset < result.headers.len() - 1 {
                            self.column_offset + 1
                        } else {
                            self.column_offset
                        }
                    } else {
                        0
                    };
                    self.update_result();
                    return Some(Msg::None);
                }
                _ => (),
            }
        };
        let _ = match ev {
            Event::User(TisqEvent::DbResponse(DbResponse::Executed(_, headers, data))) => {
                self.set_result(QueryResult { headers, data }, 0);
                return Some(Msg::ShowFetchedTable);
            }
            Event::User(TisqEvent::DbResponse(DbResponse::ConnectionIsDown {
                original_request,
                ..
            })) => {
                // Event::User(TisqEvent::DbResponse(DbResponse::ConnectionIsDown(id, name, query))) => {

                return Some(Msg::ReconnectAndRepeat(original_request));
                // return Some(Msg::ReconnectAndExecuteQuery(
                //     EditorId::new(id, name),
                //     query,
                // ));
            }
            // Event::User(TisqEvent::DbResponse(DbResponse::Error(_, data))) => {
            //     return Some(Msg::None);
            // }
            // Event::User(TisqEvent::QueryResultFetched(result)) => {
            //     self.set_result(result);
            //     return Some(Msg::None);
            // }
            // Event::Keyboard(KeyEvent {
            //     code: code @ (Key::Left | Key::Right),
            //     kind: KeyEventKind::Press,
            //     modifiers: KeyModifiers::CONTROL,
            // }) => {
            //     if let Some(result) = self.result.as_ref() {
            //         self.column_offset = match code {
            //             Key::Left => {
            //                 if self.column_offset > 0 {
            //                     self.column_offset - 1
            //                 } else {
            //                     0
            //                 }
            //             }
            //             Key::Right => {
            //                 if self.column_offset < result.headers.len() - 1 {
            //                     self.column_offset + 1
            //                 } else {
            //                     self.column_offset
            //                 }
            //             }
            //             _ => 0,
            //         };
            //         self.update_result();
            //     }
            //     CmdResult::None
            // }
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Home,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent {
                code: Key::End,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::GoTo(Position::End)),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

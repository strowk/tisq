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
use tuirealm::{AttrValue, Attribute, State, StateValue};

use crate::app::{DbResponse, SectionKeybindings, Snippet, TisqEvent, TisqKeyboundAction};
use crate::Msg;

#[derive(PartialEq, PartialOrd, Clone, Eq, Debug)]
pub(crate) struct QueryResult {
    pub headers: Vec<String>,
    pub data: Vec<Vec<String>>,
}

#[derive(MockComponent)]
pub(crate) struct SnippetsTable {
    component: Table,
    snippet_shortcuts: Vec<String>,
    // snippets: Vec<&Snippet>,
    // column_offset: usize,
    // keybindings: SectionKeybindings<TisqKeyboundAction>,
}

impl SnippetsTable {
    pub(crate) fn new(
        // keybindings: SectionKeybindings<TisqKeyboundAction>
        snippets: Vec<&Snippet>,
    ) -> Self {
        let mut result = Self {
            // snippets,
            // keybindings,
            snippet_shortcuts: snippets.iter().map(|x| x.shortcut.clone()).collect(),
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
                .column_spacing(2)
                .widths(&[30, 50]),
        };
        result.update_result(snippets);
        result
    }

    fn update_result(&mut self, snippets: Vec<&Snippet>) {
        let mut builder = TableBuilder::default();

        snippets.iter().for_each(|snippet| {
            builder.add_col(TextSpan::from(snippet.shortcut.clone()));
            builder.add_col(TextSpan::from(snippet.description.clone()));
            builder.add_row();
        });

        self.component.attr(
            tuirealm::Attribute::Content,
            AttrValue::Table(builder.build()),
        );

        let widths = self.widths(&snippets).clone();

        self.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(
                ["shortcut", "description"]
                    .iter()
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

    // fn set_result(&mut self, result: QueryResult, column_offet: usize) {
    //     // println!("Setting result: {:?}", result);
    //     // self.component.table(TableBuilder::default().build());
    //     self.column_offset = column_offet;
    //     self.result = Some(result);
    //     self.update_result();
    // }

    fn widths(&self, snippets: &Vec<&Snippet>) -> Vec<u16> {
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
        snippets.iter().for_each(|snippet| {
            [&snippet.shortcut, &snippet.description]
                .iter()
                .enumerate()
                .for_each(|(i, col)| update_widths(&mut absolute_widths, col, i));
        });

        ["shortcut", "description"]
            .iter()
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
    }

    fn apply_snippet(&self) -> Option<Msg> {
        match self.component.state() {
            State::One(StateValue::Usize(list_index)) => {
                if let Some(shortcut) = self.snippet_shortcuts.get(list_index) {
                    return Some(Msg::ApplySnippet(shortcut.clone()));
                };
                None
            }
            _ => None,
        }
    }
}

impl Component<Msg, TisqEvent> for SnippetsTable {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        // println!("ExecuteResultTable event: {:?}", ev);
        // if let Event::Keyboard(kb_event) = ev {
        //     match self.keybindings.get_action(&kb_event) {
        //         Some(TisqKeyboundAction::ResultOffsetColumnLeft) => {
        //             self.column_offset = if self.column_offset > 0 {
        //                 self.column_offset - 1
        //             } else {
        //                 0
        //             };
        //             self.update_result();
        //             return Some(Msg::None);
        //         }
        //         Some(TisqKeyboundAction::ResultOffsetColumnRight) => {
        //             self.column_offset = if let Some(result) = self.result.as_ref() {
        //                 if self.column_offset < result.headers.len() - 1 {
        //                     self.column_offset + 1
        //                 } else {
        //                     self.column_offset
        //                 }
        //             } else {
        //                 0
        //             };
        //             self.update_result();
        //             return Some(Msg::None);
        //         }
        //         _ => (),
        //     }
        // };
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
            }) => {
                return self.apply_snippet();
            }

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

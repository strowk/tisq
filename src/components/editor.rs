use itertools::Itertools;
use std::ops::BitOr;
#[cfg(feature = "search")]
use tuirealm::StateValue;
use tuirealm::{
    command::{Cmd, CmdResult, Direction, Position},
    event::{Event, Key, KeyEvent, KeyEventKind, KeyModifiers},
    props::{
        Alignment, AttrValue, Attribute, BorderSides, BorderType, Borders, Color, PropPayload,
        PropValue, Style, TextModifiers,
    },
    Component, MockComponent, State, StateValue,
};
// tui

// label
#[cfg(feature = "search")]
use tui_realm_stdlib::Input;

// textarea
#[cfg(feature = "clipboard")]
use tui_realm_textarea::TEXTAREA_CMD_PASTE;
use tui_realm_textarea::{
    TextArea, TEXTAREA_CMD_DEL_NEXT_WORD, TEXTAREA_CMD_DEL_WORD, TEXTAREA_CMD_MOVE_BOTTOM,
    TEXTAREA_CMD_MOVE_TOP, TEXTAREA_CMD_MOVE_WORD_BACK, TEXTAREA_CMD_MOVE_WORD_FORWARD,
    TEXTAREA_CMD_NEWLINE, TEXTAREA_CMD_PASTE, TEXTAREA_CMD_REDO, TEXTAREA_CMD_UNDO,
    TEXTAREA_CURSOR_POSITION,
};
#[cfg(feature = "search")]
use tui_realm_textarea::{
    TEXTAREA_CMD_SEARCH_BACK, TEXTAREA_CMD_SEARCH_FORWARD, TEXTAREA_SEARCH_PATTERN,
};

use crate::{
    app::{EditorId, SectionKeybindings, TisqEvent, TisqKeyboundAction},
    Id, Msg,
};

pub struct Editor<'a> {
    component: TextArea<'a>,
    keybindings: SectionKeybindings<TisqKeyboundAction>,
    editor_id: EditorId,
}

impl<'a> MockComponent for Editor<'a> {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        self.component.view(frame, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, query: Attribute, attr: AttrValue) {
        self.component.attr(query, attr)
    }

    fn state(&self) -> State {
        self.component.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl<'a> Editor<'a> {
    pub(crate) fn new(
        editor_id: EditorId,
        keybindings: SectionKeybindings<TisqKeyboundAction>,
    ) -> Self {
        let textarea = TextArea::default();
        Self {
            editor_id,
            keybindings,
            component: textarea
                .borders(
                    Borders::default()
                        .color(Color::LightYellow)
                        .sides(BorderSides::NONE)
                        .modifiers(BorderType::Rounded),
                )
                .cursor_line_style(Style::default())
                .cursor_style(Style::default().add_modifier(TextModifiers::REVERSED))
                // .footer_bar("<ctrl>+<alt>+<enter> to execute", Style::default())
                .line_number_style(
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(TextModifiers::ITALIC),
                )
                .max_histories(64)
                .scroll_step(4)
                // .status_bar(
                // "Ln {ROW}, Col {COL}",
                // Style::default().add_modifier(TextModifiers::REVERSED),
                // )
                .tab_length(4)
                .title("Query Editor", Alignment::Center),
        }
    }

    fn get_text(&self) -> Option<String> {
        match self.component.state() {
            State::Vec(vector) => Some(
                vector
                    .iter()
                    .flat_map(|x| match x {
                        StateValue::String(text) => Some(text),
                        _ => None,
                    })
                    .join("\n"),
            ),
            _ => return None,
        }
    }

    fn execute_message(&mut self) -> Msg {
        Msg::ExecuteQuery(
            self.editor_id.clone(),
            self.get_text().unwrap_or("".to_string()),
            0,
        )
    }
}

impl<'a> Component<Msg, TisqEvent> for Editor<'a> {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        if let Event::Keyboard(_) = ev {
            tracing::debug!("matching key {:?}", ev);
        }

        let res_message = match ev {
            Event::Keyboard(kb_event) => match self.keybindings.get_action(&kb_event) {
                Some(&TisqKeyboundAction::EditorTryExpand) => Some(Msg::EditorTryExpand(
                    self.editor_id.clone(),
                    self.component.get_current_line(),
                )),
                Some(&TisqKeyboundAction::EditorToggleComment) => {
                    let current_line = self.component.get_current_line();

                    let (return_to_row, mut return_to_column) = match self
                        .component
                        .query(Attribute::Custom(TEXTAREA_CURSOR_POSITION))
                    {
                        Some(AttrValue::Payload(PropPayload::Tup2((
                            PropValue::Usize(row),
                            PropValue::Usize(column),
                        )))) => (row, column),
                        _ => return Some(Msg::None),
                    };
                    // tracing::debug!("starting on  {}", column);
                    self.perform(Cmd::GoTo(Position::Begin));

                    let should_comment = !current_line.starts_with("--");

                    if should_comment {
                        self.component.add_text("-- ");
                        self.perform(Cmd::Delete); // add_text would add one extra endline
                        return_to_column = return_to_column + 3;
                    } else {
                        self.perform(Cmd::Cancel);
                        self.perform(Cmd::Cancel);

                        return_to_column = return_to_column - 2;
                        if current_line.starts_with("-- ") {
                            self.perform(Cmd::Cancel);
                            return_to_column = return_to_column - 1;
                        }
                    }

                    self.component.attr(
                        Attribute::Custom(TEXTAREA_CURSOR_POSITION),
                        AttrValue::Payload(PropPayload::Tup2((
                            PropValue::U16(return_to_row.try_into().unwrap()),
                            PropValue::U16(return_to_column.try_into().unwrap()),
                        ))),
                    );
                    Some(Msg::None)
                }
                Some(&TisqKeyboundAction::EditorNextTab) => Some(Msg::NextEditor),
                Some(&TisqKeyboundAction::EditorPrevTab) => Some(Msg::PreviousEditor),
                Some(&TisqKeyboundAction::EditorMoveTabLeft) => {
                    Some(Msg::MoveTabLeft(self.editor_id.clone()))
                }
                Some(&TisqKeyboundAction::EditorMoveTabRight) => {
                    Some(Msg::MoveTabRight(self.editor_id.clone()))
                }
                Some(&TisqKeyboundAction::EditorCloseTab) => {
                    Some(Msg::CloseTab(self.editor_id.clone()))
                }
                // Some(&TisqKeyboundAction::EditorBackspace) => {
                //     self.perform(Cmd::Delete);
                //     Some(Msg::None)
                // }
                Some(&TisqKeyboundAction::EditorExecute) => Some(self.execute_message()),
                Some(&TisqKeyboundAction::EditorPaste) => {
                    self.perform(Cmd::Custom(TEXTAREA_CMD_PASTE));
                    Some(Msg::None)
                }
                Some(&TisqKeyboundAction::EditorDeleteWord) => {
                    self.perform(Cmd::Custom(TEXTAREA_CMD_DEL_WORD));
                    Some(Msg::None)
                }
                Some(&TisqKeyboundAction::EditorDeleteNextWord) => {
                    self.perform(Cmd::Custom(TEXTAREA_CMD_DEL_NEXT_WORD));
                    Some(Msg::None)
                }
                Some(&TisqKeyboundAction::EditorMoveToTop) => {
                    self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_TOP));
                    self.perform(Cmd::GoTo(Position::Begin));
                    Some(Msg::None)
                }
                Some(&TisqKeyboundAction::EditorMoveToBottom) => {
                    self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_BOTTOM));
                    self.perform(Cmd::GoTo(Position::End));
                    Some(Msg::None)
                }
                // Some(&TisqKeyboundAction::GlobalExit) => Some(Msg::AppClose),
                // Some(
                //     action @ (&TisqKeyboundAction::GlobalNavigateLeft
                //     | &TisqKeyboundAction::GlobalNavigateRight
                //     | &TisqKeyboundAction::GlobalNavigateUp
                //     | &TisqKeyboundAction::GlobalNavigateDown),
                // ) => Self::switch_location_nav(action),
                // Some(&TisqKeyboundAction::GlobalCycleNavigation) => Some(Msg::CycleNavigation),
                // None => None,
                _ => None,
            },
            _ => None,
        };

        res_message.or_else(|| {
            match ev {
                Event::User(TisqEvent::EditorSnippetResolve(editor_id, content)) => {
                    if self.editor_id != editor_id {
                        return None;
                    }
                    self.perform(Cmd::Custom(TEXTAREA_CMD_DEL_WORD)); // removing snippet
                    self.component.add_text(&content);
                    Some(Msg::None)
                }
                Event::User(TisqEvent::EditorContentAdd(editor_id, content)) => {
                    // self.component.attr(attr, value)
                    // tracing::debug!("editor content reset for {:?}, check in {:?}", editor_id, self.editor_id);
                    if self.editor_id != editor_id {
                        return None;
                    }
                    self.component.add_text(&content);
                    Some(Msg::None)
                }
                // Event::Keyboard(KeyEvent {
                //     code: Key::Left | Key::Right,
                //     kind: KeyEventKind::Press,
                //     modifiers: KeyModifiers::ALT,
                // }) => Some(Msg::ChangeFocus(Id::Tree)),
                // These didn't work, should allow to customize key bindings
                // Event::Keyboard(
                //     key @ KeyEvent {
                //         code: Key::PageUp | Key::PageDown,
                //         kind: KeyEventKind::Press,
                //         modifiers,
                //     },
                // ) if (modifiers == cntrl_shift) => {
                //     return match key.code {
                //         Key::PageUp => {
                //             Some(Msg::MoveTabLeft(self.editor_id.clone()))
                //         }
                //         Key::PageDown => Some(Msg::MoveTabRight(self.editor_id.clone())),
                //         _ => None,
                //     };
                // }
                // Event::Keyboard(
                //     key @ KeyEvent {
                //         code: Key::Char('[') | Key::Char(']'),
                //         kind: KeyEventKind::Press,
                //         modifiers,
                //     },
                // ) if (modifiers == alt_control) => {
                //     return match key.code {
                //         Key::Char('[') => Some(Msg::MoveTabLeft(self.editor_id.clone())),
                //         Key::Char(']') => Some(Msg::MoveTabRight(self.editor_id.clone())),
                //         _ => None,
                //     };
                // }
                // Event::Keyboard(KeyEvent {
                //     code: Key::PageUp,
                //     kind: KeyEventKind::Press,
                //     modifiers: KeyModifiers::CONTROL,
                // }) => Some(Msg::PreviousEditor),
                // Event::Keyboard(KeyEvent {
                //     code: Key::PageDown,
                //     kind: KeyEventKind::Press,
                //     modifiers: KeyModifiers::CONTROL,
                // }) => Some(Msg::NextEditor),
                // Event::Keyboard(KeyEvent {
                //     code: Key::Enter,
                //     kind: KeyEventKind::Press,
                //     modifiers,
                // }) if (modifiers == alt_control) => Some(self.execute_message()),
                // Event::Keyboard(KeyEvent {
                //     code: Key::Char('e'),
                //     kind: KeyEventKind::Press,
                //     modifiers: KeyModifiers::CONTROL,
                // }) => Some(self.execute_message()),
                // Event::Keyboard(KeyEvent {
                //     code: Key::Char('r'),
                //     kind: KeyEventKind::Press,
                //     modifiers: KeyModifiers::CONTROL,
                // }) => Some(self.execute_message()),
                // Event::Keyboard(KeyEvent {
                //     code: Key::Esc,
                //     kind: KeyEventKind::Press,
                //     ..
                // }) => Some(Msg::AppClose),
                Event::Keyboard(KeyEvent {
                    code: Key::Backspace,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    self.perform(Cmd::Delete);
                    Some(Msg::None)
                }
                // Event::Keyboard(KeyEvent {
                //     code: Key::Backspace,
                //     kind: KeyEventKind::Press,
                //     modifiers: KeyModifiers::CONTROL,
                // }) => {
                //     self.perform(Cmd::Custom(TEXTAREA_CMD_DEL_WORD));
                //     Some(Msg::None)
                // }
                // | Event::Keyboard(KeyEvent {
                //     code: Key::Char('h'),
                //     modifiers: KeyModifiers::CONTROL,
                //     kind: KeyEventKind::Press,
                // }) => {
                //     self.perform(Cmd::Delete);
                //     Some(Msg::None)
                // }
                Event::Keyboard(KeyEvent {
                    code: Key::Delete,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    self.perform(Cmd::Cancel);
                    Some(Msg::None)
                }
                // Event::Keyboard(KeyEvent {
                //     code: Key::Delete,
                //     kind: KeyEventKind::Press,
                //     modifiers: KeyModifiers::CONTROL,
                // }) => {
                //     self.perform(Cmd::Custom(TEXTAREA_CMD_DEL_NEXT_WORD));
                //     Some(Msg::None)
                // }
                Event::Keyboard(KeyEvent {
                    code: Key::PageDown,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                })
                | Event::Keyboard(KeyEvent {
                    code: Key::Down,
                    modifiers: KeyModifiers::SHIFT,
                    kind: KeyEventKind::Press,
                }) => {
                    self.perform(Cmd::Scroll(Direction::Down));
                    Some(Msg::None)
                }
                Event::Keyboard(KeyEvent {
                    code: Key::PageUp,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                })
                | Event::Keyboard(KeyEvent {
                    code: Key::Up,
                    modifiers: KeyModifiers::SHIFT,
                    kind: KeyEventKind::Press,
                }) => {
                    self.perform(Cmd::Scroll(Direction::Up));
                    Some(Msg::None)
                }
                Event::Keyboard(KeyEvent {
                    code: Key::Down,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    self.perform(Cmd::Move(Direction::Down));
                    Some(Msg::None)
                }
                Event::Keyboard(KeyEvent {
                    code: Key::Left,
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                }) => {
                    self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_BACK));
                    Some(Msg::None)
                }
                Event::Keyboard(KeyEvent {
                    code: Key::Left,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    self.perform(Cmd::Move(Direction::Left));
                    Some(Msg::None)
                }
                Event::Keyboard(KeyEvent {
                    code: Key::Right,
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                }) => {
                    self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_FORWARD));
                    Some(Msg::None)
                }
                Event::Keyboard(KeyEvent {
                    code: Key::Right,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    self.perform(Cmd::Move(Direction::Right));
                    Some(Msg::None)
                }
                Event::Keyboard(KeyEvent {
                    code: Key::Up,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    self.perform(Cmd::Move(Direction::Up));
                    Some(Msg::None)
                }
                // Event::Keyboard(KeyEvent {
                //     code: Key::Home,
                //     kind: KeyEventKind::Press,
                //     modifiers: KeyModifiers::CONTROL,
                // }) => {
                //     self.perform(Cmd::GoTo(Position::At(0)));
                //     Some(Msg::None)
                // }
                Event::Keyboard(KeyEvent {
                    code: Key::End,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    self.perform(Cmd::GoTo(Position::End));
                    Some(Msg::None)
                }
                Event::Keyboard(KeyEvent {
                    code: Key::Enter,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    self.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE));
                    Some(Msg::None)
                }
                Event::Keyboard(KeyEvent {
                    code: Key::Home,
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    self.perform(Cmd::GoTo(Position::Begin));
                    Some(Msg::None)
                }
                #[cfg(feature = "search")]
                Event::Keyboard(KeyEvent {
                    code: Key::Char('s'),
                    modifiers: KeyModifiers::CONTROL,
                }) => {
                    self.perform(Cmd::Custom(TEXTAREA_CMD_SEARCH_BACK));
                    Some(Msg::None)
                }
                #[cfg(feature = "search")]
                Event::Keyboard(KeyEvent {
                    code: Key::Char('d'),
                    modifiers: KeyModifiers::CONTROL,
                }) => {
                    self.perform(Cmd::Custom(TEXTAREA_CMD_SEARCH_FORWARD));
                    Some(Msg::None)
                }
                // #[cfg(feature = "clipboard")]
                // Event::Keyboard(KeyEvent {
                //     code: Key::Char('v'),
                //     kind: KeyEventKind::Press,
                //     modifiers: KeyModifiers::CONTROL,
                // }) => {
                //     self.perform(Cmd::Custom(TEXTAREA_CMD_PASTE));
                //     Some(Msg::None)
                // }
                Event::Keyboard(KeyEvent {
                    code: Key::Char('z'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                }) => {
                    self.perform(Cmd::Custom(TEXTAREA_CMD_UNDO));
                    Some(Msg::None)
                }
                Event::Keyboard(KeyEvent {
                    code: Key::Char('y'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                }) => {
                    self.perform(Cmd::Custom(TEXTAREA_CMD_REDO));
                    Some(Msg::None)
                }
                Event::Keyboard(KeyEvent {
                    code: Key::Tab,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    self.perform(Cmd::Type('\t'));
                    Some(Msg::None)
                }
                Event::Keyboard(KeyEvent {
                    code: Key::Char(ch),
                    kind: KeyEventKind::Press,
                    modifiers,
                    ..
                }) if !ch.is_alphabetic()
                    || modifiers == KeyModifiers::NONE
                    || modifiers == KeyModifiers::SHIFT =>
                {
                    // either a non-alphabetic char or an alphabetic char without modifiers to allow
                    // for global key bindings on alphabetic chars
                    self.perform(Cmd::Type(ch));
                    Some(Msg::None)
                }
                // Event::Keyboard(KeyEvent {
                //     code: Key::Function(2),

                //     kind: KeyEventKind::Press,
                //     ..
                // }) => Some(Msg::ChangeFocus(Id::Label)),
                #[cfg(feature = "search")]
                Event::Keyboard(KeyEvent {
                    code: Key::Function(3),

                    kind: KeyEventKind::Press,
                    ..
                }) => Some(Msg::ChangeFocus(Id::Search)),
                _ => None,
            }
        })
    }
}

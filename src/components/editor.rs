use itertools::Itertools;
use std::ops::BitOr;
#[cfg(feature = "search")]
use tuirealm::StateValue;
use tuirealm::{
    application::PollStrategy,
    command::{Cmd, CmdResult, Direction, Position},
    event::{Event, Key, KeyEvent, KeyEventKind, KeyModifiers},
    props::{
        Alignment, AttrValue, Attribute, BorderSides, BorderType, Borders, Color, Style,
        TextModifiers,
    },
    terminal::TerminalBridge,
    Application, Component, EventListenerCfg, MockComponent, NoUserEvent, State, StateValue,
    Update,
};
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};
// label
#[cfg(feature = "search")]
use tui_realm_stdlib::Input;
use tui_realm_stdlib::Label;
// textarea
#[cfg(feature = "clipboard")]
use tui_realm_textarea::TEXTAREA_CMD_PASTE;
use tui_realm_textarea::{
    TextArea, TEXTAREA_CMD_MOVE_WORD_BACK, TEXTAREA_CMD_MOVE_WORD_FORWARD, TEXTAREA_CMD_NEWLINE,
    TEXTAREA_CMD_REDO, TEXTAREA_CMD_UNDO,
};
#[cfg(feature = "search")]
use tui_realm_textarea::{
    TEXTAREA_CMD_SEARCH_BACK, TEXTAREA_CMD_SEARCH_FORWARD, TEXTAREA_SEARCH_PATTERN,
};

use crate::{
    app::{EditorId, TisqEvent},
    Id, Msg,
};

pub struct Editor<'a> {
    component: TextArea<'a>,
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
    pub(crate) fn new(editor_id: EditorId) -> Self {
        let textarea = TextArea::default();
        Self {
            editor_id,
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
        )
    }
}

impl<'a> Component<Msg, TisqEvent> for Editor<'a> {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        let alt_control: KeyModifiers = KeyModifiers::ALT.bitor(KeyModifiers::CONTROL);
        let cntrl_shift: KeyModifiers = KeyModifiers::SHIFT.bitor(KeyModifiers::CONTROL);
        // log::debug!("matching {:?}", ev);
        match ev {
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
            Event::Keyboard(
                key @ KeyEvent {
                    code: Key::Char('[') | Key::Char(']'),
                    kind: KeyEventKind::Press,
                    modifiers,
                },
            ) if (modifiers == alt_control) => {
                return match key.code {
                    Key::Char('[') => Some(Msg::MoveTabLeft(self.editor_id.clone())),
                    Key::Char(']') => Some(Msg::MoveTabRight(self.editor_id.clone())),
                    _ => None,
                };
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageUp,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::PreviousEditor),
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::NextEditor),
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                kind: KeyEventKind::Press,
                modifiers,
            }) if (modifiers == alt_control) => Some(self.execute_message()),
            Event::Keyboard(KeyEvent {
                code: Key::Char('e'),
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::CONTROL,
            }) => Some(self.execute_message()),
            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::CONTROL,
            }) => Some(self.execute_message()),
            // Event::Keyboard(KeyEvent {
            //     code: Key::Esc,
            //     kind: KeyEventKind::Press,
            //     ..
            // }) => Some(Msg::AppClose),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                kind: KeyEventKind::Press,
                ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete,
                kind: KeyEventKind::Press,
                ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                kind: KeyEventKind::Press,
                ..
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
                ..
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
                ..
            }) => {
                self.perform(Cmd::Move(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
            }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_BACK));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                kind: KeyEventKind::Press,
                ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
            }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_FORWARD));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                kind: KeyEventKind::Press,
                ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                kind: KeyEventKind::Press,
                ..
            }) => {
                self.perform(Cmd::Move(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::End,
                kind: KeyEventKind::Press,
                ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('e'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
            }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('m'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
            }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home,
                kind: KeyEventKind::Press,
                ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
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
            #[cfg(feature = "clipboard")]
            Event::Keyboard(KeyEvent {
                code: Key::Char('v'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_PASTE));
                Some(Msg::None)
            }
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
                ..
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Function(2),

                kind: KeyEventKind::Press,
                ..
            }) => Some(Msg::ChangeFocus(Id::Label)),

            #[cfg(feature = "search")]
            Event::Keyboard(KeyEvent {
                code: Key::Function(3),

                kind: KeyEventKind::Press,
                ..
            }) => Some(Msg::ChangeFocus(Id::Search)),
            _ => None,
        }
    }
}

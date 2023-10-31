use core::fmt;
use std::{collections::HashMap, hash::Hash, vec};

use itertools::Itertools;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use tuirealm::{
    event::{Key, KeyEvent, KeyEventKind, KeyModifiers},
    AttrValue, Attribute, State, Sub, SubClause, SubEventClause,
};

use crate::{config::TisqConfig, Id};

use super::{KeybindingKeyPress, KeyboundAction, TisqEvent};

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) enum TisqKeyboundAction {
    GlobalExit,
    GlobalCycleNavigation,
    GlobalNavigateLeft,
    GlobalNavigateRight,
    GlobalNavigateUp,
    GlobalNavigateDown,

    EditorNextTab,
    EditorPrevTab,
    EditorMoveTabLeft,
    EditorMoveTabRight,
    EditorExecute,
    EditorPaste,
    EditorDeleteWord,
    EditorDeleteNextWord,
    EditorMoveToTop,
    EditorMoveToBottom,
    EditorCloseTab,
    EditorTryExpand,
    EditorToggleComment,

    BrowserAddServer,
    BrowserDatabaseOpenQueryEditor,

    ResultOffsetColumnRight,
    ResultOffsetColumnLeft,
}

pub(crate) const GLOBAL_SECTION: &str = "globals";
pub(crate) const EDITOR_SECTION: &str = "editor";
pub(crate) const BROWSER_SECTION: &str = "browser";
pub(crate) const QUERY_RESULT_SECTION: &str = "result";

impl KeyboundAction for TisqKeyboundAction {
    fn sections() -> Vec<&'static str> {
        vec![GLOBAL_SECTION, EDITOR_SECTION, BROWSER_SECTION, QUERY_RESULT_SECTION]
    }

    fn list(section: &str) -> Vec<&TisqKeyboundAction> {
        match section {
            GLOBAL_SECTION => vec![
                &TisqKeyboundAction::GlobalExit,
                &TisqKeyboundAction::GlobalCycleNavigation,
                &TisqKeyboundAction::GlobalNavigateLeft,
                &TisqKeyboundAction::GlobalNavigateRight,
                &TisqKeyboundAction::GlobalNavigateUp,
                &TisqKeyboundAction::GlobalNavigateDown,
            ],
            EDITOR_SECTION => vec![
                &TisqKeyboundAction::EditorNextTab,
                &TisqKeyboundAction::EditorPrevTab,
                &TisqKeyboundAction::EditorMoveTabLeft,
                &TisqKeyboundAction::EditorMoveTabRight,
                // &TisqKeyboundAction::EditorBackspace,
                &TisqKeyboundAction::EditorExecute,
                &TisqKeyboundAction::EditorPaste,
                &TisqKeyboundAction::EditorDeleteWord,
                &TisqKeyboundAction::EditorDeleteNextWord,
                &TisqKeyboundAction::EditorMoveToTop,
                &TisqKeyboundAction::EditorMoveToBottom,
                &TisqKeyboundAction::EditorCloseTab,
                &TisqKeyboundAction::EditorTryExpand,
                &TisqKeyboundAction::EditorToggleComment,
            ],
            BROWSER_SECTION => vec![
                &TisqKeyboundAction::BrowserAddServer,
                &TisqKeyboundAction::BrowserDatabaseOpenQueryEditor,
            ],
            QUERY_RESULT_SECTION => vec![
                &TisqKeyboundAction::ResultOffsetColumnLeft,
                &TisqKeyboundAction::ResultOffsetColumnRight,
            ],
            _ => vec![],
        }
    }

    fn get_default_bindings(&self) -> Vec<KeybindingKeyPress> {
        match self {
            &TisqKeyboundAction::GlobalExit => {
                vec![
                    KeybindingKeyPress {
                        key: Key::Esc,
                        modifiers: KeyModifiers::NONE,
                    },
                    KeybindingKeyPress {
                        key: Key::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                    },
                ]
            }

            &TisqKeyboundAction::GlobalNavigateDown => {
                vec![
                    KeybindingKeyPress {
                        key: Key::Down,
                        modifiers: KeyModifiers::ALT,
                    },
                    KeybindingKeyPress {
                        key: Key::Down,
                        modifiers: KeyModifiers::CONTROL | KeyModifiers::ALT,
                    },
                ]
            }

            &TisqKeyboundAction::GlobalNavigateUp => {
                vec![
                    KeybindingKeyPress {
                        key: Key::Up,
                        modifiers: KeyModifiers::ALT,
                    },
                    KeybindingKeyPress {
                        key: Key::Up,
                        modifiers: KeyModifiers::CONTROL | KeyModifiers::ALT,
                    },
                ]
            }

            &TisqKeyboundAction::GlobalNavigateLeft => {
                vec![
                    KeybindingKeyPress {
                        key: Key::Left,
                        modifiers: KeyModifiers::ALT,
                    },
                    KeybindingKeyPress {
                        key: Key::Left,
                        modifiers: KeyModifiers::CONTROL | KeyModifiers::ALT,
                    },
                ]
            }

            &TisqKeyboundAction::GlobalNavigateRight => {
                vec![
                    KeybindingKeyPress {
                        key: Key::Right,
                        modifiers: KeyModifiers::ALT,
                    },
                    KeybindingKeyPress {
                        key: Key::Right,
                        modifiers: KeyModifiers::CONTROL | KeyModifiers::ALT,
                    },
                ]
            }

            &TisqKeyboundAction::GlobalCycleNavigation => {
                vec![KeybindingKeyPress {
                    key: Key::Char('l'),
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            &TisqKeyboundAction::EditorNextTab => {
                vec![KeybindingKeyPress {
                    key: Key::PageUp,
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            &TisqKeyboundAction::EditorPrevTab => {
                vec![KeybindingKeyPress {
                    key: Key::PageDown,
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            &TisqKeyboundAction::EditorMoveTabLeft => {
                vec![KeybindingKeyPress {
                    key: Key::Char('['),
                    modifiers: KeyModifiers::CONTROL | KeyModifiers::ALT,
                }]
            }

            &TisqKeyboundAction::EditorMoveTabRight => {
                vec![KeybindingKeyPress {
                    key: Key::Char(']'),
                    modifiers: KeyModifiers::CONTROL | KeyModifiers::ALT,
                }]
            }

            &TisqKeyboundAction::EditorExecute => {
                vec![
                    KeybindingKeyPress {
                        key: Key::Char('e'),
                        modifiers: KeyModifiers::CONTROL,
                    },
                    KeybindingKeyPress {
                        key: Key::Char('r'),
                        modifiers: KeyModifiers::CONTROL,
                    },
                    KeybindingKeyPress {
                        key: Key::Enter,
                        modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
                    },
                ]
            }

            &TisqKeyboundAction::EditorPaste => {
                vec![KeybindingKeyPress {
                    key: Key::Char('v'),
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            &TisqKeyboundAction::EditorDeleteWord => {
                vec![KeybindingKeyPress {
                    key: Key::Backspace,
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            &TisqKeyboundAction::EditorDeleteNextWord => {
                vec![KeybindingKeyPress {
                    key: Key::Delete,
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            &TisqKeyboundAction::EditorMoveToTop => {
                vec![KeybindingKeyPress {
                    key: Key::Home,
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            &TisqKeyboundAction::EditorMoveToBottom => {
                vec![KeybindingKeyPress {
                    key: Key::End,
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            &TisqKeyboundAction::EditorCloseTab => {
                vec![KeybindingKeyPress {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            &TisqKeyboundAction::EditorTryExpand => {
                vec![KeybindingKeyPress {
                    key: Key::Char(' '),
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            &TisqKeyboundAction::EditorToggleComment => {
                vec![KeybindingKeyPress {
                    key: Key::Char('/'),
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            &TisqKeyboundAction::BrowserAddServer => {
                vec![KeybindingKeyPress {
                    key: Key::Char('a'),
                    modifiers: KeyModifiers::NONE,
                }]
            }

            &TisqKeyboundAction::BrowserDatabaseOpenQueryEditor => {
                vec![KeybindingKeyPress {
                    key: Key::Char('q'),
                    modifiers: KeyModifiers::NONE,
                }]
            }

            &TisqKeyboundAction::ResultOffsetColumnLeft => {
                vec![KeybindingKeyPress {
                    key: Key::Left,
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            &TisqKeyboundAction::ResultOffsetColumnRight => {
                vec![KeybindingKeyPress {
                    key: Key::Right,
                    modifiers: KeyModifiers::CONTROL,
                }]
            }

            _ => {
                vec![]
            }
        }
    }
}

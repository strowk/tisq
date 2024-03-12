use tui_realm_stdlib::Phantom;
use tuirealm::{Component, Event, MockComponent, Sub};

use crate::{
    app::{KeySubClause, SectionKeybindings, TisqEvent, TisqKeyboundAction},
    Id, Msg,
};

#[derive(MockComponent)]
pub struct GlobalListener {
    component: Phantom,
    keybindings: SectionKeybindings<TisqKeyboundAction>,
}

impl GlobalListener {
    pub(crate) fn new(keybindings: SectionKeybindings<TisqKeyboundAction>) -> Self {
        Self {
            component: Phantom::default(),
            keybindings,
        }
    }

    pub(crate) fn switch_location_nav(key: &TisqKeyboundAction) -> Option<Msg> {
        match key {
            &TisqKeyboundAction::GlobalNavigateLeft => Some(Msg::NavigateLeft),
            &TisqKeyboundAction::GlobalNavigateRight => Some(Msg::NavigateRight),
            &TisqKeyboundAction::GlobalNavigateUp => Some(Msg::NavigateUp),
            &TisqKeyboundAction::GlobalNavigateDown => Some(Msg::NavigateDown),
            _ => None,
        }
    }

    pub(crate) fn subscriptions(&self) -> Vec<Sub<Id, TisqEvent>> {
        self.keybindings
            .subscriptions::<Id, TisqEvent>(KeySubClause::Always)
    }
}

impl Component<Msg, TisqEvent> for GlobalListener {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(kb_event) => match self.keybindings.get_action(&kb_event) {
                Some(&TisqKeyboundAction::GlobalExit) => Some(Msg::AppClose),
                Some(
                    action @ (&TisqKeyboundAction::GlobalNavigateLeft
                    | &TisqKeyboundAction::GlobalNavigateRight
                    | &TisqKeyboundAction::GlobalNavigateUp
                    | &TisqKeyboundAction::GlobalNavigateDown),
                ) => Self::switch_location_nav(action),
                Some(&TisqKeyboundAction::GlobalCycleNavigation) => Some(Msg::CycleNavigation),
                Some(&TisqKeyboundAction::GlobalCommandMode) => Some(Msg::EnterCommandMode),
                Some(&TisqKeyboundAction::GlobalCancel) => Some(Msg::Cancel),
                _ => None,
                // None => None,
            },
            _ => None,
        }
    }
}

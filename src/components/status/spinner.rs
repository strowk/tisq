use tui_realm_stdlib::{Span, Spinner};
use tuirealm::{
    props::{Color, TextSpan},
    Component, Event, MockComponent,
};

use crate::{app::TisqEvent, tui::Msg};


#[derive(MockComponent)]
pub(crate) struct StatusSpinner {
    component: Spinner,
}

impl Default for StatusSpinner {
    fn default() -> Self {
        Self {
            component: Spinner::default()
                .foreground(Color::LightBlue)
                .sequence("⣾⣽⣻⢿⡿⣟⣯⣷"),
        }
    }
}

impl Component<Msg, TisqEvent> for StatusSpinner {
    fn on(&mut self, event: Event<TisqEvent>) -> Option<Msg> {
        match event {
            Event::User(TisqEvent::SpinnerTick) => Some(Msg::TriggerRedraw),
            _ => None,
        }
    }
}

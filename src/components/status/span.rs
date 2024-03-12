use tui_realm_stdlib::{Span, Spinner};
use tuirealm::{
    props::{Color, TextSpan},
    Component, Event, MockComponent,
};

use crate::{app::TisqEvent, tui::Msg};

#[derive(MockComponent)]
pub(crate) struct StatusSpan {
    component: Span,
}

impl Default for StatusSpan {
    fn default() -> Self {
        Self {
            component: Span::default()
                .foreground(Color::Green)
                .spans(&[TextSpan::new("Process db req... ").underlined()]),
        }
    }
}

impl Component<Msg, TisqEvent> for StatusSpan {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        Some(Msg::None)
    }
}

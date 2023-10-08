use std::io::Stdout;

use tuirealm::{
    application::ApplicationResult,
    tui::{
        prelude::{Constraint, CrosstermBackend, Direction, Layout, Rect},
        Frame,
    },
};

use crate::{app::model::TisqApplication, Id};

pub mod input_text;
mod submit;

pub use submit::FormSubmitListener;

pub(crate) struct AddServerForm {
    pub(crate) active_input: Id,
}

impl AddServerForm {
    pub(crate) fn new() -> Self {
        Self {
            active_input: Id::ServerNameInput,
        }
    }

    pub(crate) fn activate_next_input(
        &mut self,
        app: &mut TisqApplication,
    ) -> ApplicationResult<()> {
        self.active_input = match self.active_input {
            Id::ServerNameInput => Id::ConnectionUrlInput,
            Id::ConnectionUrlInput => Id::ServerNameInput,
            _ => Id::ServerNameInput,
        };
        app.active(&self.active_input)
    }

    pub(crate) fn activate_previous_input(
        &mut self,
        app: &mut TisqApplication,
    ) -> ApplicationResult<()> {
        self.active_input = match self.active_input {
            Id::ServerNameInput => Id::ConnectionUrlInput,
            Id::ConnectionUrlInput => Id::ServerNameInput,
            _ => Id::ServerNameInput,
        };
        app.active(&self.active_input)
    }

    pub(crate) fn view(
        &self,
        area: Rect,
        app: &mut TisqApplication,
        f: &mut Frame<CrosstermBackend<Stdout>>,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0), // fills remaining space
            ])
            .margin(1)
            .split(area);

        app.view(&Id::ServerNameInput, f, chunks[0]);
        app.view(&Id::ConnectionUrlInput, f, chunks[1]);
    }
}

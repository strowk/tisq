use tuirealm::{
    tui::prelude::{Constraint, Direction, Layout, Rect},
    Application, Frame, Sub, SubClause, SubEventClause,
};

use crate::{
    app::{DbResponse, TisqEvent},
    components::{StatusSpan, StatusSpinner},
    tui::{Id, Msg},
};

use super::{
    model::TisqApplication,
    spinner_ticking_port::{self, SpinnerTickingPort},
    DbRequest,
};

pub(crate) struct AppStatus {
    requests_in_progress: i32,
    doing: String,
    pub(crate) query_processing: bool,
}

impl Default for AppStatus {
    fn default() -> Self {
        Self {
            requests_in_progress: 0,
            doing: String::new(),
            query_processing: false,
        }
    }
}

impl AppStatus {
    pub(super) fn push_db_request(
        &mut self,
        db_request: &DbRequest,
        spinner_ticking_port: &mut SpinnerTickingPort,
    ) {
        self.requests_in_progress += 1;
        self.query_processing = true;
        spinner_ticking_port.set_ticking(true);
    }

    pub(super) fn pop_db_request(&mut self, spinner_ticking_port: &mut SpinnerTickingPort) {
        self.requests_in_progress -= 1;
        if self.requests_in_progress < 0 {
            tracing::error!("requests_in_progress < 0, should not be possible");
            self.requests_in_progress = 0
        }
        if self.requests_in_progress == 0 {
            self.query_processing = false;
        }
        // tracing::debug!("requests_in_progress: {}", self.requests_in_progress);
        // tracing::debug!("set ticking: {}", self.query_processing);
        spinner_ticking_port.set_ticking(self.query_processing);
    }

    pub(super) fn view(&self, rect: Rect, f: &mut Frame, app: &mut TisqApplication) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Min(0), // fills remaining space
            ])
            .split(rect);

        if self.query_processing {
            app.view(&Id::StatusSpinner, f, chunks[0]);
            app.view(&Id::StatusSpan, f, chunks[1]);
        }
    }

    pub(super) fn mount(app: &mut TisqApplication) {
        assert!(app
            .mount(
                Id::StatusSpinner,
                Box::new(StatusSpinner::default()),
                vec![
                    Sub::new(
                        SubEventClause::User(TisqEvent::SpinnerTick),
                        SubClause::IsMounted(Id::StatusSpinner)
                    ),
                    // Sub::new(
                    //     SubEventClause::User(TisqEvent::DbResponse(DbResponse::None)),
                    //     // due to comparison of TisqEvent, it does not matter which DbResponse is used
                    //     SubClause::Always
                    // )
                ]
            )
            .is_ok());

        assert!(app
            .mount(Id::StatusSpan, Box::new(StatusSpan::default()), vec![])
            .is_ok());
    }
}

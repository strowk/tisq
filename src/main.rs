//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

extern crate tuirealm;

use app::EditorId;
use tuirealm::application::PollStrategy;

use tuirealm::{AttrValue, Attribute, Update};
// -- internal
mod app;
mod components;
use app::model::Model;
use uuid::Uuid;

// Let's define the messages handled by our app. NOTE: it must derive `PartialEq`
#[derive(Debug, PartialEq)]
pub(crate) enum Msg {
    AppClose,

    ExecuteQuery(EditorId, String),
    ChangeFocus(Id),

    PreviousEditor,
    NextEditor,
    MoveTabLeft(EditorId),
    MoveTabRight(EditorId),

    StartAddingServer,
    FocusPreviousInput,
    FocusNextInput,
    SubmitAddServerForm,

    DeleteBrowsedNode(String),

    OpenConnection(Uuid),
    LoadDatabases(Uuid),

    OpenQueryEditor(Uuid, String),

    ShowFetchedTable,
    ShowErrorResult,

    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Clock,
    DigitCounter,
    LetterCounter,
    Label,
    EditorPanel,
    Editor(EditorId),
    Tree,
    GlobalListener,

    EditorTabs,
    QueryResultTable,

    AddServerForm,
    ServerNameInput,
    ConnectionUrlInput,
    FormSubmitListener,

    ExecuteErrorResult,
}

fn main() -> eyre::Result<()> {
    use log::LevelFilter;

    simple_logging::log_to_file("debug.log", LevelFilter::Debug)?;

    // Setup model
    let mut model = Model::default();
    // Enter alternate screen
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();
    // Main loop
    // NOTE: loop until quit; quit is set in update if AppClose is received from counter
    while !model.quit {
        // Tick
        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                assert!(model
                    .app
                    .attr(
                        &Id::Label,
                        Attribute::Text,
                        AttrValue::String(format!("Application error: {}", err)),
                    )
                    .is_ok());
            }
            Ok(messages) if messages.len() > 0 => {
                // NOTE: redraw if at least one msg has been processed
                model.redraw = true;
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            _ => {}
        }
        // Redraw
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }
    // Terminate terminal
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
    Ok(())
}

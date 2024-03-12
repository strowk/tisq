extern crate tuirealm;

use crate::app;
use crate::statics::*;
use app::{DbRequest, EditorId};

use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};
use tuirealm::application::PollStrategy;

use tuirealm::{AttrValue, Attribute, Update};

use app::model::Model;
use uuid::Uuid;

use crate::config::TisqConfig;

// Let's define the messages handled by our app. NOTE: it must derive `PartialEq`
#[derive(Debug, PartialEq)]
pub(crate) enum Msg {
    AppClose,

    ExecuteQuery(EditorId, String, i32),
    // ReconnectAndExecuteQuery(EditorId, String),
    ReconnectAndRepeat(DbRequest),
    ChangeFocus(Id),

    NavigateRight,
    NavigateLeft,
    NavigateUp,
    NavigateDown,

    Cancel,

    PreviousEditor,
    NextEditor,
    MoveTabLeft(EditorId),
    MoveTabRight(EditorId),
    CloseTab(EditorId),

    CycleNavigation,

    StartAddingServer,
    FocusPreviousInput,
    FocusNextInput,
    SubmitAddServerForm,

    DeleteBrowsedNode(String),

    OpenDatabase(Uuid, String, i32),
    OpenSchema {
        server_id: Uuid,
        database: String,
        schema: String,
        retries: i32,
    },
    OpenTable {
        server_id: Uuid,
        database: String,
        schema: String,
        table: String,
        retries: i32,
    },
    OpenConnection(Uuid),
    LoadDatabases(Uuid),

    OpenQueryEditor(Uuid, String),

    ShowFetchedTable,
    ShowErrorResult,

    ApplySnippet(String),
    ShowSnippets,
    EditorTryExpand {
        editor_id: EditorId,
        text: String,
        remove_input: bool,
    },

    TriggerRedraw,

    PopDbRequestStatus,

    EnterCommandMode,
    ExecuteCommand(String),

    // settings
    SetEnabledLastEnteredKey(bool), // used to show/hide last entered key in status bar

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
    SnippetsTable,

    EditorTabs,
    QueryResultTable,

    AddServerForm,
    ServerNameInput,
    ConnectionUrlInput,
    FormSubmitListener,

    ShowUsedKeyToggle,

    ExecuteErrorResult,

    DbResponseStatusListener,
    StatusSpinner,
    StatusSpan,
    StatusPressedKey,

    CommandLine,
}

pub(crate) fn run(debug_logs: bool) -> eyre::Result<()> {
    if debug_logs {
        DEBUG_LOG.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    let tracing_to_file = tracing_subscriber::registry().with(
        fmt::layer()
            .with_ansi(false)
            .with_writer(|| LOG_FILE.as_ref().expect("log file not initialized!")),
    );

    if DEBUG_LOG.load(std::sync::atomic::Ordering::Relaxed) {
        tracing_to_file.with(EnvFilter::from_default_env().add_directive("tisq=debug".parse()?))
    } else {
        // default level would be Error
        tracing_to_file.with(EnvFilter::from_default_env())
    }
    .init();

    let files_root = match FILES_ROOT.as_ref() {
        Ok(root) => root,
        Err(err) => return Err(eyre::eyre!("Failed to open tisq root directory: {}", err)),
    };

    let config = TisqConfig::read_or_create(files_root)?;

    // Setup model
    let mut model = Model::new(files_root, config);

    // Enter alternate screen
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();
    let mut quit_message = "".to_string();
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

        let fatal_quit = QUIT_CHANNEL
            .lock()
            .map(|message| Some(message))
            .unwrap_or(None)
            .and_then(|it| it.1.try_iter().next());

        if let Some(message) = fatal_quit {
            // quitting with message
            model.quit = true;
            quit_message = message;
        } else {
            // Redraw
            if model.redraw {
                model.view();
                model.redraw = false;
            }
        }
    }

    // Terminate terminal
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();

    if quit_message.len() > 0 {
        println!("{}", quit_message);
    }
    Ok(())
}

extern crate tuirealm;

use std::env;
use std::fs::File;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;

use app::EditorId;
use once_cell::sync::Lazy;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};
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

    NavigateRight,
    NavigateLeft,
    NavigateUp,
    NavigateDown,

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

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

static QUIT_CHANNEL: Lazy<Mutex<(Sender<String>, Receiver<String>)>> =
    Lazy::new(|| Mutex::new(mpsc::channel()));

static DEBUG_LOG: AtomicBool = AtomicBool::new(false);

fn main() -> eyre::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        if args[1] == "--version" {
            println!("tisq v{}", VERSION);
            return Ok(());
        }

        if args[1] == "--debug" {
            DEBUG_LOG.store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }

    static LOG_FILE: Lazy<Option<File>> = Lazy::new(|| {
        File::create(if DEBUG_LOG.load(std::sync::atomic::Ordering::Relaxed) {
            "tisq-debug.log"
        } else {
            "tisq-errors.log"
        })
        .map_err(|_| {
            QUIT_CHANNEL
                .lock()
                .unwrap()
                .0
                .send("Failed to create log file `tisq.log` - exited abnormally.".to_string())
                .unwrap();
        })
        .ok()
    });

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

    // Setup model
    let mut model = Model::default();

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

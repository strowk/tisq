extern crate tuirealm;

use std::env;

mod app;
mod components;
mod config;
mod files;
mod tui;

use once_cell::sync::Lazy;
pub(crate) use tui::Id;
pub(crate) use tui::Msg;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

use argh::FromArgs;

#[derive(FromArgs)]
/// tisq - a TUI for SQL
struct CliArgs {
    /// print version and exit
    #[argh(switch)]
    version: bool,

    /// enable debug logging
    #[argh(switch)]
    debug: bool,

    #[argh(subcommand)]
    nested: Option<Subcommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    AddServer(AddServerArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Add a server to storage
#[argh(subcommand, name = "add-server")]
pub(crate) struct AddServerArgs {
    #[argh(positional)]
    /// name of the server
    name: String,

    #[argh(positional)]
    /// server connection string
    connection_url: String,
}

fn main() -> eyre::Result<()> {
    let args: CliArgs = argh::from_env();

    if args.version {
        println!("tisq v{}", VERSION);
        return Ok(());
    }

    if let Some(Subcommands::AddServer(add_server_args)) = args.nested {
        let files = files::open_tisq_root()?;
        return app::cmd::add_server::run(add_server_args, &files);
    }

    tui::run(args.debug)
}

use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;

static QUIT_CHANNEL: Lazy<Mutex<(Sender<String>, Receiver<String>)>> =
    Lazy::new(|| Mutex::new(mpsc::channel()));

static DEBUG_LOG: AtomicBool = AtomicBool::new(false);

static FILES_ROOT: Lazy<eyre::Result<PathBuf>> = Lazy::new(|| files::open_tisq_root());

static LOG_FILE: Lazy<Option<File>> = Lazy::new(|| {
    File::create(if DEBUG_LOG.load(std::sync::atomic::Ordering::Relaxed) {
        FILES_ROOT.as_ref().unwrap().join("tisq-debug.log")
    } else {
        FILES_ROOT.as_ref().unwrap().join("tisq-errors.log")
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

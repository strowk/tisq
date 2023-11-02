extern crate tuirealm;

use std::env;

mod app;
mod components;
mod config;
mod files;
mod tui;

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
}

fn main() -> eyre::Result<()> {
    let args: CliArgs = argh::from_env();

    if args.version {
        println!("tisq v{}", VERSION);
        return Ok(());
    }

    tui::run(args.debug)
}

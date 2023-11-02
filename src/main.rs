extern crate tuirealm;

use std::env;

mod app;
mod cli;
mod cmd;
mod components;
mod config;
mod files;
mod statics;
mod tui;

use cli::CliArgs;
use cli::Subcommands;

pub(crate) use tui::Id;
pub(crate) use tui::Msg;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> eyre::Result<()> {
    let args: CliArgs = argh::from_env();

    if args.version {
        println!("tisq v{}", VERSION);
        return Ok(());
    }

    if let Some(Subcommands::AddServer(add_server_args)) = args.subcommands {
        let files = files::open_tisq_root()?;
        return app::cmd::add_server::run(add_server_args, &files);
    }

    tui::run(args.debug)
}

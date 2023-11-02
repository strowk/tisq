use argh::FromArgs;

use crate::app::cmd::add_server::AddServerArgs;

#[derive(FromArgs)]
/// tisq - a TUI for SQL
pub(crate) struct CliArgs {
    /// print version and exit
    #[argh(switch)]
    pub version: bool,

    /// enable debug logging
    #[argh(switch)]
    pub debug: bool,

    #[argh(subcommand)]
    pub subcommands: Option<Subcommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub(crate) enum Subcommands {
    AddServer(AddServerArgs),
}

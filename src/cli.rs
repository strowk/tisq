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
    Servers(Servers),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Subcommand to manage servers
#[argh(subcommand, name = "server")]
pub(crate) struct Servers {
    #[argh(subcommand)]
    pub subcommands: ServerSubcommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub(crate) enum ServerSubcommands {
    AddServer(AddServerArgs),
}

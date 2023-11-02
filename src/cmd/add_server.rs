use std::path::PathBuf;

use argh::FromArgs;

use crate::app::storage::{NewServer, Storage};

pub(crate) fn run(add_server_args: AddServerArgs, files_root: &PathBuf) -> eyre::Result<()> {
    let mut storage = Storage::open(files_root)?;
    let server = NewServer {
        name: add_server_args.name,
        connection_properties: {
            let mut map = std::collections::HashMap::new();
            map.insert("url".to_string(), add_server_args.connection_url);
            map
        },
    };
    storage.add_server(server)?;
    Ok(())
}

#[derive(FromArgs, PartialEq, Debug)]
/// Add a server to storage
#[argh(subcommand, name = "add")]
pub(crate) struct AddServerArgs {
    #[argh(positional)]
    /// name of the server
    name: String,

    #[argh(positional)]
    /// server connection string
    connection_url: String,
}

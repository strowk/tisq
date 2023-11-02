use std::path::PathBuf;

use crate::{AddServerArgs, app::storage::{Storage, NewServer}};

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
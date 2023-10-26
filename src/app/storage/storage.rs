use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use super::id::Id;
use kv::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) struct Storage {
    pub(super) store: Store,
}

const SERVERS_BUCKET: &str = "servers";

#[derive(Serialize, Deserialize)]
pub(crate) struct StoredServer {
    pub id: Uuid,
    pub name: String,
    pub connection_properties: HashMap<String, String>,
}

pub(crate) struct NewServer {
    pub name: String,
    pub connection_properties: HashMap<String, String>,
}

impl Storage {
    pub fn add_server(&mut self, server: NewServer) -> eyre::Result<()> {
        let bucket = Self::get_servers_bucket(&self.store)?;
        let id = Uuid::new_v4();
        let server = StoredServer {
            id,
            name: server.name,
            connection_properties: server.connection_properties,
        };
        bucket.set(&Id(id), &Json(server))?;
        // bucket.set()
        Ok(())
    }

    pub fn delete_server(&mut self, id: Uuid) -> eyre::Result<()> {
        let bucket = Self::get_servers_bucket(&self.store)?;
        bucket.remove(&Id(id))?;
        Ok(())
    }

    pub fn get_server(&self, id: Uuid) -> eyre::Result<Option<StoredServer>> {
        let bucket = Self::get_servers_bucket(&self.store)?;
        let server = bucket.get(&Id(id))?;
        let server = match server {
            Some(Json(server)) => Some(server),
            _ => None,
        };
        Ok(server)
    }

    pub fn read_servers(&self) -> eyre::Result<Vec<StoredServer>> {
        // pub fn read_servers(bucket: Bucket<Id, Json<StoredServer>>) -> eyre::Result<Vec<StoredServer>> {
        let servers: eyre::Result<Vec<StoredServer>> = Self::get_servers_bucket(&self.store)?
            .iter()
            .map(|item| {
                let Json(server): Json<StoredServer> = item?.value()?;
                Ok(server)
            })
            .collect();
        servers
    }

    fn get_servers_bucket<'a>(store: &Store) -> eyre::Result<Bucket<'a, Id, Json<StoredServer>>> {
        let bucket = store.bucket(Some(SERVERS_BUCKET))?;
        Ok(bucket)
    }

    pub fn open(files_root: &PathBuf) -> eyre::Result<Storage> {
        let storage_path = files_root.join("storage");

        // // create folder if it doesn't exist
        if !storage_path.exists() {
            fs::create_dir_all(&storage_path)?;
        }

        let cfg = Config::new(storage_path);
        let store = Store::new(cfg)?;

        // let servers_bucket = Self::get_servers_bucket(&store)?;
        // let servers = Self::read_servers(servers_bucket)?;
        // for bucket in servers_bucket.iter() {
        //     let Json(server): Json<StoredServer> = bucket?.value()?;

        // }
        // servers_bucket.get("")

        Ok(Storage { store })
    }

    // fn (&self) {}
}

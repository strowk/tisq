use std::{collections::HashMap, fs, path::PathBuf};

use super::id::Id;
use kv::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) struct Storage {
    files_root: PathBuf,
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
        let store = self.get_store()?;
        let bucket = Self::get_servers_bucket(&store)?;
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
        let store = self.get_store()?;
        let bucket = Self::get_servers_bucket(&store)?;
        bucket.remove(&Id(id))?;
        Ok(())
    }

    pub fn get_server(&self, id: Uuid) -> eyre::Result<Option<StoredServer>> {
        let store = self.get_store()?;
        let bucket = Self::get_servers_bucket(&store)?;
        let server = bucket.get(&Id(id))?;
        let server = match server {
            Some(Json(server)) => Some(server),
            _ => None,
        };
        Ok(server)
    }

    pub fn read_servers(&self) -> eyre::Result<Vec<StoredServer>> {
        let store = self.get_store()?;
        // pub fn read_servers(bucket: Bucket<Id, Json<StoredServer>>) -> eyre::Result<Vec<StoredServer>> {
        let servers: eyre::Result<Vec<StoredServer>> = Self::get_servers_bucket(&store)?
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

    pub fn set_enabled_showing_pressed_key(&mut self, enabled: bool) -> eyre::Result<()> {
        let store = self.get_store()?;
        let bucket = store.bucket(Some("settings"))?;
        let key = "show_pressed_key".to_string();
        bucket.set(&key, &Json(enabled))?;
        Ok(())
    }

    pub fn get_enabled_showing_pressed_key(&self) -> eyre::Result<bool> {
        let store = self.get_store()?;
        let bucket = store.bucket(Some("settings"))?;
        let key = "show_pressed_key".to_string();
        let enabled: Option<Json<bool>> = bucket.get(&key)?;
        let enabled = match enabled {
            Some(Json(enabled)) => enabled,
            _ => false,
        };
        Ok(enabled)
    }

    pub(crate) fn get_store(&self) -> eyre::Result<Store> {
        let storage_path = self.files_root.join("storage");
        let cfg = Config::new(storage_path);
        let store = Store::new(cfg)?;
        Ok(store)
    }

    pub fn open(files_root: &PathBuf) -> eyre::Result<Storage> {
        let storage_path = files_root.join("storage");

        // // create folder if it doesn't exist
        if !storage_path.exists() {
            fs::create_dir_all(&storage_path)?;
        }

        // let cfg = Config::new(storage_path);
        // let store = Store::new(cfg)?;

        // let servers_bucket = Self::get_servers_bucket(&store)?;
        // let servers = Self::read_servers(servers_bucket)?;
        // for bucket in servers_bucket.iter() {
        //     let Json(server): Json<StoredServer> = bucket?.value()?;

        // }
        // servers_bucket.get("")

        Ok(Storage {
            files_root: files_root.clone(),
        })
    }

    // fn (&self) {}
}

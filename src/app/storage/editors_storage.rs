use kv::{Bucket, Error, Json, Key, Raw, Store};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Storage;

const EDITORS_BUCKET: &str = "editors";

#[derive(Serialize, Deserialize)]
pub(crate) struct StoredEditor {
    pub server_id: Uuid,
    pub database: String,
    pub content: String,
}

pub(crate) struct StoredEditorId {
    pub server_id: Uuid,
    pub database: String,

    encoded: Vec<u8>,
}

impl AsRef<[u8]> for StoredEditorId {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.encoded
    }
}

impl Key<'_> for StoredEditorId {
    fn from_raw_key(r: &'_ Raw) -> Result<Self, Error> {
        let id = String::from_utf8(r.as_ref().to_vec())
            .map_err(|_| kv::Error::Message("Failed to parse stored editor id".to_owned()))?;
        let mut id = id.split(":");
        let server_id = id.next().unwrap();
        let database = id.next().unwrap();
        Ok(StoredEditorId {
            server_id: Uuid::parse_str(server_id).unwrap(),
            database: database.to_owned(),
            encoded: r.as_ref().to_vec(),
        })
    }

    fn to_raw_key(&self) -> Result<Raw, Error> {
        Ok(self.as_ref().into())
    }
}

impl Storage {
    pub fn new_editor_id(server_id: Uuid, database: String) -> StoredEditorId {
        StoredEditorId {
            server_id,
            database: database.clone(),
            encoded: format!("{}:{}", server_id, database).as_bytes().to_vec(),
        }
    }

    pub fn put_editor(&mut self, id: StoredEditorId, content: String) -> eyre::Result<()> {
        let bucket = Self::get_editors_bucket(&self.store)?;
        let editor = StoredEditor {
            server_id: id.server_id.clone(),
            database: id.database.clone(),
            content,
        };
        bucket.set(&id, &Json(editor))?;
        Ok(())
    }

    pub fn delete_editor(&mut self, id: StoredEditorId) -> eyre::Result<()> {
        let bucket = Self::get_editors_bucket(&self.store)?;
        bucket.remove(&id)?;
        Ok(())
    }

    pub fn get_editor(&self, id: StoredEditorId) -> eyre::Result<Option<StoredEditor>> {
        let bucket = Self::get_editors_bucket(&self.store)?;
        let editor = bucket.get(&id)?;
        let editor = match editor {
            Some(Json(editor)) => Some(editor),
            _ => None,
        };
        Ok(editor)
    }

    pub fn read_editors(&self) -> eyre::Result<Vec<StoredEditor>> {
        let editors: eyre::Result<Vec<StoredEditor>> = Self::get_editors_bucket(&self.store)?
            .iter()
            .map(|item| {
                let Json(editor): Json<StoredEditor> = item?.value()?;
                Ok(editor)
            })
            .collect();
        editors
    }

    fn get_editors_bucket<'a>(
        store: &Store,
    ) -> eyre::Result<Bucket<'a, StoredEditorId, Json<StoredEditor>>> {
        let bucket = store.bucket(Some(EDITORS_BUCKET))?;
        Ok(bucket)
    }
}

use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
};

use async_std::task;
use futures::TryStreamExt;
use sqlx::{
    postgres::{PgConnectOptions, PgRow, PgTypeKind},
    Column, Connection as SqlxConnection, Executor, PgConnection, Row,
};
use uuid::Uuid;


mod executing;
mod posgres;
mod types;

use executing::Executing;

pub(crate) struct Connection {
    pub(crate) name: String,
    pub(crate) url: String,

    pub(crate) internal: TypedConnection,
}

pub(crate) enum TypedConnection {
    Postgres(PgConnection),
}

const DEFAULT_MANAGEMENT_DATABASE: &str = "postgres";

impl Connection {
    pub(crate) async fn connect(name: &str, url: &str) -> Result<Self, sqlx::Error> {
        let opts: PgConnectOptions = url.parse()?;
        let opts = opts.database(&name);
        tracing::info!("Connecting to database: {:?}", opts);
        let connection = PgConnection::connect_with(&opts).await?;
        // let connection = PgConnection::connect(&url).await?;
        let connection = TypedConnection::Postgres(connection);
        Ok(Self {
            name: name.to_string(),
            url: url.to_string(),
            internal: connection,
        })
    }

    pub(crate) async fn list_databases(&mut self) -> Result<Vec<String>, sqlx::Error> {
        match &mut self.internal {
            TypedConnection::Postgres(connection) => {
                let databases = connection
                    .fetch_all(sqlx::query(
                        "SELECT datname FROM pg_database WHERE datistemplate = false;",
                    ))
                    .await?;
                let databases: Vec<String> = databases
                    .iter()
                    .map(|row| row.get::<String, usize>(0))
                    .collect();
                Ok(databases)
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub(crate) struct ConnectionKey {
    name: String,
    server_id: Uuid,
}

pub(crate) struct ConnectionsManager {
    pub(crate) tx: Sender<DbResponse>,
    pub(crate) connections: HashMap<ConnectionKey, Connection>,
}

pub(crate) enum DbRequest {
    ListDatabases(Uuid),
    ConnectToServer(Uuid, String),
    ConnectToDatabase(Uuid, String, String),
    Execute(Uuid, String, String),
}

#[derive(PartialEq, PartialOrd, Clone, Eq, Debug)]
pub(crate) enum DbResponse {
    DatabasesListed(Uuid, Vec<String>),
    Connected(Uuid),
    Executed(Uuid, Vec<String>, Vec<Vec<String>>),
    Error(Uuid, String),
    None,
}

impl ConnectionsManager {
    pub fn new(tx: Sender<DbResponse>) -> Self {
        Self {
            connections: HashMap::new(),
            tx,
        }
    }

    fn process_db_error(error: sqlx::Error, id: Uuid) -> DbResponse {
        tracing::error!("Error executing query: {:?}", error);
        match error {
            sqlx::Error::Database(db_error) => {
                return DbResponse::Error(id, db_error.message().to_string())
            }
            _ => DbResponse::Error(id, format!("unknown db error: {:?}", error)),
        }
    }

    fn process_request(&mut self, request: DbRequest) -> DbResponse {
        match request {
            DbRequest::ConnectToServer(id, url) => {
                let key = ConnectionKey {
                    name: DEFAULT_MANAGEMENT_DATABASE.to_string(),
                    server_id: id,
                };
                if self.connections.contains_key(&key) {
                    return DbResponse::Connected(id);
                }
                match task::block_on(Connection::connect(&key.name, &url)) {
                    Ok(connection) => {
                        self.connections.insert(key, connection);
                        DbResponse::Connected(id)
                    }
                    Err(e) => Self::process_db_error(e, id),
                }
            }
            DbRequest::ConnectToDatabase(id, name, url) => {
                let key = ConnectionKey {
                    name,
                    server_id: id,
                };
                if self.connections.contains_key(&key) {
                    return DbResponse::None;
                }
                match task::block_on(Connection::connect(&key.name, &url)) {
                    Ok(connection) => {
                        self.connections.insert(key, connection);
                        DbResponse::None
                    }
                    Err(e) => Self::process_db_error(e, id),
                }
            }
            DbRequest::Execute(id, name, query) => {
                match task::block_on(self.execute(query, id, name)) {
                    Ok((headers, data)) => DbResponse::Executed(id, headers, data),
                    Err(e) => Self::process_db_error(e, id),
                }
            }
            DbRequest::ListDatabases(id) => {
                if let Some(connection) = self.connections.get_mut(&ConnectionKey {
                    name: DEFAULT_MANAGEMENT_DATABASE.to_string(),
                    server_id: id,
                }) {
                    match task::block_on(connection.list_databases()) {
                        Ok(databases) => DbResponse::DatabasesListed(id, databases),
                        Err(e) => Self::process_db_error(e, id),
                    }
                } else {
                    DbResponse::Error(id, "No connection to management database".to_string())
                }
            }
        }
    }

    pub fn requests_loop(mut self, rx: Receiver<DbRequest>) {
        loop {
            let request = rx.recv();
            match request {
                Ok(request) => {
                    let response = self.process_request(request);
                    self.tx.send(response).unwrap();
                }
                Err(_e) => {
                    // TODO: enable this back when understand how to ignore shutdown sequence
                    // tracing::error!("Error receiving request from main thread: {:?}", e);
                }
            }
        }
    }

    async fn execute(
        &mut self,
        query: String,
        id: Uuid,
        name: String,
    ) -> Result<(Vec<String>, Vec<Vec<String>>), sqlx::Error> {
        tracing::info!("Executing query: {}", query);
        let key = ConnectionKey {
            name,
            server_id: id,
        };
        let connection = self.connections.get_mut(&key).unwrap();
        let connection = match &mut connection.internal {
            TypedConnection::Postgres(connection) => connection,
        };

        connection.execute_sqlx(query).await
    }
}

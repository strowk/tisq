use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
};

use async_std::task;

use sqlx::{
    postgres::{PgArguments, PgConnectOptions},
    Arguments, Column, Connection as SqlxConnection, Executor, PgConnection, Row,
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

    pub(crate) async fn list_schemas(&mut self) -> Result<Vec<String>, sqlx::Error> {
        match &mut self.internal {
            TypedConnection::Postgres(connection) => {
                let schemas = connection
                    .fetch_all(sqlx::query(
                        "SELECT schema_name FROM information_schema.schemata;",
                    ))
                    .await?;
                let schemas: Vec<String> = schemas
                    .iter()
                    .map(|row| row.get::<String, usize>(0))
                    .collect();
                Ok(schemas)
            }
        }
    }

    pub(crate) async fn list_tables(&mut self, schema: &str) -> Result<Vec<String>, sqlx::Error> {
        match &mut self.internal {
            TypedConnection::Postgres(connection) => {
                let mut args = PgArguments::default();
                args.add(schema);
                let tables = connection
                    .fetch_all(sqlx::query_with(
                        "SELECT table_name FROM information_schema.tables where table_schema = $1;",
                        args,
                    ))
                    .await?;
                let tables: Vec<String> = tables
                    .iter()
                    .map(|row| row.get::<String, usize>(0))
                    .collect();
                Ok(tables)
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

#[derive(PartialEq, PartialOrd, Clone, Eq, Debug)]
pub(crate) enum DbRequest {
    ListSchemas {
        server_id: Uuid,
        database: String,
        retries: i32,
    },
    ListTables {
        server_id: Uuid,
        database: String,
        schema: String,
        retries: i32,
    },
    ListDatabases(Uuid),
    ConnectToServer(Uuid, String),
    ConnectToDatabase(Uuid, String, String),
    Execute(Uuid, String, String, i32),
}

#[derive(PartialEq, PartialOrd, Clone, Eq, Debug)]
pub(crate) enum DownConnectionReason {
    IoError(String),
    MissingConnection,
}

#[derive(PartialEq, PartialOrd, Clone, Eq, Debug)]
pub(crate) enum DbResponse {
    DatabasesListed(Uuid, Vec<String>),
    SchemasListed {
        server_id: Uuid,
        database: String,
        schemas: Vec<String>,
    },
    TablesListed {
        server_id: Uuid,
        database: String,
        schema: String,
        tables: Vec<String>,
    },
    Connected(Uuid),
    Executed(Uuid, Vec<String>, Vec<Vec<String>>),
    Error(Uuid, String),
    // ConnectionIsDown(Uuid, String, String),
    ConnectionIsDown {
        original_request: DbRequest,
        reason: DownConnectionReason,
    },
    None,
}

impl ConnectionsManager {
    pub fn new(tx: Sender<DbResponse>) -> Self {
        Self {
            connections: HashMap::new(),
            tx,
        }
    }

    fn process_db_error(
        &mut self,
        key: &ConnectionKey,
        error: sqlx::Error,
        id: Uuid,
        repeat: Option<&dyn Fn() -> DbRequest>,
    ) -> DbResponse {
        tracing::error!("Error executing query: {:?}", error);
        match error {
            sqlx::Error::Database(db_error) => {
                return DbResponse::Error(id, db_error.message().to_string())
            }
            sqlx::Error::Io(io_error) if repeat.is_some() => {
                if let Some(repeat) = repeat {
                    self.connections.remove(&key);
                    DbResponse::ConnectionIsDown {
                        original_request: repeat(),
                        reason: DownConnectionReason::IoError(format!("IO Error: {:?}", io_error)),
                    }
                } else {
                    DbResponse::Error(id, format!("IO Error: {:?}", io_error))
                }
            }
            _ => DbResponse::Error(id, format!("unknown db error: {:?}", error)),
        }
    }

    fn process_request(&mut self, request: DbRequest) -> DbResponse {
        match request {
            DbRequest::ListTables {
                server_id,
                database,
                schema,
                retries,
            } => {
                let connection_key = ConnectionKey {
                    name: database.to_string(),
                    server_id,
                };
                let repeat = || DbRequest::ListTables {
                    server_id,
                    database: (&database).to_string(),
                    schema: (&schema).to_string(),
                    retries: retries + 1,
                };
                if let Some(connection) = self.connections.get_mut(&connection_key) {
                    match task::block_on(connection.list_tables(&schema)) {
                        Ok(tables) => DbResponse::TablesListed {
                            server_id,
                            database: database.to_string(),
                            schema: schema.to_string(),
                            tables,
                        },
                        Err(e) => {
                            self.process_db_error(&connection_key, e, server_id, Some(&repeat))
                        }
                    }
                } else {
                    DbResponse::Error(server_id, "No connection to database".to_string())
                }
            }
            DbRequest::ListSchemas {
                server_id,
                database,
                retries,
            } => {
                let connection_key = ConnectionKey {
                    name: database.to_string(),
                    server_id,
                };
                let repeat = || DbRequest::ListSchemas {
                    server_id,
                    database: database.to_string(),
                    retries: retries + 1,
                };
                if let Some(connection) = self.connections.get_mut(&connection_key) {
                    match task::block_on(connection.list_schemas()) {
                        Ok(tables) => DbResponse::SchemasListed {
                            server_id,
                            database: database.to_string(),
                            schemas: tables,
                        },
                        Err(e) => {
                            self.process_db_error(&connection_key, e, server_id, Some(&repeat))
                        }
                    }
                } else {
                    DbResponse::ConnectionIsDown {
                        original_request: repeat(),
                        reason: DownConnectionReason::MissingConnection,
                    }
                }
            }
            DbRequest::ConnectToServer(id, url) => {
                let connection_key = ConnectionKey {
                    name: DEFAULT_MANAGEMENT_DATABASE.to_string(),
                    server_id: id,
                };
                if self.connections.contains_key(&connection_key) {
                    return DbResponse::Connected(id);
                }
                match task::block_on(Connection::connect(&connection_key.name, &url)) {
                    Ok(connection) => {
                        self.connections.insert(connection_key, connection);
                        DbResponse::Connected(id)
                    }
                    Err(e) => self.process_db_error(&connection_key, e, id, None),
                }
            }
            DbRequest::ConnectToDatabase(id, name, url) => {
                let connection_key = ConnectionKey {
                    name,
                    server_id: id,
                };
                if self.connections.contains_key(&connection_key) {
                    return DbResponse::None;
                }
                match task::block_on(Connection::connect(&connection_key.name, &url)) {
                    Ok(connection) => {
                        self.connections.insert(connection_key, connection);
                        DbResponse::None
                    }
                    Err(e) => self.process_db_error(&connection_key, e, id, None),
                }
            }
            DbRequest::Execute(id, name, query, retries) => {
                let connection_key = ConnectionKey {
                    name: name.to_string(),
                    server_id: id,
                };
                let repeat =
                    || DbRequest::Execute(id, (&name).clone(), (&query).clone(), retries + 1);
                match task::block_on(self.execute(&query, &connection_key)) {
                    Ok(Some((headers, data))) => DbResponse::Executed(id, headers, data),
                    Ok(None) => DbResponse::ConnectionIsDown {
                        original_request: repeat(),
                        reason: DownConnectionReason::MissingConnection,
                    },
                    // Ok(None) => DbResponse::ConnectionIsDown(id, name, query),
                    Err(e) => self.process_db_error(&connection_key, e, id, Some(&repeat)),
                }
            }
            DbRequest::ListDatabases(id) => {
                let connection_key = ConnectionKey {
                    name: DEFAULT_MANAGEMENT_DATABASE.to_string(),
                    server_id: id,
                };
                if let Some(connection) = self.connections.get_mut(&connection_key) {
                    match task::block_on(connection.list_databases()) {
                        Ok(databases) => DbResponse::DatabasesListed(id, databases),
                        Err(e) => self.process_db_error(&connection_key, e, id, None),
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
        query: &str,
        key: &ConnectionKey,
    ) -> Result<Option<(Vec<String>, Vec<Vec<String>>)>, sqlx::Error> {
        tracing::info!("Executing query: {}", query);
        let connection = match self.connections.get_mut(&key) {
            Some(connection) => connection,
            None => return Ok(None),
        };
        let connection = match &mut connection.internal {
            TypedConnection::Postgres(connection) => connection,
        };

        connection
            .execute_sqlx(query)
            .await
            .map(|(headers, data)| Some((headers, data)))
    }
}

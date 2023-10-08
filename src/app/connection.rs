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

pub(crate) struct Connection {
    pub(crate) name: String,
    pub(crate) url: String,

    pub(crate) internal: TypedConnection,
}

const DEFAULT_MANAGEMENT_DATABASE: &str = "postgres";

impl Connection {
    pub(crate) async fn connect(name: &str, url: &str) -> Result<Self, sqlx::Error> {
        let opts: PgConnectOptions = url.parse()?;
        let opts = opts.database(&name);
        log::info!("Connecting to database: {:?}", opts);
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

pub(crate) enum TypedConnection {
    Postgres(PgConnection),
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
        log::error!("Error executing query: {:?}", error);
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
                Err(e) => {
                    log::error!("Error receiving request from main thread: {:?}", e);
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
        log::info!("Executing query: {}", query);
        let key = ConnectionKey {
            name,
            server_id: id,
        };
        let connection = self.connections.get_mut(&key).unwrap();
        let connection = match &mut connection.internal {
            TypedConnection::Postgres(connection) => connection,
        };

        // let prepared_query = connection.prepare(&query).await?;
        let mut headers: Vec<String> = vec![];
        let mut rows = sqlx::query(&query)
            .persistent(false)
            .map(|row: PgRow| {
                let mut data: Vec<String> = vec![];
                if headers.is_empty() {
                    headers = row
                        .columns()
                        .iter()
                        .map(|col| col.name().to_string())
                        .collect();
                }
                // select * from test;
                // select * from test2;
                for (i, col) in row.columns().iter().enumerate() {
                    let type_info = col.type_info();
                    match *type_info.kind() {
                        PgTypeKind::Simple => {
                            // let raw = row.try_get_raw(i).unwrap();
                            // println!("{}: {:?}", col.name(), raw.as_str().unwrap());
                            let oid = type_info.oid().unwrap().0;

                            if oid >= 20 && oid <= 23 {
                                let val: i32 = row.get::<i32, usize>(i);
                                // println!("{}: {:?}", col.name(), val);
                                data.push(format!("{}", val));
                            } else if (oid == 1043) || (oid == 25) {
                                let val: String = row.get::<String, usize>(i);
                                // println!("{}: {:?}", col.name(), val);
                                data.push(val);
                            } else {
                                log::warn!("Unknown oid: {}, try bind to string", oid);
                                let val: String = row.get::<String, usize>(i);
                                data.push(val);
                            }

                            // match  type_info {
                            //     PgTypeInfo::BOOL => {}
                            // }
                        }
                        PgTypeKind::Pseudo => todo!(),
                        PgTypeKind::Domain(_) => todo!(),
                        PgTypeKind::Composite(_) => todo!(),
                        PgTypeKind::Array(_) => todo!(),
                        PgTypeKind::Enum(_) => todo!(),
                        PgTypeKind::Range(_) => todo!(),
                    };
                    // if PgType::from_oid(col.type_info().oid().unwrap().0).unwrap() == PgType::INT4 {
                    //     let val: i32 = row.get::<i32, usize>(i);
                    //     println!("val: {:?}", val);
                    // }
                }

                // println!("row 0: {:?}", data.as_str().unwrap());
                // data.as_str().unwrap();
                data
                //     data.as_str().unwrap()
            })
            .fetch(connection);

        // let rows = sqlx::query(&query).fetch(&mut conn);

        let mut data: Vec<Vec<String>> = vec![];

        while let Some(row) = rows.try_next().await? {
            data.push(row);
            // map the row into a user-defined domain type
            // let email: &str = row.try_get("email")?;
            // let data = row.try_get_raw(0).unwrap();
            // println!("test: {:?}", row);
        }
        drop(rows);

        // if data.is_empty() {
        //     headers = prepared_query
        //         .columns()
        //         .iter()
        //         .map(|col| col.name().to_string())
        //         .collect();
        // }

        // println!("row: {:?}", row);

        Ok((headers, data))
    }
}

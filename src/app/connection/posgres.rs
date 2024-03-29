use async_trait::async_trait;
use futures::TryStreamExt;
use sqlx::{
    postgres::{PgRow, PgTypeInfo, PgTypeKind},
    Column, PgConnection, Postgres, Row, Type, TypeInfo,
};

use super::{
    executing::Executing,
    types::{GenericArrayTypeWriter, GenericTypeWriter},
};

// TODO: check other drivers, like f.e https://www.sea-ql.org/SeaORM/docs/basic-crud/raw-sql/#use-raw-query--execute-interface

// types: https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html
// types: https://docs.rs/sqlx-mysql/0.7.2/sqlx_mysql/types/index.html
// types: https://docs.rs/sqlx-sqlite/0.7.2/sqlx_sqlite/types/index.html

struct PgWriter {}

impl GenericTypeWriter<'_, PgTypeInfo, PgRow, Postgres> for PgWriter {}

struct PgArrayWriter {}

impl GenericArrayTypeWriter<'_, PgTypeInfo, PgRow, Postgres> for PgArrayWriter {}

#[async_trait]
impl Executing for PgConnection {
    async fn execute_sqlx(
        &mut self,
        query: &str,
    ) -> Result<(Vec<String>, Vec<Vec<String>>), sqlx::Error> {
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
                for (i, col) in row.columns().iter().enumerate() {
                    let type_info = col.type_info();
                    // tracing::debug!("checking type: {}", type_info.name());

                    match type_info.kind() {
                        PgTypeKind::Simple => {
                            // tracing::debug!("Simple type: {}", type_info.name());
                            PgWriter::write_row_cell(type_info, &row, i, &mut data);
                        }
                        PgTypeKind::Array(internal_type_info) => {
                            // tracing::debug!("Array type: {}", type_info.name());
                            PgArrayWriter::write_row_cell(internal_type_info, &row, i, &mut data);
                        }
                        PgTypeKind::Pseudo => {
                            tracing::debug!("Pseudo type not supported: {}", type_info.name());
                            data.push("not supported".to_string());
                        }
                        PgTypeKind::Domain(_) => {
                            tracing::debug!("Domain type not supported: {}", type_info.name());
                            data.push("not supported".to_string());
                        }
                        PgTypeKind::Composite(_) => {
                            tracing::debug!("Composite type not supported: {}", type_info.name());
                            data.push("not supported".to_string());
                        }
                        PgTypeKind::Enum(enum_values) => {
                            match row.try_get_raw(i) {
                                Ok(value) => {
                                    match value.as_str() {
                                        Ok(value) => data.push(value.to_string()),
                                        Err(e) => {
                                            tracing::debug!("Error getting enum value: {} {:?} {}", type_info.name(), enum_values, e);
                                            data.push("not supported".to_string());
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::debug!("Error getting enum value: {} {:?} {}", type_info.name(), enum_values, e);
                                    data.push("not supported".to_string());
                                }
                            }
                        }
                        PgTypeKind::Range(_) => {
                            tracing::debug!("Range type not supported: {}", type_info.name());
                            data.push("not supported".to_string());
                        }
                    };
                }

                data
            })
            .fetch(self);
            // .fetch_all(self);

        let mut data: Vec<Vec<String>> = vec![];

        // for row in rows.await?.into_iter() {
        //         data.push(row);
        // }

        while let Some(row) = rows.try_next().await? {
            data.push(row);
        }
        drop(rows);

        Ok((headers, data))
    }
}

use std::fmt::Display;

use async_trait::async_trait;
use futures::TryStreamExt;
use sqlx::{
    postgres::{PgRow, PgTypeKind, PgTypeInfo},
    Column, ColumnIndex, Database, PgConnection, Postgres, Row, Type, TypeInfo,
};

use super::{executing::Executing, types::GenericTypeWriter};

// TODO: check other drivers, like f.e https://www.sea-ql.org/SeaORM/docs/basic-crud/raw-sql/#use-raw-query--execute-interface


// types: https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html
// types: https://docs.rs/sqlx-mysql/0.7.2/sqlx_mysql/types/index.html
// types: https://docs.rs/sqlx-sqlite/0.7.2/sqlx_sqlite/types/index.html

struct PgWriter {}

impl GenericTypeWriter<'_, PgTypeInfo, PgRow, Postgres> for PgWriter {}

#[async_trait]
impl Executing for PgConnection {
    async fn execute_sqlx(
        &mut self,
        query: String,
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
                    match *type_info.kind() {
                        PgTypeKind::Simple => {
                            PgWriter::write_row_cell(type_info, &row, i, &mut data);
                        }
                        PgTypeKind::Pseudo => todo!(),
                        PgTypeKind::Domain(_) => todo!(),
                        PgTypeKind::Composite(_) => todo!(),
                        PgTypeKind::Array(_) => todo!(),
                        PgTypeKind::Enum(_) => todo!(),
                        PgTypeKind::Range(_) => todo!(),
                    };
                }

                data
            })
            .fetch(self);

        let mut data: Vec<Vec<String>> = vec![];

        while let Some(row) = rows.try_next().await? {
            data.push(row);
        }
        drop(rows);

        Ok((headers, data))
    }
}

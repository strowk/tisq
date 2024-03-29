use std::fmt::Display;
use std::sync::Arc;

use async_trait::async_trait;
use futures::TryStreamExt;
use itertools::Itertools;
use sqlx::types::BigDecimal;
use sqlx::ValueRef;
use sqlx::{
    error::BoxDynError,
    postgres::{PgRow, PgTypeInfo, PgTypeKind},
    Column, PgConnection, Postgres, Row, Type, TypeInfo,
};

use super::{
    executing::Executing,
    types::{GenericArrayTypeWriter, GenericTypeWriter},
};
use sqlx_postgres::types::PgRecordDecoder;

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
                        PgTypeKind::Composite(type_def) => match row.try_get_raw(i) {
                            Ok(value) => {
                                if value.is_null() {
                                    data.push("null".to_string());
                                    continue;
                                }
                                let decoder = PgRecordDecoder::new(value);
                                match decoder {
                                    Ok(mut decoder) => {
                                        decode_and_write(&mut decoder, type_def.clone(), &mut data);
                                    }
                                    Err(e) => {
                                        tracing::debug!(
                                            "Composite type not supported: {} {}",
                                            type_info.name(),
                                            e
                                        );
                                        data.push("not supported".to_string());
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::debug!(
                                    "Composite type not supported: {} {}",
                                    type_info.name(),
                                    e
                                );
                                data.push("not supported".to_string());
                            }
                        },
                        PgTypeKind::Enum(enum_values) => match row.try_get_raw(i) {
                            Ok(value) => match value.as_str() {
                                Ok(value) => data.push(value.to_string()),
                                Err(e) => {
                                    tracing::debug!(
                                        "Error getting enum value: {} {:?} {}",
                                        type_info.name(),
                                        enum_values,
                                        e
                                    );
                                    data.push("not supported".to_string());
                                }
                            },
                            Err(e) => {
                                tracing::debug!(
                                    "Error getting enum value: {} {:?} {}",
                                    type_info.name(),
                                    enum_values,
                                    e
                                );
                                data.push("not supported".to_string());
                            }
                        },
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

fn do_for_known_types(decoder: &mut PgRecordDecoder,
    field: &(String, PgTypeInfo),
    result: &mut Vec<(String, String)>,
    type_info: &PgTypeInfo) -> bool {
    return do_for_type::<bool>(decoder, field, result, type_info)
    || do_for_type::<String>(decoder, field, result, type_info)
    || do_for_type::<i8>(decoder, field, result, type_info)
    || do_for_type::<i16>(decoder, field, result, type_info)
    || do_for_type::<i32>(decoder, field, result, type_info)
    || do_for_type::<i64>(decoder, field, result, type_info)
    || do_for_type::<f32>(decoder, field, result, type_info)
    || do_for_type::<f64>(decoder, field, result, type_info)
    || do_for_type::<BigDecimal>(decoder, field, result, type_info)
    || do_for_type::<sqlx::types::time::PrimitiveDateTime>(
        decoder,
        field,
        result,
        type_info,
    )
    || do_for_type::<sqlx::types::time::OffsetDateTime>(
        decoder,
        field,
        result,
        type_info,
    )
    || do_for_type::<sqlx::types::time::Date>(decoder, field, result, type_info)
    || do_for_type::<sqlx::types::time::Time>(decoder, field, result, type_info)
    || do_for_type::<sqlx::types::uuid::Uuid>(decoder, field, result, type_info)
    || do_for_type::<sqlx::types::ipnetwork::IpNetwork>(
        decoder,
        field,
        result,
        type_info,
    )
    || do_for_type::<sqlx::types::mac_address::MacAddress>(
        decoder,
        field,
        result,
        type_info,
    )
    || do_for_type::<sqlx::types::JsonValue>(decoder, field, result, type_info)
    || do_for_type::<sqlx_postgres::types::PgTimeTz>(
        decoder,
        field,
        result,
        type_info,
    )
    || do_for_type::<sqlx::types::BitVec>(decoder, field, result, type_info)
    || do_for_type::<Vec<u8>>(decoder, field, result, type_info)
    || do_for_type::<()>(decoder, field, result, type_info);
}

fn decode_and_write(
    decoder: &mut PgRecordDecoder,
    types: Arc<[(String, PgTypeInfo)]>,
    data: &mut Vec<String>,
) {
    let mut result = vec![];
    for (_, field) in types.iter().enumerate() {
        let type_info = &field.1;
        match type_info.kind()  {
            PgTypeKind::Simple => {
                if ! do_for_known_types(decoder, field, &mut result, type_info) {
                    result.push((field.0.clone(), "not supported".to_string()));
                }
            },
            _ => {
                // TODO: support nested composites
                result.push((field.0.clone(), "not supported".to_string()));
            }
        }
    }

    data.push(format!(
        "({})",
        result
            .iter()
            .map(|(name, value)| { format!("{}:{}", name, value) })
            .join(",")
    ));
}

fn do_for_type<K>(
    decoder: &mut PgRecordDecoder,
    field: &(String, PgTypeInfo),
    result: &mut Vec<(String, String)>,
    type_info: &PgTypeInfo,
) -> bool
where
    K: sqlx::Type<sqlx_postgres::Postgres>,
    K: for<'b> sqlx::Decode<'b, sqlx_postgres::Postgres>,
    K: PgDisplayComposite,
{
    if let Some(value) = decode_for_type::<K>(decoder, type_info) {
        write_for_type(value, field, result, type_info);
        return true;
    }
    false
}

fn write_for_type<K>(
    value: Result<K, BoxDynError>,
    field: &(String, PgTypeInfo),
    result: &mut Vec<(String, String)>,
    type_info: &PgTypeInfo,
) where
    K: sqlx::Type<sqlx_postgres::Postgres> + PgDisplayComposite,
{
    match value {
        Ok(value) => {
            result.push((field.0.clone(), value.to_string()));
        }
        Err(e) => {
            tracing::debug!("Composite type decoding issue: {} {}", type_info.name(), e);
            result.push((field.0.clone(), "not supported".to_string()));
        }
    }
}

fn decode_for_type<K>(
    decoder: &mut PgRecordDecoder,
    type_info: &PgTypeInfo,
) -> Option<Result<K, BoxDynError>>
where
    K: sqlx::Type<sqlx_postgres::Postgres>,
    K: for<'b> sqlx::Decode<'b, sqlx_postgres::Postgres>,
{
    if <K as Type<sqlx_postgres::Postgres>>::compatible(type_info) {
        Some(decoder.try_decode::<K>())
    } else {
        None
    }
}

trait PgDisplayComposite {
    fn to_string(&self) -> String;
}

impl PgDisplayComposite for sqlx_postgres::types::PgTimeTz {
    fn to_string(&self) -> String {
        format!("{}{}", self.time, self.offset)
    }
}

impl PgDisplayComposite for sqlx::types::BitVec {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl PgDisplayComposite for Vec<u8> {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl PgDisplayComposite for () {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

trait UseStdDisplay {}
impl UseStdDisplay for bool {}
impl UseStdDisplay for String {}
impl UseStdDisplay for i8 {}
impl UseStdDisplay for i16 {}
impl UseStdDisplay for i32 {}
impl UseStdDisplay for i64 {}
impl UseStdDisplay for f32 {}
impl UseStdDisplay for f64 {}
impl UseStdDisplay for BigDecimal {}
impl UseStdDisplay for sqlx::types::time::PrimitiveDateTime {}
impl UseStdDisplay for sqlx::types::time::OffsetDateTime {}
impl UseStdDisplay for sqlx::types::time::Date {}
impl UseStdDisplay for sqlx::types::time::Time {}
impl UseStdDisplay for sqlx::types::uuid::Uuid {}
impl UseStdDisplay for sqlx::types::ipnetwork::IpNetwork {}
impl UseStdDisplay for sqlx::types::mac_address::MacAddress {}
impl UseStdDisplay for sqlx::types::JsonValue {}

impl<T: Display + UseStdDisplay> PgDisplayComposite for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

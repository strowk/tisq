use std::fmt::{Debug, Display};

use sqlx::{types::BigDecimal, ColumnIndex, Row, Type, TypeInfo};

pub(super) trait GenericArrayTypeWriter<'a, T, R, D>
where
    R: Row<Database = D>,
    D: sqlx::Database<TypeInfo = T>,

    bool: sqlx::Type<D>,
    String: sqlx::Type<D>,
    i8: sqlx::Type<D>,
    i16: sqlx::Type<D>,
    i32: sqlx::Type<D>,
    i64: sqlx::Type<D>,
    f32: sqlx::Type<D>,
    f64: sqlx::Type<D>,
    Vec<u8>: sqlx::Type<D>,
    BigDecimal: sqlx::Type<D>,
    sqlx::types::time::PrimitiveDateTime: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::time::OffsetDateTime: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::time::Date: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::time::Time: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::uuid::Uuid: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::ipnetwork::IpNetwork: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::mac_address::MacAddress: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::BitVec: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::JsonValue: sqlx::Type<D> + sqlx::Decode<'a, D>,

    sqlx_postgres::types::PgTimeTz: sqlx::Type<D> + sqlx::Decode<'a, D>,

    Vec<bool>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<String>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<i8>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<i16>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<i32>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<i64>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<f32>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<f64>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<Vec<u8>>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<BigDecimal>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    // Vec<()>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    usize: ColumnIndex<R>,

    Vec<sqlx::types::time::PrimitiveDateTime>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<sqlx::types::time::OffsetDateTime>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<sqlx::types::time::Date>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<sqlx::types::time::Time>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<sqlx::types::uuid::Uuid>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<sqlx::types::ipnetwork::IpNetwork>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<sqlx::types::mac_address::MacAddress>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<sqlx::types::BitVec>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<sqlx::types::JsonValue>: sqlx::Type<D> + sqlx::Decode<'a, D>,

    Vec<sqlx_postgres::types::PgTimeTz>: sqlx::Type<D> + sqlx::Decode<'a, D>,
{
    fn write_row_cell(type_info: &T, row: &'a R, i: usize, data: &mut Vec<String>) {
        if Self::write_via_display::<bool>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<String>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<i64>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<i32>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<i16>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<i8>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<f32>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<f64>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<BigDecimal>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_debug::<Vec<u8>>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<sqlx::types::time::PrimitiveDateTime>(type_info, row, i, data)
        {
            return;
        }
        if Self::write_via_display::<sqlx::types::time::OffsetDateTime>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<sqlx::types::time::Date>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<sqlx::types::time::Time>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_custom_display::<sqlx_postgres::types::PgTimeTz>(type_info, row, i, data)
        {
            return;
        }
        if Self::write_via_display::<sqlx::types::uuid::Uuid>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<sqlx::types::ipnetwork::IpNetwork>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<sqlx::types::mac_address::MacAddress>(type_info, row, i, data)
        {
            return;
        }
        if Self::write_via_debug::<sqlx::types::BitVec>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<sqlx::types::JsonValue>(type_info, row, i, data) {
            return;
        }
        data.push("not supported".to_string());
    }

    fn write_via_debug<K>(type_info: &T, row: &'a R, i: usize, data: &mut Vec<String>) -> bool
    where
        K: sqlx::Type<D>,
        Vec<K>: sqlx::Type<D>,
        Vec<K>: sqlx::Decode<'a, D>,
        K: Debug,
    {
        if <K as Type<D>>::compatible(type_info) {
            let val: Option<Vec<K>> = row.get::<Option<Vec<K>>, usize>(i);
            let val = val
                .map(|val| format!("{:?}", val))
                .unwrap_or_else(|| "null".to_string());
            data.push(val);
            return true;
        }
        false
    }

    fn write_via_custom_display<K>(
        type_info: &T,
        row: &'a R,
        i: usize,
        data: &mut Vec<String>,
    ) -> bool
    where
        Vec<K>: sqlx::Type<D>,
        K: sqlx::Type<D>,
        Vec<K>: sqlx::Decode<'a, D>,
        K: CustomDisplay,
    {
        if <K as Type<D>>::compatible(type_info) {
            let val: Option<Vec<K>> = row.get::<Option<Vec<K>>, usize>(i);
            let val = val
                .map(|val| {
                    "[".to_string()
                        + &val
                            .iter()
                            .map(|val_item| val_item.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                        + "]"
                })
                .unwrap_or_else(|| "null".to_string());
            data.push(val);
            return true;
        }
        false
    }

    fn write_via_display<K>(type_info: &T, row: &'a R, i: usize, data: &mut Vec<String>) -> bool
    where
        Vec<K>: sqlx::Type<D>,
        K: sqlx::Type<D>,
        Vec<K>: sqlx::Decode<'a, D>,
        K: Display,
    {
        if <K as Type<D>>::compatible(type_info) {
            let val: Option<Vec<K>> = row.get::<Option<Vec<K>>, usize>(i);
            let val = val
                .map(|val| {
                    "[".to_string()
                        + &val
                            .iter()
                            .map(|val_item| val_item.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                        + "]"
                })
                .unwrap_or_else(|| "null".to_string());
            data.push(val);
            return true;
        }
        false
    }
}

pub(super) trait GenericTypeWriter<'a, T, R, D>
where
    R: Row<Database = D>,
    D: sqlx::Database<TypeInfo = T>,
    T: Debug + TypeInfo,
    bool: sqlx::Type<D> + sqlx::Decode<'a, D>,
    String: sqlx::Type<D> + sqlx::Decode<'a, D>,
    i8: sqlx::Type<D> + sqlx::Decode<'a, D>,
    i16: sqlx::Type<D> + sqlx::Decode<'a, D>,
    i32: sqlx::Type<D> + sqlx::Decode<'a, D>,
    i64: sqlx::Type<D> + sqlx::Decode<'a, D>,
    f32: sqlx::Type<D> + sqlx::Decode<'a, D>,
    f64: sqlx::Type<D> + sqlx::Decode<'a, D>,
    Vec<u8>: sqlx::Type<D> + sqlx::Decode<'a, D>,
    BigDecimal: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::time::PrimitiveDateTime: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::time::OffsetDateTime: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::time::Date: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::time::Time: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx_postgres::types::PgTimeTz: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::uuid::Uuid: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::ipnetwork::IpNetwork: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::mac_address::MacAddress: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::BitVec: sqlx::Type<D> + sqlx::Decode<'a, D>,
    sqlx::types::JsonValue: sqlx::Type<D> + sqlx::Decode<'a, D>,
    (): sqlx::Type<D> + sqlx::Decode<'a, D>,
    usize: ColumnIndex<R>,
{
    fn write_row_cell(type_info: &T, row: &'a R, i: usize, data: &mut Vec<String>) {
        if Self::write_via_display::<bool>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<String>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<i64>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<i32>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<i16>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<i8>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<f32>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<f64>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<BigDecimal>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_debug::<Vec<u8>>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_debug::<()>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<sqlx::types::time::PrimitiveDateTime>(type_info, row, i, data)
        {
            return;
        }
        if Self::write_via_display::<sqlx::types::time::OffsetDateTime>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<sqlx::types::time::Date>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<sqlx::types::time::Time>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_custom_display::<sqlx_postgres::types::PgTimeTz>(type_info, row, i, data)
        {
            return;
        }
        if Self::write_via_display::<sqlx::types::uuid::Uuid>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<sqlx::types::ipnetwork::IpNetwork>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<sqlx::types::mac_address::MacAddress>(type_info, row, i, data)
        {
            return;
        }
        if Self::write_via_debug::<sqlx::types::BitVec>(type_info, row, i, data) {
            return;
        }
        if Self::write_via_display::<sqlx::types::JsonValue>(type_info, row, i, data) {
            return;
        }
        tracing::debug!("Type not supported: {:?}", type_info);
        data.push(format!("{} not supported", type_info.name()));

        // TODO: Add support for other types
        // PgInterval	INTERVAL
        // PgRange<T>	INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE
        // PgMoney	MONEY
        // PgLTree	LTREE
        // PgLQuery	LQUERY

        // check more in https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html
    }

    fn write_via_custom_display<K>(
        type_info: &T,
        row: &'a R,
        i: usize,
        data: &mut Vec<String>,
    ) -> bool
    where
        K: sqlx::Type<D>,
        K: sqlx::Decode<'a, D>,
        K: CustomDisplay,
    {
        if <K as Type<D>>::compatible(type_info) {
            let val: Option<K> = row.get::<Option<K>, usize>(i);
            let val = val
                .map(|val| val.to_string())
                .unwrap_or_else(|| "null".to_string());
            data.push(val);
            return true;
        }
        false
    }

    fn write_via_debug<K>(type_info: &T, row: &'a R, i: usize, data: &mut Vec<String>) -> bool
    where
        K: sqlx::Type<D>,
        K: sqlx::Decode<'a, D>,
        K: Debug,
    {
        if <K as Type<D>>::compatible(type_info) {
            let val: Option<K> = row.get::<Option<K>, usize>(i);
            let val = val
                .map(|val| format!("{:?}", val))
                .unwrap_or_else(|| "null".to_string());
            data.push(val);
            return true;
        }
        false
    }

    fn write_via_display<K>(type_info: &T, row: &'a R, i: usize, data: &mut Vec<String>) -> bool
    where
        K: sqlx::Type<D>,
        K: sqlx::Decode<'a, D>,
        K: Display,
    {
        if <K as Type<D>>::compatible(type_info) {
            let val: Option<K> = row.get::<Option<K>, usize>(i);
            let val = val
                .map(|val| val.to_string())
                .unwrap_or_else(|| "null".to_string());
            data.push(val);
            return true;
        }
        false
    }
}


pub(super) trait CustomDisplay {
    fn to_string(&self) -> String;
}

impl CustomDisplay for sqlx_postgres::types::PgTimeTz {
    fn to_string(&self) -> String {
        format!("{}{}", self.time, self.offset)
    }
}

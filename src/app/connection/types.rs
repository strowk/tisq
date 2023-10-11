use std::fmt::{Debug, Display};

use sqlx::{
    postgres::{PgRow, PgTypeInfo},
    types::BigDecimal,
    ColumnIndex, Postgres, Row, Type,
};

pub(super) trait GenericTypeWriter<'a, T, R, D>
where
    R: Row<Database = D>,
    D: sqlx::Database<TypeInfo = T>,
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

        // TODO: Add support for other types
        // PgInterval	INTERVAL
        // PgRange<T>	INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE
        // PgMoney	MONEY
        // PgLTree	LTREE
        // PgLQuery	LQUERY

        // Requires the time Cargo feature flag.

        // Rust type	Postgres type(s)
        // Rust type	Postgres type(s)
        // time::PrimitiveDateTime	TIMESTAMP
        // time::OffsetDateTime	TIMESTAMPTZ
        // time::Date	DATE
        // time::Time	TIME
        // [PgTimeTz]	TIMETZ

        // Requires the uuid Cargo feature flag.

        // uuid::Uuid	UUID

        // Requires the ipnetwork Cargo feature flag.

        //         ipnetwork::IpNetwork	INET, CIDR
        // std::net::IpAddr	INET, CIDR

        // check more in https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html
    }

    fn write_via_debug<K>(type_info: &T, row: &'a R, i: usize, data: &mut Vec<String>) -> bool
    where
        K: sqlx::Type<D>,
        K: sqlx::Decode<'a, D>,
        K: Debug,
    {
        if <K as Type<D>>::compatible(type_info) {
            let val: K = row.get::<K, usize>(i);
            data.push(format!("{:?}", val));
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

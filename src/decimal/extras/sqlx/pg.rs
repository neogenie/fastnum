use sqlx::postgres::{types::Oid, PgTypeInfo};

pub const NUMERIC: PgTypeInfo = PgTypeInfo::with_oid(Oid(1700));
pub const NUMERIC_ARRAY: PgTypeInfo = PgTypeInfo::with_oid(Oid(1231));

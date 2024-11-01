// use sqlx::decode::Decode;
// use sqlx::encode::{Encode, IsNull};
// use sqlx::error::BoxDynError;
// use sqlx::postgres::types::Oid;
// use sqlx::postgres::{
//     PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres,
// };
// use sqlx::types::numeric::{PgNumeric, PgNumericSign};
// use sqlx::types::Type;
// 
// use crate::decimal::extras::utils::db::postgres;
// use crate::decimal::macros::decimal_err;
// use crate::decimal::unsigned::UnsignedDecimal;
// use crate::decimal::ParseError;
// use crate::{UD128, UD256, UD512};
// 
// impl<UINT> Type<Postgres> for UnsignedDecimal<UINT> {
//     fn type_info() -> PgTypeInfo {
//         PgTypeInfo::with_oid(Oid(1700))
//     }
// }
// 
// impl<UINT> PgHasArrayType for UnsignedDecimal<UINT> {
//     fn array_type_info() -> PgTypeInfo {
//         PgTypeInfo::with_oid(Oid(1231))
//     }
// }
// 
// macro_rules! macro_impl {
//     ($UDEC: ident, $bits: literal, $module: ident) => {
//         impl TryFrom<PgNumeric> for $UDEC {
//             type Error = BoxDynError;
// 
//             fn try_from(numeric: PgNumeric) -> Result<Self, BoxDynError> {
//                 Self::try_from(&numeric)
//             }
//         }
//     };
// }
// 
// macro_impl!(UD128, 128, u128);
// macro_impl!(UD256, 256, u256);
// macro_impl!(UD512, 512, u512);

use std::error::Error;
use std::fmt::Debug;

use diesel::data_types::PgNumeric;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Numeric;
use diesel::{deserialize, serialize};

use crate::decimal::unsigned::UnsignedDecimal;
use crate::decimal::extras::utils::db::postgres;
use crate::decimal::ParseError;
use crate::decimal::macros::decimal_err;
use crate::{UD128, UD256, UD512};

macro_rules! macro_impl {
    ($UDEC: ident, $bits: literal, $module: ident) => {
        impl<'a> TryFrom<&'a PgNumeric> for $UDEC {
            type Error = Box<dyn Error + Send + Sync>;

            fn try_from(numeric: &'a PgNumeric) -> deserialize::Result<Self> {
                let (weight, scale, digits) = match *numeric {
                    PgNumeric::Positive {
                        weight,
                        scale,
                        ref digits,
                    } => (weight, scale, digits),
                    PgNumeric::Negative { .. } => return Err(decimal_err!($UDEC, ParseError::Signed).into()),
                    PgNumeric::NaN => return Err(decimal_err!($UDEC, ParseError::NaN).into()),
                };
                
                postgres::$module::from_nbase(weight, scale, digits).map_err(|e| {
                    decimal_err!($UDEC, e).into()
                })
            }
        }

        impl TryFrom<PgNumeric> for $UDEC {
            type Error = Box<dyn Error + Send + Sync>;
            
            #[inline]
            fn try_from(numeric: PgNumeric) -> deserialize::Result<Self> {
                (&numeric).try_into()
            }
        }

        impl<'a> TryFrom<&'a $UDEC> for PgNumeric {
            type Error = Box<dyn Error + Send + Sync>;
            
            fn try_from(decimal: &'a $UDEC) -> deserialize::Result<Self> {
                
                let (weight, scale, digits) = postgres::$module::to_nbase(decimal).map_err(|e| {
                    decimal_err!($UDEC, e)
                })?;
                
                Ok(PgNumeric::Positive {
                    weight,
                    scale,
                    digits,
                })
            }
        }

        impl TryFrom<$UDEC> for PgNumeric {
            type Error = Box<dyn Error + Send + Sync>;
            
            #[inline]
            fn try_from(decimal: $UDEC) -> deserialize::Result<Self> {
                (&decimal).try_into()
            }
        }
    };
}

macro_impl!(UD128, 128, u128);
macro_impl!(UD256, 256, u256);
macro_impl!(UD512, 512, u512);

impl<UINT> ToSql<Numeric, Pg> for UnsignedDecimal<UINT>
where
    UnsignedDecimal<UINT>: Debug,
    PgNumeric: for<'a> TryFrom<&'a UnsignedDecimal<UINT>, Error = Box<dyn Error + Send + Sync>>,
{
    #[inline]
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let numeric = PgNumeric::try_from(self)?;
        ToSql::<Numeric, Pg>::to_sql(&numeric, &mut out.reborrow())
    }
}

impl<UINT> FromSql<Numeric, Pg> for UnsignedDecimal<UINT>
where
    Self: TryFrom<PgNumeric, Error = Box<dyn Error + Send + Sync>>,
{
    #[inline]
    fn from_sql(numeric: PgValue<'_>) -> deserialize::Result<Self> {
        PgNumeric::from_sql(numeric)?.try_into()
    }
}

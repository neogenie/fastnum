use std::error::Error;
use std::fmt::Debug;

use diesel::data_types::PgNumeric;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Numeric;
use diesel::{deserialize, serialize};

use crate::decimal::extras::utils::db::postgres;
use crate::decimal::macros::decimal_err;
use crate::decimal::signed::Decimal;
use crate::decimal::signed::Sign;
use crate::decimal::ParseError;
use crate::{D128, D256, D512};

macro_rules! macro_impl {
    ($DEC: ident, $bits: literal, $module: ident) => {
        impl<'a> TryFrom<&'a PgNumeric> for $DEC {
            type Error = Box<dyn Error + Send + Sync>;

            fn try_from(numeric: &'a PgNumeric) -> deserialize::Result<Self> {
                let (sign, weight, scale, digits) = match *numeric {
                    PgNumeric::Positive {
                        weight,
                        scale,
                        ref digits,
                    } => (Sign::NoSign, weight, scale, digits),
                    PgNumeric::Negative {
                        weight,
                        scale,
                        ref digits,
                    } => (Sign::Minus, weight, scale, digits),
                    PgNumeric::NaN => return Err(decimal_err!($DEC, ParseError::NaN).into()),
                };

                let udec = postgres::$module::from_nbase(weight, scale, digits)
                    .map_err(|e| decimal_err!($DEC, e))?;

                Ok(Decimal::new(udec, sign))
            }
        }

        impl TryFrom<PgNumeric> for $DEC {
            type Error = Box<dyn Error + Send + Sync>;

            #[inline]
            fn try_from(numeric: PgNumeric) -> deserialize::Result<Self> {
                (&numeric).try_into()
            }
        }

        impl<'a> TryFrom<&'a $DEC> for PgNumeric {
            type Error = Box<dyn Error + Send + Sync>;

            fn try_from(decimal: &'a $DEC) -> deserialize::Result<Self> {
                let sign = decimal.sign();
                let udec = decimal.abs();

                let (weight, scale, digits) =
                    postgres::$module::to_nbase(&udec).map_err(|e| decimal_err!($DEC, e))?;

                match sign {
                    Sign::Minus => Ok(PgNumeric::Negative {
                        weight,
                        scale,
                        digits,
                    }),
                    Sign::NoSign | Sign::Plus => Ok(PgNumeric::Positive {
                        weight,
                        scale,
                        digits,
                    }),
                }
            }
        }

        impl TryFrom<$DEC> for PgNumeric {
            type Error = Box<dyn Error + Send + Sync>;

            #[inline]
            fn try_from(decimal: $DEC) -> deserialize::Result<Self> {
                (&decimal).try_into()
            }
        }
    };
}

macro_impl!(D128, 128, u128);
macro_impl!(D256, 256, u256);
macro_impl!(D512, 512, u512);

impl<UINT> ToSql<Numeric, Pg> for Decimal<UINT>
where
    Decimal<UINT>: Debug,
    PgNumeric: for<'a> TryFrom<&'a Decimal<UINT>, Error = Box<dyn Error + Send + Sync>>,
{
    #[inline]
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let numeric = PgNumeric::try_from(self)?;
        ToSql::<Numeric, Pg>::to_sql(&numeric, &mut out.reborrow())
    }
}

impl<UINT> FromSql<Numeric, Pg> for Decimal<UINT>
where
    Self: TryFrom<PgNumeric, Error = Box<dyn Error + Send + Sync>>,
{
    #[inline]
    fn from_sql(numeric: PgValue<'_>) -> deserialize::Result<Self> {
        PgNumeric::from_sql(numeric)?.try_into()
    }
}

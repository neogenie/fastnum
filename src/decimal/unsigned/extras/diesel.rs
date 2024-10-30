use diesel;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql, Queryable};
use diesel::expression::AsExpression;
use diesel::internal::derives::as_expression::Bound;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::{Nullable, SingleValue};

use crate::decimal::unsigned::UnsignedDecimal;
use crate::{UD128, UD256, UD512};

impl<DB, ST, UINT> Queryable<ST, DB> for UnsignedDecimal<UINT>
where
    DB: Backend,
    ST: SingleValue,
    Self: FromSql<ST, DB>,
{
    type Row = Self;
    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(row)
    }
}

impl<'expr, UINT> AsExpression<diesel::sql_types::Decimal> for &'expr UnsignedDecimal<UINT> {
    type Expression = Bound<diesel::sql_types::Decimal, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}
impl<'expr, UINT> AsExpression<Nullable<diesel::sql_types::Decimal>> for &'expr UnsignedDecimal<UINT> {
    type Expression = Bound<Nullable<diesel::sql_types::Decimal>, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<DB, UINT> ToSql<Nullable<diesel::sql_types::Decimal>, DB> for UnsignedDecimal<UINT>
where
    DB: Backend,
    Self: ToSql<diesel::sql_types::Decimal, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        ToSql::<diesel::sql_types::Decimal, DB>::to_sql(self, out)
    }
}

impl<UINT> AsExpression<diesel::sql_types::Decimal> for UnsignedDecimal<UINT> {
    type Expression = Bound<diesel::sql_types::Decimal, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<UINT> AsExpression<Nullable<diesel::sql_types::Decimal>> for UnsignedDecimal<UINT> {
    type Expression = Bound<Nullable<diesel::sql_types::Decimal>, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl ToSql<diesel::sql_types::Decimal, Pg> for Decimal {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<diesel::sql_types::Decimal, Pg>::to_sql(&self.0, out)
    }
}

impl FromSql<diesel::sql_types::Decimal, Pg> for Decimal {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        Ok(Decimal(
            FromSql::<diesel::sql_types::Decimal, Pg>::from_sql(bytes)?,
        ))
    }
}
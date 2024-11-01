#[cfg(feature = "diesel_mysql")]
mod mysql;

#[cfg(feature = "diesel_postgres")]
mod pg;

use diesel;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql, Queryable};
use diesel::expression::AsExpression;
use diesel::internal::derives::as_expression::Bound;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::{Nullable, Numeric, SingleValue};

use crate::decimal::unsigned::UnsignedDecimal;

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

impl<'expr, UINT> AsExpression<Numeric> for &'expr UnsignedDecimal<UINT> {
    type Expression = Bound<Numeric, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}
impl<'expr, UINT> AsExpression<Nullable<Numeric>> for &'expr UnsignedDecimal<UINT> {
    type Expression = Bound<Nullable<Numeric>, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<DB, UINT> ToSql<Nullable<Numeric>, DB> for UnsignedDecimal<UINT>
where
    DB: Backend,
    Self: ToSql<Numeric, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        ToSql::<Numeric, DB>::to_sql(self, out)
    }
}

impl<UINT> AsExpression<Numeric> for UnsignedDecimal<UINT> {
    type Expression = Bound<Numeric, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<UINT> AsExpression<Nullable<Numeric>> for UnsignedDecimal<UINT> {
    type Expression = Bound<Nullable<Numeric>, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

#[cfg(feature = "diesel_mysql")]
mod mysql;

#[cfg(feature = "diesel_postgres")]
mod pg;

use diesel::{
    self,
    backend::Backend,
    deserialize::{self, FromSql, Queryable},
    expression::AsExpression,
    internal::derives::as_expression::Bound,
    serialize::{self, Output, ToSql},
    sql_types::{Nullable, Numeric, SingleValue},
};

use crate::decimal::Decimal;

type D<const N: usize> = Decimal<N>;

impl<DB, ST, const N: usize> Queryable<ST, DB> for D<N>
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

impl<'expr, const N: usize> AsExpression<Numeric> for &'expr D<N> {
    type Expression = Bound<Numeric, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<'expr, const N: usize> AsExpression<Nullable<Numeric>> for &'expr D<N> {
    type Expression = Bound<Nullable<Numeric>, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<DB, const N: usize> ToSql<Nullable<Numeric>, DB> for D<N>
where
    DB: Backend,
    Self: ToSql<Numeric, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        ToSql::<Numeric, DB>::to_sql(self, out)
    }
}

impl<const N: usize> AsExpression<Numeric> for D<N> {
    type Expression = Bound<Numeric, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<const N: usize> AsExpression<Nullable<Numeric>> for D<N> {
    type Expression = Bound<Nullable<Numeric>, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}
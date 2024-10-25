use diesel;
use diesel::deserialize::{self, FromSql, Queryable};
use diesel::expression::AsExpression;
use diesel::internal::derives::as_expression::Bound;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Nullable;

use crate::Decimal;

impl<__DB, __ST> Queryable<__ST, __DB> for Decimal
where
    __DB: diesel::backend::Backend,
    __ST: diesel::sql_types::SingleValue,
    Self: FromSql<__ST, __DB>,
{
    type Row = Self;
    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(row)
    }
}

impl<'__expr> AsExpression<diesel::sql_types::Decimal> for &'__expr Decimal {
    type Expression = Bound<diesel::sql_types::Decimal, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}
impl<'__expr> AsExpression<Nullable<diesel::sql_types::Decimal>> for &'__expr Decimal {
    type Expression = Bound<Nullable<diesel::sql_types::Decimal>, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}
impl<'__expr, '__expr2> AsExpression<diesel::sql_types::Decimal> for &'__expr2 &'__expr Decimal {
    type Expression = Bound<diesel::sql_types::Decimal, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}
impl<'__expr, '__expr2> AsExpression<Nullable<diesel::sql_types::Decimal>>
    for &'__expr2 &'__expr Decimal
{
    type Expression = Bound<Nullable<diesel::sql_types::Decimal>, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}
impl<__DB> diesel::serialize::ToSql<Nullable<diesel::sql_types::Decimal>, __DB> for Decimal
where
    __DB: diesel::backend::Backend,
    Self: ToSql<diesel::sql_types::Decimal, __DB>,
{
    fn to_sql<'__b>(&'__b self, out: &mut Output<'__b, '_, __DB>) -> serialize::Result {
        ToSql::<diesel::sql_types::Decimal, __DB>::to_sql(self, out)
    }
}
impl AsExpression<diesel::sql_types::Decimal> for Decimal {
    type Expression = Bound<diesel::sql_types::Decimal, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}
impl AsExpression<Nullable<diesel::sql_types::Decimal>> for Decimal {
    type Expression = Bound<Nullable<diesel::sql_types::Decimal>, Self>;
    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

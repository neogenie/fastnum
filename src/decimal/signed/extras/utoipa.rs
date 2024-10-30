use crate::decimal::signed::Decimal;

impl<UINT> utoipa::PartialSchema for Decimal<UINT> {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
        <String as utoipa::PartialSchema>::schema()
    }
}
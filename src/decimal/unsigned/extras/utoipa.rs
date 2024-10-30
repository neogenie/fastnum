use crate::decimal::unsigned::UnsignedDecimal;

impl<UINT> utoipa::PartialSchema for UnsignedDecimal<UINT> {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
        <String as utoipa::PartialSchema>::schema()
    }
}
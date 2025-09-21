use alloc::borrow::Cow;

use utoipa::{
    __dev::ComposeSchema,
    openapi::{schema::SchemaType, ObjectBuilder, RefOr, Schema, SchemaFormat, Type},
    *,
};

use crate::decimal::UnsignedDecimal;

impl<const N: usize> ComposeSchema for UnsignedDecimal<N> {
    fn compose(_: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        ObjectBuilder::new()
            .schema_type(SchemaType::Type(Type::String))
            .title(Some(Self::type_name()))
            .description(Some(format!(
                "Fixed-size unsigned {}-bits decimal number",
                N * 64
            )))
            .examples(["0.0", "1.23", "1.5"])
            .format(Some(SchemaFormat::Custom("number".to_string())))
            .build()
            .into()
    }
}

impl<const N: usize> ToSchema for UnsignedDecimal<N> {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed(Self::type_name())
    }
}

use axum::response::IntoResponse;
use macros::IntoResponse;

/// Generated code:
/// ```rust
/// impl IntoResponse for TestEnum {
///   fn into_response(self) -> axum::response::Response {
///     match self {
///         TestEnum::OneJsonField(body) => IntoResponse::into_response(axum::Json(body)),
///         TestEnum::OneField(body) => IntoResponse::into_response(body),
///         TestEnum::ZeroFields() => IntoResponse::into_response(()),
///         TestEnum::NoFields => IntoResponse::into_response(()),
///     }
///   }
/// }
/// ```
#[derive(IntoResponse)]
enum TestEnum {
    OneJsonField(#[json] Vec<String>),
    OneField(&'static str),
    ZeroFields(),
    NoFields,
}

#[test]
fn one_json_field_into_response_works() {
    TestEnum::OneJsonField(vec!["hello".into(), "world".into()]).into_response();
}

#[test]
fn one_field_into_response_works() {
    TestEnum::OneField("world").into_response();
}

#[test]
fn zero_fields_into_response_works() {
    TestEnum::ZeroFields().into_response();
}

#[test]
fn no_fields_into_response_works() {
    TestEnum::NoFields.into_response();
}

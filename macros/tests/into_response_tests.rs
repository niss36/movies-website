use axum::{http::StatusCode, response::IntoResponse};
use macros::IntoResponse;

/// Generated code:
/// ```rust
/// impl IntoResponse for TestEnum {
///   fn into_response(self) -> axum::response::Response {
///     match self {
///         TestEnum::OneJsonField(body) => IntoResponse::into_response((axum::http::StatusCode::OK, axum::Json(body))),
///         TestEnum::OneField(body) => IntoResponse::into_response((axum::http::StatusCode::OK, body)),
///         TestEnum::ZeroFields() => IntoResponse::into_response(axum::http::StatusCode::OK),
///         TestEnum::NoFields => IntoResponse::into_response(axum::http::StatusCode::OK),
///     }
///   }
/// }
/// ```
#[derive(IntoResponse)]
enum TestEnum {
    #[response(status = OK)]
    OneJsonField(#[json] Vec<String>),

    #[response(status = BAD_REQUEST)]
    OneField(&'static str),

    #[response(status = NOT_FOUND)]
    ZeroFields(),

    #[response(status = INTERNAL_SERVER_ERROR)]
    NoFields,
}

#[test]
fn one_json_field_into_response_works() {
    let response = TestEnum::OneJsonField(vec!["hello".into(), "world".into()]).into_response();

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn one_field_into_response_works() {
    let response = TestEnum::OneField("world").into_response();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn zero_fields_into_response_works() {
    let response = TestEnum::ZeroFields().into_response();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn no_fields_into_response_works() {
    let response = TestEnum::NoFields.into_response();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

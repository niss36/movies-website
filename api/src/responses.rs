use axum::{http::StatusCode, response::IntoResponse};
use utoipa::ToResponse;

#[derive(ToResponse)]
pub struct NoContent;
impl IntoResponse for NoContent {
    fn into_response(self) -> axum::response::Response {
        StatusCode::NO_CONTENT.into_response()
    }
}

#[derive(ToResponse)]
pub struct NotFound(pub String);
impl IntoResponse for NotFound {
    fn into_response(self) -> axum::response::Response {
        let NotFound(message) = self;

        (StatusCode::NOT_FOUND, message).into_response()
    }
}

#[derive(ToResponse)]
pub struct DatabaseError;
impl IntoResponse for DatabaseError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
    }
}

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorBody {
    pub message: String,
}

pub fn database_error() -> ApiErrorBody {
    ApiErrorBody {
        message: "Database error".into(),
    }
}

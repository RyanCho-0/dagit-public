use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

use dioxus_translate::Translate;

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum ServiceError {
    //Common
    Unknown(String),
    NotFound,
    BadRequest(String),
    // Auth Errors
    Unauthorized,

    #[translate(
        ko = "인증코드가 만료되었습니다.",
        en = "Verification code is expired."
    )]
    AuthenticationCodeExpired,

    InternalServerError(String),
    ReqwestError(String),
    DatabaseError(String),
    ValidationError(String),
    JwtGenerationFailed(String),
}

impl From<reqwest::Error> for ServiceError {
    fn from(e: reqwest::Error) -> Self {
        ServiceError::ReqwestError(e.to_string())
    }
}

impl From<validator::ValidationErrors> for ServiceError {
    fn from(e: validator::ValidationErrors) -> Self {
        ServiceError::ValidationError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl From<sqlx::Error> for ServiceError {
    fn from(e: sqlx::Error) -> Self {
        ServiceError::DatabaseError(e.to_string())
    }
}

impl ServiceError {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[cfg(feature = "server")]
impl by_axum::axum::response::IntoResponse for ServiceError {
    fn into_response(self) -> by_axum::axum::response::Response {
        let status_code = match self {
            ServiceError::NotFound => by_axum::axum::http::StatusCode::NOT_FOUND,
            ServiceError::Unauthorized => by_axum::axum::http::StatusCode::UNAUTHORIZED,
            ServiceError::BadRequest(_) | ServiceError::ValidationError(_) => {
                by_axum::axum::http::StatusCode::BAD_REQUEST
            }
            ServiceError::InternalServerError(_) | ServiceError::DatabaseError(_) => {
                by_axum::axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            _ => by_axum::axum::http::StatusCode::BAD_REQUEST,
        };

        (status_code, by_axum::axum::Json(self)).into_response()
    }
}

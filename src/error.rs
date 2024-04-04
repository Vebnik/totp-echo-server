use thiserror::Error;
use axum::response::{IntoResponse, Response};
use axum::http::status::StatusCode;
use axum::Json;
use serde_json::json;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Secret parse error")]
    TOTPSecretParseError(#[from] totp_rs::SecretParseError),

    #[error("Totp url error")]
    TOTPTotpUrlError(#[from] totp_rs::TotpUrlError),

    #[error("Rfc6238 error")]
    TOTPRfc6238Error(#[from] totp_rs::Rfc6238Error),

    #[error("Raise some panic error")]
    UBError,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        log::error!("Some error on api handler: {}", self);

        (StatusCode::BAD_REQUEST, Json(json!({"msg": self.to_string()}))).into_response()
    }
}

pub type Result<T, E = CustomError> = anyhow::Result<T, E>;
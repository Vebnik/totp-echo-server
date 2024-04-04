use axum::{
    extract::{Path, Json},
    http::StatusCode, response::{Response, IntoResponse},
};

use crate::error::Result;
use crate::types::{TwoFA, OtpAuthCreate};

pub async fn generate(Path(secret): Path<String>,) -> Result<Response> {
    let totp = TwoFA::new_with_secret(secret).await?;

    Ok((StatusCode::OK, Json(totp)).into_response())
}

pub async fn generate_by_url(Json(body): Json<OtpAuthCreate>) -> Result<Response> {

    let totp = TwoFA::new_with_url(body).await?;

    Ok((StatusCode::OK, Json(totp)).into_response())
}
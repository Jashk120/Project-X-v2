use super::dto::{CreateDidRequest, ResolveDidRequest};
use super::service;
use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

pub async fn create_did_handler(
    Json(req): Json<CreateDidRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let out = service::create(req.network, req.controller)
        .await
        .map_err(internal_error)?;
    Ok(Json(json!(out)))
}

pub async fn resolve_did_handler(
    Json(req): Json<ResolveDidRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let out = service::resolve(req.did)
        .await
        .map_err(internal_error)?;
    Ok(Json(out))
}

fn internal_error(err: anyhow::Error) -> (StatusCode, Json<Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": err.to_string() })),
    )
}

use axum::{
    Router,
    routing::post,
};

use super::handler::{create_did_handler, resolve_did_handler};

pub fn router() -> Router {
    Router::new()
        .route("/did/create", post(create_did_handler))
        .route("/did/resolve", post(resolve_did_handler))
}

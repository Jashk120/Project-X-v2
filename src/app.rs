use axum::Router;

use crate::modules::did::routes::router as did_router;
use crate::modules::health::routes::router as health_router;

pub fn build_app() -> Router {
    Router::new()
        .merge(health_router())
        .merge(did_router())
}

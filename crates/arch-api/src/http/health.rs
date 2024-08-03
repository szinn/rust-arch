use std::sync::Arc;

use arch_core::ArchService;
use axum::{extract::State, response::IntoResponse};
use hyper::StatusCode;

#[tracing::instrument(level = "trace", skip(arch_service))]
pub(crate) async fn health(State(arch_service): State<Arc<ArchService>>) -> impl IntoResponse {
    match arch_service.health_service.is_healthy().await {
        true => StatusCode::OK,
        false => StatusCode::BAD_REQUEST,
    }
}

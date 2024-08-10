use std::sync::Arc;

use arch_core::HealthService;
use axum::{extract::State, response::IntoResponse};
use hyper::StatusCode;

#[tracing::instrument(level = "trace", skip(health_service))]
pub(crate) async fn health(State(health_service): State<Arc<Box<dyn HealthService>>>) -> impl IntoResponse {
    match health_service.is_healthy().await {
        true => StatusCode::OK,
        false => StatusCode::BAD_REQUEST,
    }
}

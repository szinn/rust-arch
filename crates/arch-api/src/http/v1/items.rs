use std::{sync::Arc, time::Duration};

use arch_domain_api::ArchApi;
use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tower_http::timeout::TimeoutLayer;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct Item {}

#[derive(Deserialize)]
struct ItemParams {}

pub(crate) fn get_routes(_arch_api: Arc<ArchApi>) -> Router<()> {
    Router::new()
        .route("/:id", get(get_item))
        .route("/", post(create_item))
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
}

#[tracing::instrument(level = "trace", skip(_id))]
async fn get_item(Path(_id): Path<Uuid>) -> Result<Json<Item>, StatusCode> {
    Err(StatusCode::BAD_REQUEST)
}

#[tracing::instrument(level = "trace", skip(_params))]
async fn create_item(Json(_params): Json<ItemParams>) -> Json<Item> {
    let item = Item {};

    Json(item)
}

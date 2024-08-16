use std::{sync::Arc, time::Duration};

use arch_domain_api::{item::NewItem, ArchApi, ItemApi};
use arch_utils::arcbox::ArcBox;
use axum::{
    extract::{Path, State},
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

pub(crate) fn get_routes(arch_api: Arc<ArchApi>) -> Router<()> {
    Router::new()
        .route("/:id", get(get_item))
        .route("/", post(create_item))
        .with_state(arch_api.item_api.clone())
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
}

#[tracing::instrument(level = "trace", skip(_id))]
async fn get_item(Path(_id): Path<Uuid>) -> Result<Json<Item>, StatusCode> {
    Err(StatusCode::BAD_REQUEST)
}

#[tracing::instrument(level = "trace", skip(item_api, _params))]
async fn create_item(State(item_api): State<ArcBox<dyn ItemApi>>, Json(_params): Json<ItemParams>) -> Json<Item> {
    let new_item = NewItem {
        text: "This is text".to_string(),
    };
    let _item = item_api.create_item(&new_item).await;

    let item = Item {};

    Json(item)
}

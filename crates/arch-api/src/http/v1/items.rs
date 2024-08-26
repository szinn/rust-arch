use std::{sync::Arc, time::Duration};

use arch_domain_api::{ArchApi, Error, ItemApi};
use arch_domain_models::item::NewItem;
use arch_utils::arcbox::ArcBox;
use axum::{
    extract::{Path, State},
    routing::{get, patch, post},
    Json, Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tower_http::timeout::TimeoutLayer;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct Item {
    pub id: i64,
    pub version: i32,
    pub uuid: Uuid,
    pub text: String,
}

#[derive(Deserialize)]
struct NewItemParams {
    text: String,
}

#[derive(Debug, Deserialize)]
struct UpdateItemParams {
    text: String,
}

pub(crate) fn get_routes(arch_api: Arc<ArchApi>) -> Router<()> {
    Router::new()
        .route("/:uuid", get(get_item))
        .route("/", post(create_item))
        .route("/:uuid", patch(update_item))
        .with_state(arch_api.item_api.clone())
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
}

#[tracing::instrument(level = "trace", skip(item_api))]
async fn get_item(State(item_api): State<ArcBox<dyn ItemApi>>, Path(uuid): Path<Uuid>) -> Result<Json<Item>, StatusCode> {
    match item_api.get_item(&uuid).await {
        Err(_) => Err(StatusCode::BAD_REQUEST),
        Ok(Some(item)) => Ok(Json(item.into())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
    }
}

#[tracing::instrument(level = "trace", skip(item_api, params))]
async fn create_item(State(item_api): State<ArcBox<dyn ItemApi>>, Json(params): Json<NewItemParams>) -> Result<Json<Item>, StatusCode> {
    let new_item = NewItem { text: params.text };

    match item_api.create_item(&new_item).await {
        Err(_) => Err(StatusCode::BAD_REQUEST),
        Ok(item) => Ok(Json(item.into())),
    }
}

#[tracing::instrument(level = "trace", skip(item_api))]
async fn update_item(
    State(item_api): State<ArcBox<dyn ItemApi>>,
    Path(uuid): Path<Uuid>,
    Json(params): Json<UpdateItemParams>,
) -> Result<Json<Item>, StatusCode> {
    match item_api.update_item_text(&uuid, &params.text).await {
        Ok(item) => Ok(Json(item.into())),
        Err(Error::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

impl From<arch_domain_models::item::Item> for Item {
    fn from(value: arch_domain_models::item::Item) -> Self {
        Item {
            id: value.id,
            version: value.version,
            uuid: value.uuid,
            text: value.text,
        }
    }
}

use std::{sync::Arc, time::Duration};

use arch_domain_api::ArchApi;
use axum::Router;
use tower_http::timeout::TimeoutLayer;

pub(crate) mod items;

pub(crate) fn get_routes(arch_api: Arc<ArchApi>) -> Router<()> {
    let items_routes = items::get_routes(arch_api.clone());
    axum::Router::new()
        .nest("/items", items_routes)
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
}

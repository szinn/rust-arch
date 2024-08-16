use std::sync::Arc;

use arch_db::Repository;
use arch_domain_api::{item::NewItem, Error, ItemApi};
use arch_domain_models::item::Item;
use async_trait::async_trait;

#[derive(Clone)]
pub(crate) struct ItemService {
    repository: Arc<Repository>,
}

impl ItemService {
    pub(crate) fn new(repository: Arc<Repository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ItemApi for ItemService {
    #[tracing::instrument(level = "trace", skip(self, _new_item))]
    async fn create_item(&self, _new_item: &NewItem) -> Result<Item, Error> {
        let _ = self.repository.clone();

        Err(Error::Message("Todo".to_string()))
    }
}

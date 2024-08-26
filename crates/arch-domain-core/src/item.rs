use std::sync::Arc;

use arch_db::{adapters::ItemAdapter, sea_orm::prelude::Uuid, Repository};
use arch_domain_api::{Error, ItemApi};
use arch_domain_models::item::{Item, NewItem};
use arch_utils::arcbox::ArcBox;
use async_trait::async_trait;

#[derive(Clone)]
pub(crate) struct ItemService {
    repository: Arc<Repository>,
    item_adapter: ArcBox<dyn ItemAdapter>,
}

impl ItemService {
    pub(crate) fn new(repository: Arc<Repository>, item_adapter: ArcBox<dyn ItemAdapter>) -> Self {
        Self { repository, item_adapter }
    }
}

#[async_trait]
impl ItemApi for ItemService {
    #[tracing::instrument(level = "trace", skip(self, new_item))]
    async fn create_item(&self, new_item: &NewItem) -> Result<Item, Error> {
        let adapter = self.item_adapter.clone();
        let new_item = new_item.clone();
        let result: Result<Item, arch_db::Error> = self
            .repository
            .transaction(|tx| Box::pin(async move { adapter.create_item(tx, &new_item).await }))
            .await;
        match result {
            Ok(item) => Ok(item),
            Err(err) => Err(Error::DatabaseError(err)),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_item(&self, uuid: &Uuid) -> Result<Option<Item>, Error> {
        let adapter = self.item_adapter.clone();
        let uuid = *uuid;

        let result: Result<Option<Item>, arch_db::Error> = self
            .repository
            .transaction(|tx| Box::pin(async move { adapter.get_item(tx, &uuid).await }))
            .await;

        match result {
            Err(err) => Err(Error::DatabaseError(err)),
            Ok(item) => Ok(item),
        }
    }

    #[tracing::instrument(level = "trace", skip(self, text))]
    async fn update_item_text(&self, uuid: &Uuid, text: &str) -> Result<Item, Error> {
        let adapter = self.item_adapter.clone();
        let uuid = *uuid;
        let text = text.to_string();

        let result: Result<Item, arch_db::Error> = self
            .repository
            .transaction(|tx| {
                Box::pin(async move {
                    match adapter.get_item(tx, &uuid).await {
                        Ok(Some(mut item)) => {
                            item.text = text;
                            adapter.update_item(tx, &item).await
                        }
                        Ok(None) => Err(arch_db::Error::NotFound),
                        Err(err) => Err(err),
                    }
                })
            })
            .await;

        match result {
            Err(arch_db::Error::NotFound) => Err(Error::NotFound),
            Err(err) => Err(Error::DatabaseError(err)),
            Ok(item) => Ok(item),
        }
    }
}

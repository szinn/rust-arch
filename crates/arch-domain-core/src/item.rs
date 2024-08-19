use arch_db::{adapters::ItemAdapter, sea_orm::prelude::Uuid};
use arch_domain_api::{Error, ItemApi};
use arch_domain_models::item::{Item, NewItem};
use arch_utils::arcbox::ArcBox;
use async_trait::async_trait;

#[derive(Clone)]
pub(crate) struct ItemService {
    repository: ArcBox<dyn ItemAdapter>,
}

impl ItemService {
    pub(crate) fn new(repository: ArcBox<dyn ItemAdapter>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ItemApi for ItemService {
    #[tracing::instrument(level = "trace", skip(self, new_item))]
    async fn create_item(&self, new_item: &NewItem) -> Result<Item, Error> {
        match self.repository.create_item(new_item).await {
            Ok(item) => Ok(item),
            Err(err) => Err(Error::DatabaseError(err)),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_item(&self, uuid: &Uuid) -> Result<Option<Item>, Error> {
        match self.repository.get_item(uuid).await {
            Err(err) => Err(Error::DatabaseError(err)),
            Ok(item) => Ok(item),
        }
    }
}

use arch_db::adapters::ItemAdapter;
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
    #[tracing::instrument(level = "trace", skip(self, _new_item))]
    async fn create_item(&self, _new_item: &NewItem) -> Result<Item, Error> {
        let _ = self.repository.clone();

        Err(Error::Message("Todo".to_string()))
    }
}

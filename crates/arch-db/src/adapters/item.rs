use std::sync::Arc;

use arch_domain_models::item::{Item, NewItem};
use sea_orm_migration::async_trait::async_trait;

use crate::{Error, Repository};

use super::ItemAdapter;

pub(crate) struct ItemAdapterImpl {
    repository: Arc<Repository>,
}

impl ItemAdapterImpl {
    pub(crate) fn new(repository: Arc<Repository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ItemAdapter for ItemAdapterImpl {
    async fn create_item(&self, _new_item: &NewItem) -> Result<Item, Error> {
        let _ = self.repository;
        Err(Error::Message("Todo".to_string()))
    }
}

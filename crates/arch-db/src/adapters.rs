use arch_domain_models::item::{Item, NewItem};
use sea_orm::prelude::Uuid;
use sea_orm_migration::async_trait::async_trait;

use crate::Error;

pub(crate) mod item;

#[async_trait]
pub trait ItemAdapter: Send + Sync {
    async fn create_item(&self, new_item: &NewItem) -> Result<Item, Error>;
    async fn get_item(&self, uuid: &Uuid) -> Result<Option<Item>, Error>;
}

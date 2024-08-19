use arch_db::sea_orm::prelude::Uuid;
use arch_domain_models::item::{Item, NewItem, UpdateItem};
use async_trait::async_trait;

use crate::Error;

#[async_trait]
pub trait ItemApi: Send + Sync {
    async fn create_item(&self, new_item: &NewItem) -> Result<Item, Error>;
    async fn get_item(&self, uuid: &Uuid) -> Result<Option<Item>, Error>;
    async fn update_item(&self, uuid: &Uuid, update_item: &UpdateItem) -> Result<Item, Error>;
}

use arch_domain_models::item::Item;
use async_trait::async_trait;

use crate::Error;

pub struct NewItem {
    pub text: String,
}

#[async_trait]
pub trait ItemApi: Send + Sync {
    async fn create_item(&self, new_item: &NewItem) -> Result<Item, Error>;
}

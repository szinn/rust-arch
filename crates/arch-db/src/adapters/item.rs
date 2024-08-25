use std::sync::Arc;

use arch_domain_models::item::{Item, NewItem};
use sea_orm::{prelude::Uuid, ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter, Set, TransactionTrait};
use sea_orm_migration::async_trait::async_trait;

use crate::{
    entities::{items, prelude},
    Error, Repository,
};

use super::ItemAdapter;

pub(crate) struct ItemAdapterImpl {
    repository: Arc<Repository>,
}

impl ItemAdapterImpl {
    pub(crate) fn new(repository: Arc<Repository>) -> Self {
        Self { repository }
    }

    fn from_model(model: items::Model) -> Item {
        Item {
            id: model.id,
            version: model.version,
            uuid: model.uuid,
            text: model.text,
        }
    }
}

#[async_trait]
impl ItemAdapter for ItemAdapterImpl {
    #[tracing::instrument(level = "trace", skip(self, new_item))]
    async fn create_item(&self, tx: &mut DatabaseTransaction, new_item: &NewItem) -> Result<Item, Error> {
        let new_item = items::ActiveModel {
            version: Set(0),
            uuid: Set(Uuid::new_v4()),
            text: Set(new_item.text.clone()),
            ..Default::default()
        };

        let item = new_item.insert(tx).await?;

        Ok(ItemAdapterImpl::from_model(item))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_item(&self, uuid: &Uuid) -> Result<Option<Item>, Error> {
        let model = prelude::Items::find()
            .filter(items::Column::Uuid.eq(*uuid))
            .one(&self.repository.database)
            .await?;

        match model {
            None => Ok(None),
            Some(model) => Ok(Some(ItemAdapterImpl::from_model(model))),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn update_item(&self, item: &Item) -> Result<Item, Error> {
        let tx = self.repository.database.begin().await?;

        let model = prelude::Items::find().filter(items::Column::Uuid.eq(item.uuid)).one(&tx).await?;

        let model = match model {
            Some(model) => model,
            None => return Err(Error::SeaOrm(sea_orm::DbErr::RecordNotFound(item.uuid.to_string()))),
        };

        if model.version != item.version {
            return Err(Error::SeaOrm(sea_orm::DbErr::RecordNotUpdated));
        }

        let new_version = model.version + 1;
        let mut new_model: items::ActiveModel = model.into();
        new_model.text = Set(item.text.clone());
        new_model.version = Set(new_version);

        new_model.update(&tx).await?;

        let model = prelude::Items::find().filter(items::Column::Uuid.eq(item.uuid)).one(&tx).await?;

        let item = match model {
            None => return Err(Error::SeaOrm(sea_orm::DbErr::RecordNotFound(item.uuid.to_string()))),
            Some(model) => ItemAdapterImpl::from_model(model),
        };

        tx.commit().await?;

        Ok(item)
    }
}

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Items::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Items::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Items::Uuid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Items::Text).string().string_len(200).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(Items::IndexUuid.to_string())
                    .table(Items::Table)
                    .col(Items::Uuid)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(Index::drop().name(Items::IndexUuid.to_string()).to_owned()).await?;
        manager.drop_table(Table::drop().table(Items::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Items {
    Table,
    Id,
    Uuid,
    Text,
    #[sea_orm(iden = "idx_items_uuid")]
    IndexUuid,
}

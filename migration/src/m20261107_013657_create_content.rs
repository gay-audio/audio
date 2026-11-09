use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261107_013657_create_content"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("content")
                    .if_not_exists()
                    .col(pk_uuid("id").default(Expr::cust("GEN_RANDOM_UUID()")))
                    .col(string("name").unique_key())
                    .col(boolean("published").default(false))
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("content").take())
            .await
    }
}

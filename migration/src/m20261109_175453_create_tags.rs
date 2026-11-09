use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261109_175453_create_tags"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("tags")
                    .col(string("id").primary_key())
                    .col(string("long"))
                    .take(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("content_tags")
                    .primary_key(Index::create().col("tag_id").col("content_id"))
                    .col(string("tag_id"))
                    .col(uuid("content_id"))
                    .take(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from_tbl("content_tags")
                    .from_col("tag_id")
                    .to_tbl("tags")
                    .to_col("id")
                    .take(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from_tbl("content_tags")
                    .from_col("content_id")
                    .to_tbl("content")
                    .to_col("id")
                    .take(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("content_part_tags")
                    .primary_key(Index::create().col("tag_id").col("content_id").col("part"))
                    .col(string("tag_id"))
                    .col(uuid("content_id"))
                    .col(string("part"))
                    .take(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from_tbl("content_part_tags")
                    .from_col("tag_id")
                    .to_tbl("tags")
                    .to_col("id")
                    .take(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from_tbl("content_part_tags")
                    .from_col("content_id")
                    .from_col("part")
                    .to_tbl("content_part")
                    .to_col("content_id")
                    .to_col("part_type")
                    .take(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("tags").take())
            .await?;
        manager
            .drop_table(Table::drop().table("content_tags").take())
            .await?;
        Ok(())
    }
}

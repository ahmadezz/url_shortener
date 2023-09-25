use sea_orm_migration::prelude::*;

use crate::m20230925_123910_create_urls_table::Urls;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Stats::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Stats::Id).string().not_null().primary_key())
                    .col(
                        ColumnDef::new(Stats::VisitsCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("urls_stats_id_fkey")
                            .from(Urls::Table, Urls::Id)
                            .to(Stats::Table, Stats::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Stats::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Stats {
    Table,
    Id,
    VisitsCount,
}

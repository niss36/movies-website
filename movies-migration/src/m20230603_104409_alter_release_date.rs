use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table::Movie;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Movie::Table)
                    .modify_column(
                        ColumnDef::new(Movie::ReleaseDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Movie::Table)
                    .modify_column(ColumnDef::new(Movie::ReleaseDate).date_time().not_null())
                    .to_owned(),
            )
            .await
    }
}

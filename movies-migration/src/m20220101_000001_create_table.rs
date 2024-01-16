use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Movie::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Movie::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Movie::Title).string().not_null())
                    .col(ColumnDef::new(Movie::ReleaseDate).date_time().not_null())
                    .col(ColumnDef::new(Movie::PosterUrl).string().not_null())
                    .col(ColumnDef::new(Movie::Description).text().not_null())
                    .col(ColumnDef::new(Movie::Rating).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Movie::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Movie {
    Table,
    Id,
    Title,
    ReleaseDate,
    PosterUrl,
    Description,
    Rating,
}

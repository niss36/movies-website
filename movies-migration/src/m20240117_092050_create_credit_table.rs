use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

use crate::{m20220101_000001_create_table::Movie, m20240117_090322_create_person_table::Person};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(CreditType::Table)
                    .values([
                        CreditType::Director,
                        CreditType::Producer,
                        CreditType::Actor,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Credit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Credit::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Credit::MovieId).integer().not_null())
                    .col(ColumnDef::new(Credit::PersonId).integer().not_null())
                    .col(
                        ColumnDef::new(Credit::Type)
                            .enumeration(
                                CreditType::Table,
                                [
                                    CreditType::Director,
                                    CreditType::Producer,
                                    CreditType::Actor,
                                ],
                            )
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from_tbl(Credit::Table)
                    .to_tbl(Movie::Table)
                    .from_col(Credit::MovieId)
                    .to_col(Movie::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from_tbl(Credit::Table)
                    .to_tbl(Person::Table)
                    .from_col(Credit::PersonId)
                    .to_col(Person::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Credit::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(CreditType::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Credit {
    Table,
    Id,
    MovieId,
    PersonId,
    Type,
}

#[derive(DeriveIden)]
pub enum CreditType {
    Table,
    Director,
    Producer,
    Actor,
}

pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230603_104409_alter_release_date;
mod m20240117_090322_create_person_table;
mod m20240117_092050_create_credit_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230603_104409_alter_release_date::Migration),
            Box::new(m20240117_090322_create_person_table::Migration),
            Box::new(m20240117_092050_create_credit_table::Migration),
        ]
    }
}

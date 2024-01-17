use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Schema};

pub async fn prepare_test_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect("sqlite::memory:").await?;

    let schema = Schema::new(DbBackend::Sqlite);

    let statements = [
        schema.create_table_from_entity(movies_entity::prelude::Movie),
        schema.create_table_from_entity(movies_entity::prelude::Person),
        schema.create_table_from_entity(movies_entity::prelude::Credit),
    ];

    for statement in statements {
        db.execute(db.get_database_backend().build(&statement))
            .await?;
    }

    Ok(db)
}

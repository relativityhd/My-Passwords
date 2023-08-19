use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use sea_orm_migration::prelude::*;

use migration::{Migrator, MigratorTrait};

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let db_url = "postgres://tobias:tobias@localhost/my_passwords";
    let options = ConnectOptions::new(db_url.to_owned());
    let db = Database::connect(options).await?;

    let schema_manager = SchemaManager::new(&db);
    let refresh = Migrator::up(&db, None).await;
    if refresh.is_err() {
        println!("Error refreshing database: {:?}", refresh.err());
    }
    assert!(schema_manager.has_table("user").await?);
    assert!(schema_manager.has_table("bucket").await?);
    assert!(schema_manager.has_table("institution").await?);
    assert!(schema_manager.has_table("account").await?);

    Ok(db)
}

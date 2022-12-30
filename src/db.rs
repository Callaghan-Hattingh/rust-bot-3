use sea_orm::*;
use sea_orm_migration::prelude::*;

use crate::migrator::Migrator;

const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";

pub async fn db_migration() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let db = &db;

    let schema_manager = SchemaManager::new(db);

    Migrator::refresh(db).await?;
    assert!(schema_manager.has_table("trade").await?);
    assert!(schema_manager.has_table("lot").await?);

    Ok(())
}

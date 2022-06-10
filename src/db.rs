use std::{
    ops::DerefMut,
    path::{Path, PathBuf},
};

use sqlx::{
    migrate::{Migrate, MigrateDatabase, Migrator},
    postgres::{PgConnectOptions, PgConnection, PgPoolOptions, Postgres},
    Error as SqlxError, Pool, Type,
};

#[derive(Debug, Clone)]
pub struct Db(pub Pool<Postgres>);
impl Db {
    pub async fn new() -> sqlx::Result<Self> {
        let db = PgPoolOptions::new()
            .max_connections(50)
            .connect(&Self::db_url())
            .await?;
        Self::migrate(&db).await?;
        Ok(Self(db))
    }

    pub fn db_url() -> String {
        std::env::var("DATABASE_URL").expect("Must have DATABASE_URL envvar set")
    }
    pub async fn migrate(db: &Pool<Postgres>) -> sqlx::Result<()> {
        Migrator::new(PathBuf::from("data/migrate"))
            .await?
            .run(db)
            .await?;
        Ok(())
    }
}

impl std::ops::Deref for Db {
    type Target = Pool<Postgres>;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

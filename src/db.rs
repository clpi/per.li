use std::{
    ops::DerefMut,
    path::{Path, PathBuf},
};
use std::

use axum_database_sessions::{
    AxumDatabasePool, AxumSessionConfig, AxumSessionMode, AxumSessionStore,
};
use deadpool::{
    managed::Manager as CoreManager,
    Status,
};
use serde::{Serialize, Deserialize};
use tokio_postgres::{NoTls, GenericClient, connect, Error, AsyncMessage, Connection};
// use deadpool_postgres::{
//     Manager, Pool, PoolError, Runtime,
//     Client, ManagerConfig, Config,
// };
use sqlx::{
    migrate::{Migrate, MigrateDatabase, Migrator},
    postgres::{PgConnectOptions, PgConnection, PgPoolOptions, Postgres},
    Error as SqlxError, Pool, Type,
};
// pub type ConfigResult<T> = Result<T, deadpool_postgres::ConfigError>;
// pub type ConnectResult<T> = Result<T, tokio_postgres::error::Error>;

// #[derive(Serialize, Deserialize)]
// pub struct DbCfg {
//     pg: deadpool_postgres::Config,
// }
// impl DbCfg {
//     pub async fn new() -> ConnectResult<Self> {
//         let (client, connection) = connect(&Self::db_url(), NoTls).await?;
//         tokio::spawn(async move {
//             if let Err(e) = connection.await {
//                 eprintln!("Conn error: {}", e);
//             }
//         });
//         Ok(Self{ pg: connection })
//
//     }
//     pub fn from_env() -> ConfigResult<Self> {
//         let rt = Runtime::Tokio1;
//         let mut c = Config::default();
//         c.host = Some("bit.io".to_string());
//         c.user = Some("clpi".into());
//         c.dbname = Some("cdb".into());
//         return Ok(Self{ pg: c})
//
//     }
//
//     pub fn db_url() -> String {
//         std::env::var("DATABASE_URL").expect("Must have DATABASE_URL envvar set")
//     }
// }
#[derive(Debug, Clone)]
pub struct Db(pub Pool<Postgres>);

// #[async_trait::async_trait]
// impl CoreManager for Manager {
//     type Type =
// }
impl Db {
    pub async fn new() -> sqlx::Result<Self> {
        let db = PgPoolOptions::new()
            .max_connections(50)
            .connect(&Self::db_url())
            .await?;
        // Self::migrate(&db).await?;
        Ok(Self(db))
    }

    pub async fn up(&mut self) -> sqlx::Result<()> {
        Ok(())
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
impl Into<AxumDatabasePool> for Db {
    fn into(self) -> AxumDatabasePool {
        AxumDatabasePool::from(self.0)
    }
}
impl Into<AxumSessionStore> for Db {
    fn into(self) -> AxumSessionStore {
        let dbpool: AxumDatabasePool = self.into();
        let sessions = AxumSessionConfig::default()
            .with_secure(true)
            .with_table_name("User");
        AxumSessionStore::new(Some(dbpool), sessions)
    }
}

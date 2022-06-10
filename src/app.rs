use super::{auth, code, db, lang, rt, user};
use crate::models::user::User;
use async_session::{MemoryStore, Session, SessionStore};
use axum::{Extension, Router};
use axum_database_sessions::{
    AxumDatabasePool, AxumSession, AxumSessionConfig, AxumSessionLayer, AxumSessionStore,
};
use axum_sessions_auth::{
    Auth, AuthSession, AuthSessionLayer, AuthSessionService, Authentication, HasPermission,
};
use sqlx::{
    postgres::{PgConnectionInfo, PgPoolOptions},
    ConnectOptions,
};
use std::convert::{TruFrom, TryInfo};
use std::{net::SocketAddr, ops::Deref};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct App {
    pub routes: Router,
    pub db: db::Db,
    pub addr: SocketAddr,
}

impl App {
    pub async fn init() -> Result<Self, axum::Error> {
        Self::init_tracing();
        let db: db::Db = db::Db::new().await.expect("Could not get DB conn");
        tracing::debug!("Got db conn!");

        let mem_store = MemoryStore::new();
        let sessions = AxumSessionConfig::default()
            .with_secure(true)
            .with_table_name("User");
        // let pgpool = |db| { db.deref().clone() };
        let dbpool = Some(AxumDatabasePool(db.clone().deref()));
        let sess_store = AxumSessionStore::new(dbpool, sessions);

        let app = Router::new()
            .nest("/user", user::routes())
            .nest("/auth", auth::routes())
            .nest("/chat", rt::routes())
            .nest("/languages", lang::routes())
            .nest("/code", code::routes())
            .layer(Extension(tower_cookies::CookieManagerLayer::new()))
            .layer(Extension(mem_store))
            .layer(AxumSessionLayer::new(sess_store))
            .layer(AuthSessionLayer::<User>::new(dbpool, None));

        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
        tracing::debug!("Listening on {}", addr);
        Ok(Self {
            routes: app,
            addr,
            db,
        })
    }

    pub async fn serve(self: Self) -> Result<(), axum::Error> {
        axum::Server::bind(&self.addr)
            .serve(self.routes.into_make_service())
            .await
            .expect("Couldn't bind to addr");
        Ok(())
    }

    pub fn init_tracing() {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "example_oauth=debug".to_string()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}

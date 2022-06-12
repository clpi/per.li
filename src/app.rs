use super::{auth, code, db, lang, rt, user};
use crate::models::user::User;
use async_session::{MemoryStore, Session, SessionStore};
use axum::{Extension, Router, routing::{on, any, get, put, post, IntoMakeService}, response::IntoResponse, Json};
use axum_database_sessions::{
    AxumDatabasePool, AxumSession, AxumSessionConfig, AxumSessionLayer, AxumSessionStore,
};
use axum_sessions_auth::{
    Auth, AuthSession, AuthSessionLayer, AuthSessionService, Authentication, HasPermission,
};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgConnectionInfo, PgPoolOptions},
    ConnectOptions,
};
use tower_http::add_extension::AddExtensionLayer;
use std::{net::SocketAddr, ops::Deref, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use time::serde::timestamp;

pub struct App {
    pub routes: Router,
    // pub db: db::Db,
    pub addr: SocketAddr,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    active: Vec<String>,
    #[serde(
        default = "time::OffsetDateTime::now_utc",
        with = "timestamp",
    )]
    started: time::OffsetDateTime,
}

impl Default for State {
    fn default() -> Self {
        State {
            started: time::OffsetDateTime::now_utc(),
            active: Vec::new(),
        }
    }
}

impl App {
    pub async fn init() -> Result<Self, Box<dyn std::error::Error>> {
        Self::init_tracing();
        let db: db::Db = db::Db::new().await.expect("Could not get DB conn");
        tracing::debug!("Got db conn!");

        let mem_store = MemoryStore::new();
        let sessions: AxumSessionStore = db.clone().into();
        let state: Arc<State> = Arc::new(State::default());
        // let pgpool = |db| { db.deref().clone() };

        let app = Router::new()
            .route("/", get(Root::index))
            .route("/status", get(Self::status))
            .nest("/user", user::routes())
            .nest("/auth", auth::routes())
            .nest("/chat", rt::routes())
            .nest("/languages", lang::routes())
            .nest("/code", code::routes())
            .layer(Extension(tower_cookies::CookieManagerLayer::new()))
            .layer(AddExtensionLayer::new(state))
            .layer(Extension(mem_store));
            // .layer(AxumSessionLayer::new(sessions));
        // .layer(AuthSessionLayer::<User>::new(Some(db.clone().into()), None));

        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
        tracing::debug!("Listening on {}", addr);
        Ok(Self {
            routes: app,
            addr,
            // db,
        })
    }

    pub async fn status(
        Extension(state): Extension<Arc<State>>,
    ) -> impl IntoResponse {
        Json(state)
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

pub struct Root {
}
impl Root {
    pub async fn index() -> &'static str {
        "Hi world"
    }
}

pub struct Context {

}

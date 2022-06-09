#![allow(unused)]

pub mod app;
pub mod auth;
pub mod code;
pub mod config;
pub mod db;
pub mod lang;
pub mod rt;
pub mod user;

use async_session::{MemoryStore, Session, SessionStore};
use axum::{
    self,
    body::{Body, Bytes, Full},
    handler::{Handler, IntoService, Layered},
    response::{Html, Json, Response, Result},
    routing::{delete, get, post, put},
    Extension, Router, Server,
};
use tonic::{
    codegen::{ok, Context, InterceptedService},
    server::{Grpc, StreamingService, UnaryService},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type AxumResult<T> = Result<T, axum::Error>;

#[tokio::main]
async fn main() -> AxumResult<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_oauth=debug".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let store = MemoryStore::new();

    let db = db::Db::new().await.expect("Could not get DB conn");
    tracing::debug!("Got db conn!");
    let app = Router::new()
        .nest("/user", user::routes())
        .nest("/auth", auth::routes())
        .nest("/chat", rt::routes())
        .nest("/languages", lang::routes())
        .nest("/code", code::routes())
        .layer(Extension(db))
        .layer(Extension(tower_cookies::CookieManagerLayer::new()))
        .layer(Extension(store));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Couldn't bind to addr");

    Ok(())
}

use axum::{
    response::{Html, IntoResponse},
    routing::{get, post, Router},
    extract::{
        Extension, ws::{Message, WebSocket, WebSocketUpgrade},
    },
};
use futures::{sink::SinkExt, stream::StreamExt};
use indexmap::{IndexSet, IndexMap};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone)]
pub struct Context {
    users: IndexSet<String>,
    tx: broadcast::Sender<String>,
}
impl Context {
    pub fn new() -> Self {
        let (tx, mut _rx1) = broadcast::channel(16);
        let mut _rx2 = tx.subscribe();
        Self { users: IndexSet::<String>::new(), tx}
    }
}

pub fn routes() -> Router {
    Router::default()
        // .layer(Context::new())
}

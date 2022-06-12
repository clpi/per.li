pub mod chat;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    headers::{self, UserAgent},
    response::IntoResponse,
    routing::get_service,
    routing::{delete, get, post, put},
    Router, TypedHeader,
};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

pub fn routes() -> Router {
    Router::default().route("/", get(ws_handler)).layer(
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)),
    )
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    println!("Client sent str: {:?}", t);
                }
                Message::Binary(b) => {
                    println!("Client sent binary: {:?}", b);
                }
                Message::Ping(t) => {
                    println!("Client sent ping: {:?}", t);
                }
                Message::Pong(t) => {
                    println!("Client sent pong: {:?}", t);
                }
                Message::Close(t) => {
                    println!("Client sent close: {:?}", t);
                    return;
                }
            }
        } else {
            println!("Client disconnected");
            return;
        }
    }
    loop {
        if socket
            .send(Message::Text(String::from("Hi")))
            .await
            .is_err()
        {
            println!("Client disconnected from saying hi");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

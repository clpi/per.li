use axum::{
    response::{Html, IntoResponse, Json, Response, Result},
    routing::{delete, get, head, options, post, put, MethodFilter, MethodRouter, Route, Router},
    Extension,
};
use tonic::server::Grpc;

pub fn routes() -> axum::Router {
    Router::new()
        .route("/", get(users))
}

pub async fn users() -> Response {
    Html("<h1>User index</h1>").into_response()
}

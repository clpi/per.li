use axum::{
    response::{Html, IntoResponse, Json, Response, Result},
    routing::{delete, get, head, options, post, put, MethodFilter, MethodRouter, Route, Router},
    Extension,
};
use tonic::server::Grpc;

pub fn routes() -> axum::Router {
    Router::new().route("/index", get(get_user_index))
}

pub async fn get_user_index() -> Response {
    Html("<h1>User index</h1>").into_response()
}

use axum::{
    response::{Html, IntoResponse, Json, Redirect, Response, Result},
    routing::{delete, get, on, patch, post, put},
    BoxError, Error, Router,
};

pub fn routes() -> axum::Router {
    Router::default()
        .route("/login", post(login))
        .route("/signup", post(signup))
}

pub async fn login() -> Response {
    Html("<h1>Login<h1>").into_response()
}

pub async fn signup() -> Response {
    Html("<h1>Signup<h1>").into_response()
}

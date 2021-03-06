#![allow(unused)]
#![feature(ready_macro)]

pub mod app;
pub mod auth;
pub mod code;
pub mod config;
pub mod db;
pub mod lang;
pub mod models;
pub mod rt;
pub mod user;

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

pub type AxumResult<T> = Result<T, axum::Error>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::init().await?;
    app.serve().await?;

    Ok(())
}

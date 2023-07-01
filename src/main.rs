use std::{net::SocketAddr, sync::Arc};

use axum::{extract::State, response::Response, routing::get, Router};
use config::{check_env, port};
use dotenvy::dotenv;
use error::Error;

use prisma::PrismaClient;
use response::Web;
use routes::{callback::oauth_callback, login::login, profile::profile};

mod config;
mod error;
mod routes;
mod utils;

#[allow(warnings)]
mod prisma;

mod model;
#[allow(warnings)]
mod response;

#[derive(Clone)]
pub struct GlobalState {
    pub db: Arc<PrismaClient>,
}

pub async fn health_check(State(GlobalState { .. }): State<GlobalState>) -> WebResult {
    Ok(Web::ok("Server is up", ()))
}

type WebResult = std::result::Result<Response, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    check_env();

    let prisma = PrismaClient::_builder()
        .build()
        .await
        .expect("Failed to connect to database");

    let db = Arc::new(prisma);

    let global_state = GlobalState { db };

    let routes = Router::new()
        .route("/", get(health_check))
        .merge(login())
        .merge(oauth_callback())
        .merge(profile())
        .with_state(global_state);

    let port = port();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("Failed to start server")
}

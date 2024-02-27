use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use serde::Deserialize;
use sqlx::{Acquire, Connection, SqliteConnection, SqlitePool};
use tokio::net::TcpListener;

use axum::{
    body::Body, extract::State, http::StatusCode, response::IntoResponse,
    routing::post, Form, Router,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let database_pool = SqlitePool::connect("db.sqlite").await.unwrap();
    let routes = Router::new()
        .route("/login", post(login_handler))
        .with_state(AppState {
            pool: database_pool,
        });

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

async fn login_handler(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> Result<impl IntoResponse, Infallible> {

    sqlx::query!("SELECT DISTINCT username FROM users WHERE password = ?").fetch_one(&state.pool).await?;

    Ok((
        StatusCode::TEMPORARY_REDIRECT,
        [("Location", "/")],
        Body::empty(),
    ))
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

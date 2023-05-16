use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::{routing::get, Router, Server};
use core::sea_orm::{Database, DatabaseConnection};
use entity::movie;
use migration::{DbErr, Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv()?;
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    let app = Router::new()
        .route("/movies", get(list_movies).post(create_movie))
        .route(
            "/movies/:id",
            get(get_movie).delete(delete_movie).put(update_movie),
        )
        .with_state(state);

    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FlashData {
    kind: String,
    message: String,
}

async fn list_movies(
    state: State<AppState>,
) -> Result<Json<Vec<movie::Model>>, (StatusCode, &'static str)> {
    let movies = core::get_all_movies(&state.conn)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?;

    Ok(Json(movies))
}

async fn create_movie(
    state: State<AppState>,
    Json(data): Json<movie::Model>,
) -> Result<Json<movie::Model>, (StatusCode, &'static str)> {
    let created_movie = core::create_movie(&state.conn, data)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?;

    Ok(Json(created_movie))
}

async fn get_movie(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<movie::Model>, (StatusCode, &'static str)> {
    let movie = core::get_movie(&state.conn, id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Movie not found"))?;

    Ok(Json(movie))
}

async fn delete_movie(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<()>, (StatusCode, &'static str)> {
    core::delete_movie(&state.conn, id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?;

    Ok(Json(()))
}

async fn update_movie(
    state: State<AppState>,
    Path(id): Path<i32>,
    Json(data): Json<movie::Model>,
) -> Result<Json<movie::Model>, (StatusCode, String)> {
    let movie = core::update_movie(&state.conn, id, data)
        .await
        .map_err(|err| match err {
            DbErr::RecordNotFound(message) => (StatusCode::NOT_FOUND, message),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into()),
        })?;

    Ok(Json(movie))
}

pub fn main() {
    if let Err(err) = start() {
        println!("Error: {err}");
    }
}

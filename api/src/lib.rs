use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::{routing::get, Router, Server};
use core::sea_orm::{Database, DatabaseConnection, DeleteResult};
use core::PartialMovie;
use entity::movie;
use macros::IntoResponse;
use migration::{DbErr, Migrator, MigratorTrait};
use responses::{DatabaseError, NoContent, NotFound};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{env, net::SocketAddr};

mod responses;

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
            get(get_movie)
                .delete(delete_movie)
                .put(update_movie)
                .patch(patch_movie),
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

#[derive(IntoResponse)]
enum ListMoviesResponses {
    Success(#[json] Vec<movie::Model>),
    DatabaseError(DatabaseError),
}

async fn list_movies(state: State<AppState>) -> ListMoviesResponses {
    match core::get_all_movies(&state.conn).await {
        Ok(movies) => ListMoviesResponses::Success(movies),
        Err(_) => ListMoviesResponses::DatabaseError(DatabaseError),
    }
}

#[derive(IntoResponse)]
enum CreateMovieResponses {
    Success(#[json] movie::Model),
    DatabaseError(DatabaseError),
}

async fn create_movie(
    state: State<AppState>,
    Json(data): Json<movie::Model>,
) -> CreateMovieResponses {
    match core::create_movie(&state.conn, data).await {
        Ok(created_movie) => CreateMovieResponses::Success(created_movie),
        Err(_) => CreateMovieResponses::DatabaseError(DatabaseError),
    }
}

#[derive(IntoResponse)]
enum GetMovieResponses {
    Success(#[json] movie::Model),
    NotFound(NotFound),
    DatabaseError(DatabaseError),
}

async fn get_movie(state: State<AppState>, Path(id): Path<i32>) -> GetMovieResponses {
    match core::get_movie(&state.conn, id).await {
        Ok(Some(movie)) => GetMovieResponses::Success(movie),
        Ok(None) => {
            GetMovieResponses::NotFound(NotFound(format!("Movie with id `{id}` not found")))
        }
        Err(_) => GetMovieResponses::DatabaseError(DatabaseError),
    }
}

#[derive(IntoResponse)]
enum DeleteMovieResponses {
    Success(NoContent),
    NotFound(NotFound),
    DatabaseError(DatabaseError),
}

async fn delete_movie(state: State<AppState>, Path(id): Path<i32>) -> DeleteMovieResponses {
    match core::delete_movie(&state.conn, id).await {
        Ok(DeleteResult { rows_affected }) if rows_affected > 0 => {
            DeleteMovieResponses::Success(NoContent)
        }
        Ok(_) => {
            DeleteMovieResponses::NotFound(NotFound(format!("Movie with id `{id}` not found")))
        }
        Err(_) => DeleteMovieResponses::DatabaseError(DatabaseError),
    }
}

#[derive(IntoResponse)]
enum UpdateMovieResponses {
    Success(#[json] movie::Model),
    NotFound(NotFound),
    DatabaseError(DatabaseError),
}

async fn update_movie(
    state: State<AppState>,
    Path(id): Path<i32>,
    Json(data): Json<movie::Model>,
) -> UpdateMovieResponses {
    match core::update_movie(&state.conn, id, data).await {
        Ok(movie) => UpdateMovieResponses::Success(movie),
        Err(DbErr::RecordNotFound(message)) => UpdateMovieResponses::NotFound(NotFound(message)),
        Err(_) => UpdateMovieResponses::DatabaseError(DatabaseError),
    }
}

async fn patch_movie(
    state: State<AppState>,
    Path(id): Path<i32>,
    Json(data): Json<PartialMovie>,
) -> UpdateMovieResponses {
    match core::update_movie_partial(&state.conn, id, data).await {
        Ok(movie) => UpdateMovieResponses::Success(movie),
        Err(DbErr::RecordNotFound(message)) => UpdateMovieResponses::NotFound(NotFound(message)),
        Err(_) => UpdateMovieResponses::DatabaseError(DatabaseError),
    }
}

pub fn main() {
    if let Err(err) = start() {
        println!("Error: {err}");
    }
}

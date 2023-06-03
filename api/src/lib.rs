use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::{routing::get, Router, Server};
use core::sea_orm::{Database, DatabaseConnection, DeleteResult};
use core::PartialMovie;
use entity::movie::Model as Movie;
use macros::IntoResponse;
use migration::{DbErr, Migrator, MigratorTrait};
use responses::{DatabaseError, NoContent, NotFound};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{env, net::SocketAddr};
use utoipa::{IntoResponses, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust Movies",
        contact()
    ),
    paths(
        movies::list_movies,
        movies::create_movie,
        movies::get_movie,
        movies::delete_movie,
        movies::update_movie,
        movies::patch_movie,
    ),
    components(schemas(Movie, PartialMovie), responses(NoContent, NotFound, DatabaseError)),
    tags((name = "movies", description = "Rust Movies API"))
)]
pub struct ApiDocs;

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

    use movies::*;
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDocs::openapi()))
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

mod movies {
    use super::*;

    #[derive(IntoResponse, IntoResponses)]
    pub(super) enum ListMoviesResponses {
        #[response(status = OK)]
        Success(#[json] Vec<Movie>),

        #[response(status = INTERNAL_SERVER_ERROR)]
        DatabaseError(#[ref_response] DatabaseError),
    }

    #[utoipa::path(get, path = "/movies", responses(ListMoviesResponses))]
    pub(super) async fn list_movies(state: State<AppState>) -> ListMoviesResponses {
        match core::get_all_movies(&state.conn).await {
            Ok(movies) => ListMoviesResponses::Success(movies),
            Err(_) => ListMoviesResponses::DatabaseError(DatabaseError),
        }
    }

    #[derive(IntoResponse, IntoResponses)]
    pub(super) enum CreateMovieResponses {
        #[response(status = OK)]
        Success(#[json] Movie),

        #[response(status = INTERNAL_SERVER_ERROR)]
        DatabaseError(#[ref_response] DatabaseError),
    }

    #[utoipa::path(
        post,
        path = "/movies",
        request_body = Movie,
        responses(CreateMovieResponses),
    )]
    pub(super) async fn create_movie(
        state: State<AppState>,
        Json(data): Json<Movie>,
    ) -> CreateMovieResponses {
        match core::create_movie(&state.conn, data).await {
            Ok(created_movie) => CreateMovieResponses::Success(created_movie),
            Err(_) => CreateMovieResponses::DatabaseError(DatabaseError),
        }
    }

    #[derive(IntoResponse, IntoResponses)]
    pub(super) enum GetMovieResponses {
        #[response(status = OK)]
        Success(#[json] Movie),

        #[response(status = NOT_FOUND)]
        NotFound(#[ref_response] NotFound),

        #[response(status = INTERNAL_SERVER_ERROR)]
        DatabaseError(#[ref_response] DatabaseError),
    }

    #[utoipa::path(
        get,
        path = "/movies/{id}",
        params(
            ("id", description = "Movie id")
        ),
        responses(GetMovieResponses),
    )]
    pub(super) async fn get_movie(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> GetMovieResponses {
        match core::get_movie(&state.conn, id).await {
            Ok(Some(movie)) => GetMovieResponses::Success(movie),
            Ok(None) => {
                GetMovieResponses::NotFound(NotFound(format!("Movie with id `{id}` not found")))
            }
            Err(_) => GetMovieResponses::DatabaseError(DatabaseError),
        }
    }

    #[derive(IntoResponse, IntoResponses)]
    pub(super) enum DeleteMovieResponses {
        #[response(status = NO_CONTENT)]
        Success(#[ref_response] NoContent),

        #[response(status = NOT_FOUND)]
        NotFound(#[ref_response] NotFound),

        #[response(status = INTERNAL_SERVER_ERROR)]
        DatabaseError(#[ref_response] DatabaseError),
    }

    #[utoipa::path(
        delete,
        path = "/movies/{id}",
        params(
            ("id", description = "Movie id")
        ),
        responses(DeleteMovieResponses),
    )]
    pub(super) async fn delete_movie(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> DeleteMovieResponses {
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

    #[derive(IntoResponse, IntoResponses)]
    pub(super) enum UpdateMovieResponses {
        #[response(status = OK)]
        Success(#[json] Movie),

        #[response(status = NOT_FOUND)]
        NotFound(#[ref_response] NotFound),

        #[response(status = INTERNAL_SERVER_ERROR)]
        DatabaseError(#[ref_response] DatabaseError),
    }

    #[utoipa::path(
        put,
        path = "/movies/{id}",
        params(
            ("id", description = "Movie id")
        ),
        request_body = Movie,
        responses(UpdateMovieResponses),
    )]
    pub(super) async fn update_movie(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(data): Json<Movie>,
    ) -> UpdateMovieResponses {
        match core::update_movie(&state.conn, id, data).await {
            Ok(movie) => UpdateMovieResponses::Success(movie),
            Err(DbErr::RecordNotFound(message)) => {
                UpdateMovieResponses::NotFound(NotFound(message))
            }
            Err(_) => UpdateMovieResponses::DatabaseError(DatabaseError),
        }
    }

    #[utoipa::path(
        patch,
        path = "/movies/{id}",
        params(
            ("id", description = "Movie id")
        ),
        request_body = PartialMovie,
        responses(UpdateMovieResponses),
    )]
    pub(super) async fn patch_movie(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(data): Json<PartialMovie>,
    ) -> UpdateMovieResponses {
        match core::update_movie_partial(&state.conn, id, data).await {
            Ok(movie) => UpdateMovieResponses::Success(movie),
            Err(DbErr::RecordNotFound(message)) => {
                UpdateMovieResponses::NotFound(NotFound(message))
            }
            Err(_) => UpdateMovieResponses::DatabaseError(DatabaseError),
        }
    }
}

pub fn main() {
    if let Err(err) = start() {
        println!("Error: {err}");
    }
}

use core::sea_orm::{DatabaseConnection, DeleteResult};
use core::PartialMovie;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use entity::movie::Model as Movie;
use macros::IntoResponse;
use migration::DbErr;
use utoipa::{IntoResponses, OpenApi};

use crate::responses::{DatabaseError, NoContent, NotFound};

#[derive(OpenApi)]
#[openapi(
    paths(
        list_movies,
        create_movie,
        get_movie,
        delete_movie,
        update_movie,
        patch_movie,
    ),
    components(schemas(Movie, PartialMovie), responses(NoContent, NotFound, DatabaseError)),
    tags((name = "movies", description = "Rust Movies API"))
)]
pub struct MoviesApiDocs;

pub fn movies_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(list_movies).post(create_movie))
        .route(
            "/:id",
            get(get_movie)
                .delete(delete_movie)
                .put(update_movie)
                .patch(patch_movie),
        )
        .with_state(MoviesState { db })
}

#[derive(Clone)]
struct MoviesState {
    db: DatabaseConnection,
}

#[derive(IntoResponse, IntoResponses)]
enum ListMoviesResponses {
    #[response(status = OK)]
    Success(#[json] Vec<Movie>),

    #[response(status = INTERNAL_SERVER_ERROR)]
    DatabaseError(#[ref_response] DatabaseError),
}

#[utoipa::path(get, path = "/movies", responses(ListMoviesResponses))]
async fn list_movies(state: State<MoviesState>) -> ListMoviesResponses {
    match core::get_all_movies(&state.db).await {
        Ok(movies) => ListMoviesResponses::Success(movies),
        Err(_) => ListMoviesResponses::DatabaseError(DatabaseError),
    }
}

#[derive(IntoResponse, IntoResponses)]
enum CreateMovieResponses {
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
async fn create_movie(state: State<MoviesState>, Json(data): Json<Movie>) -> CreateMovieResponses {
    match core::create_movie(&state.db, data).await {
        Ok(created_movie) => CreateMovieResponses::Success(created_movie),
        Err(_) => CreateMovieResponses::DatabaseError(DatabaseError),
    }
}

#[derive(IntoResponse, IntoResponses)]
enum GetMovieResponses {
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
async fn get_movie(state: State<MoviesState>, Path(id): Path<i32>) -> GetMovieResponses {
    match core::get_movie(&state.db, id).await {
        Ok(Some(movie)) => GetMovieResponses::Success(movie),
        Ok(None) => {
            GetMovieResponses::NotFound(NotFound(format!("Movie with id `{id}` not found")))
        }
        Err(_) => GetMovieResponses::DatabaseError(DatabaseError),
    }
}

#[derive(IntoResponse, IntoResponses)]
enum DeleteMovieResponses {
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
async fn delete_movie(state: State<MoviesState>, Path(id): Path<i32>) -> DeleteMovieResponses {
    match core::delete_movie(&state.db, id).await {
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
enum UpdateMovieResponses {
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
async fn update_movie(
    state: State<MoviesState>,
    Path(id): Path<i32>,
    Json(data): Json<Movie>,
) -> UpdateMovieResponses {
    match core::update_movie(&state.db, id, data).await {
        Ok(movie) => UpdateMovieResponses::Success(movie),
        Err(DbErr::RecordNotFound(message)) => UpdateMovieResponses::NotFound(NotFound(message)),
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
async fn patch_movie(
    state: State<MoviesState>,
    Path(id): Path<i32>,
    Json(data): Json<PartialMovie>,
) -> UpdateMovieResponses {
    match core::update_movie_partial(&state.db, id, data).await {
        Ok(movie) => UpdateMovieResponses::Success(movie),
        Err(DbErr::RecordNotFound(message)) => UpdateMovieResponses::NotFound(NotFound(message)),
        Err(_) => UpdateMovieResponses::DatabaseError(DatabaseError),
    }
}

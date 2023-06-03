use ::entity::movie;
use sea_orm::prelude::DateTime;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub async fn create_movie(db: &DbConn, data: movie::Model) -> Result<movie::Model, DbErr> {
    let active_movie = movie::ActiveModel {
        title: Set(data.title),
        release_date: Set(data.release_date),
        poster_url: Set(data.poster_url),
        description: Set(data.description),
        rating: Set(data.rating),
        ..Default::default()
    };

    active_movie.save(db).await?.try_into()
}

pub async fn delete_movie(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
    movie::Entity::delete_by_id(id).exec(db).await
}

pub async fn update_movie(db: &DbConn, id: i32, data: movie::Model) -> Result<movie::Model, DbErr> {
    let active_movie: movie::ActiveModel = movie::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound(format!(
            "Movie with id {id} not found"
        )))?
        .into();

    movie::ActiveModel {
        id: active_movie.id,
        title: Set(data.title),
        release_date: Set(data.release_date),
        poster_url: Set(data.poster_url),
        description: Set(data.description),
        rating: Set(data.rating),
    }
    .update(db)
    .await
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PartialMovie {
    pub title: Option<String>,
    pub release_date: Option<DateTime>,
    pub poster_url: Option<String>,
    pub description: Option<String>,
    pub rating: Option<i32>,
}

fn option_into_active_value<T>(maybe_value: Option<T>) -> ActiveValue<T>
where
    T: Into<Value>,
{
    match maybe_value {
        Some(value) => Set(value),
        None => NotSet,
    }
}

pub async fn update_movie_partial(
    db: &DbConn,
    id: i32,
    data: PartialMovie,
) -> Result<movie::Model, DbErr> {
    let active_movie: movie::ActiveModel = movie::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound(format!(
            "Movie with id {id} not found"
        )))?
        .into();

    movie::ActiveModel {
        id: active_movie.id,
        title: option_into_active_value(data.title),
        release_date: option_into_active_value(data.release_date),
        poster_url: option_into_active_value(data.poster_url),
        description: option_into_active_value(data.description),
        rating: option_into_active_value(data.rating),
    }
    .update(db)
    .await
}

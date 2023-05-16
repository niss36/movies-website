use ::entity::movie;
use sea_orm::*;

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

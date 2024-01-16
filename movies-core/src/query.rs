use ::movies_entity::movie;
use sea_orm::*;

pub async fn get_all_movies(db: &DbConn) -> Result<Vec<movie::Model>, DbErr> {
    movie::Entity::find().all(db).await
}

pub async fn get_movie(db: &DbConn, id: i32) -> Result<Option<movie::Model>, DbErr> {
    movie::Entity::find_by_id(id).one(db).await
}

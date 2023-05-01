use ::entity::movie;
use sea_orm::*;

pub async fn get_all_movies(db: &DbConn) -> Result<Vec<movie::Model>, DbErr> {
    movie::Entity::find().all(db).await
}

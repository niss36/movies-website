use chrono::{TimeZone, Utc};
use movies_core::{create_movie, get_all_movies};
use movies_entity::movie::Model;
use sea_orm::DbErr;
use setup::prepare_test_db;

mod setup;

fn assert_eq_ignore_id(mut this: Model, that: Model) {
    this.id = that.id;

    assert_eq!(this, that);
}

#[tokio::test]
async fn create_and_list_movies() -> Result<(), DbErr> {
    // arrange
    let db = prepare_test_db().await?;

    let star_wars = Model {
        id: 0,
        title: "Star Wars: Episode IV - A New Hope".to_owned(),
        release_date: Utc.with_ymd_and_hms(1977, 10, 19, 0, 0, 0).unwrap(),
        poster_url: Default::default(),
        description: Default::default(),
        rating: 5,
    };

    let dune = Model {
        id: 0,
        title: "Dune".to_owned(),
        release_date: Utc.with_ymd_and_hms(1984, 12, 3, 0, 0, 0).unwrap(),
        poster_url: Default::default(),
        description: Default::default(),
        rating: 5,
    };

    // act
    create_movie(&db, star_wars.clone()).await?;
    create_movie(&db, dune.clone()).await?;

    let movies = get_all_movies(&db).await?;

    // assert
    for (this, that) in movies.into_iter().zip([star_wars, dune].into_iter()) {
        assert_eq_ignore_id(this, that);
    }

    Ok(())
}

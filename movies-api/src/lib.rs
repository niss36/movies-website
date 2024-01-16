use axum::{Router, Server};
use movies::{movies_routes, MoviesApiDocs};
use movies_core::sea_orm::Database;
use movies_migration::{Migrator, MigratorTrait};
use std::str::FromStr;
use std::{env, net::SocketAddr};
use utoipa::{openapi, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

mod movies;
mod responses;

pub fn get_api_docs() -> openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(info(title = "Rust Movies", contact()))]
    struct BaseApiDocs;

    let mut api_docs = BaseApiDocs::openapi();
    api_docs.merge(MoviesApiDocs::openapi());

    api_docs
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", get_api_docs()))
        .nest("/movies", movies_routes(conn));

    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

pub fn main() {
    if let Err(err) = start() {
        println!("Error: {err}");
    }
}

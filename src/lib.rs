pub mod db;
pub mod handlers;
pub mod routes;

use anyhow::Result;
use axum::Router;
use db::Database;

pub async fn create_app() -> Result<Router> {
    let db = Database::new().await?;

    let app = Router::new().merge(routes::create_routes(db));

    Ok(app)
}

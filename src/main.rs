use std::sync::Arc;

use repository::pg_repository::PgRepository;
use router::router;
use state::AppState;
use tokio::net::TcpListener;
mod error;
pub mod repository;
pub mod router;
pub mod state;
pub mod models;
pub mod rest;

pub use error::Result;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let repo = PgRepository::init().await?;
    let state = AppState {
        repository: Arc::new(repo),
    };
    let listener = TcpListener::bind(&format!("{host}:{port}")).await?;
    tracing::info!("Starting server at {host}:{port}");
    let app = router(state);
    axum::serve(listener, app).await?;
    Ok(())
}

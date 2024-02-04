use axum::{http::StatusCode, routing::get, Router, Json, Extension};
use axum::extract::Path;
use tracing::instrument;
use uuid::Uuid;
use crate::models::books::Book;
use crate::state::AppState;

pub fn books_service() -> Router {
    Router::new()
        .route("/", get(get_all_books).post(add_book))
        .route("/:id", get(get_book).put(update_book).delete(delete_book))
}
#[instrument]
async fn get_all_books(Extension(state): Extension<AppState>) -> Result<Json<Vec<Book>>, StatusCode> {
    match state.repository.list_all().await {
        Ok(b) => Ok(Json(b)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
#[instrument]
async fn get_book(Extension(state): Extension<AppState>, Path(id): Path<Uuid>) -> Result<Json<Book>, StatusCode> {
    match state.repository.retrieve(id).await {
        Ok(b) => Ok(Json(b)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
#[instrument]
async fn add_book(Extension(state): Extension<AppState>, Json(book): Json<Book>) -> Result<Json<Book>, StatusCode> {
    match state.repository.create(book).await {
        Ok(b) => Ok(Json(b)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
#[instrument]
async fn update_book(Extension(state): Extension<AppState>, Path(id): Path<Uuid>, Json(book): Json<Book>) -> Result<Json<Book>, StatusCode> {
    match state.repository.update(id, book).await {
        Ok(b) => Ok(Json(b)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
#[instrument]
async fn delete_book(Extension(state): Extension<AppState>, Path(id): Path<Uuid>) -> Result<Json<Book>, StatusCode> {
    match state.repository.delete(id).await {
        Ok(b) => Ok(Json(b)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
pub fn general_service() -> Router {
    Router::new().route("/health", get(health_check))
}
async fn health_check() -> StatusCode {
    StatusCode::OK
}

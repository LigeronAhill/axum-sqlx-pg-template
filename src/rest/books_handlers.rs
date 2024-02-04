use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use tracing::instrument;
use uuid::Uuid;
use crate::models::books::Book;
use crate::state::AppState;

#[instrument]
pub async fn get_all_books(Extension(state): Extension<AppState>) -> Result<Json<Vec<Book>>, StatusCode> {
    match state.repository.list_all().await {
        Ok(b) => Ok(Json(b)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
#[instrument]
pub async fn get_book(Extension(state): Extension<AppState>, Path(id): Path<Uuid>) -> Result<Json<Book>, StatusCode> {
    match state.repository.retrieve(id).await {
        Ok(b) => Ok(Json(b)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
#[instrument]
pub async fn add_book(Extension(state): Extension<AppState>, Json(book): Json<Book>) -> Result<Json<Book>, StatusCode> {
    match state.repository.create(book).await {
        Ok(b) => Ok(Json(b)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
#[instrument]
pub async fn update_book(Extension(state): Extension<AppState>, Path(id): Path<Uuid>, Json(book): Json<Book>) -> Result<Json<Book>, StatusCode> {
    match state.repository.update(id, book).await {
        Ok(b) => Ok(Json(b)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
#[instrument]
pub async fn delete_book(Extension(state): Extension<AppState>, Path(id): Path<Uuid>) -> Result<Json<Book>, StatusCode> {
    match state.repository.delete(id).await {
        Ok(b) => Ok(Json(b)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

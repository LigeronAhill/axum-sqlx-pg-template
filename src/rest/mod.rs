use axum::{http::StatusCode, Router, routing::get};
use crate::rest::books_handlers::{add_book, delete_book, get_all_books, get_book, update_book};

mod books_handlers;

async fn health_check() -> StatusCode {
    StatusCode::OK
}
pub fn books_service() -> Router {
    Router::new()
        .route("/", get(get_all_books).post(add_book))
        .route("/:id", get(get_book).put(update_book).delete(delete_book))
}
pub fn general_service() -> Router {
    Router::new().route("/health", get(health_check))
}

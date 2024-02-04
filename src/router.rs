use axum::{Extension, Router};

use crate::{
    rest::{books_service, general_service},
    state::AppState,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest_service("/api/v1/books", books_service())
        .nest_service("/", general_service())
        .layer(Extension(state))
}

use std::fmt::Debug;
use uuid::Uuid;
use crate::models::books::Book;
use super::Result;
pub mod pg_repository;

#[async_trait::async_trait]
pub trait Repository: Sync + Send + Debug {
    async fn list_all(&self) -> Result<Vec<Book>>;
    async fn retrieve(&self, id: Uuid) -> Result<Book>;
    async fn create(&self, book: Book) -> Result<Book>;
    async fn update(&self, id: Uuid, book: Book) -> Result<Book>;
    async fn delete(&self, id: Uuid) -> Result<Book>;
}

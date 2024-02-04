use chrono::Local;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::instrument;
use uuid::Uuid;
use crate::models::books::Book;

use crate::Result;

use super::Repository;
#[derive(Debug, Clone)]
pub struct PgRepository {
    pool: PgPool,
}
impl PgRepository {
    #[instrument]
    pub async fn init() -> Result<Self> {
        let db_url = std::env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();
        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }
    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }
}
#[async_trait::async_trait]
impl Repository for PgRepository {
    #[instrument]
    async fn list_all(&self) -> Result<Vec<Book>> {
        let books = sqlx::query_as!(
            Book,
            r#"SELECT * FROM books ORDER BY author, year;"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(books)
    }
    #[instrument]
    async fn retrieve(&self, id: Uuid) -> Result<Book> {
        let book = sqlx::query_as!(
            Book,
            r#"SELECT * FROM books WHERE id = $1;"#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(book)
    }
    #[instrument]
    async fn create(&self, book: Book) -> Result<Book> {
        let book = sqlx::query_as!(
            Book,
            r#"INSERT INTO books (title, author, year) VALUES ($1, $2, $3) RETURNING *;"#,
            book.title,
            book.author,
            book.year,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(book)
    }
    #[instrument]
    async fn update(&self, id: Uuid, book: Book) -> Result<Book> {
        let now = Local::now().naive_local();
        let book = sqlx::query_as!(
            Book,
            r#"UPDATE books SET title = $1, author = $2, year = $3, updated = $4 WHERE id = $5 RETURNING *;"#,
            book.title,
            book.author,
            book.year,
            now,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(book)
    }
    #[instrument]
    async fn delete(&self, id: Uuid) -> Result<Book> {
        let book = sqlx::query_as!(
            Book,
            r#"DELETE FROM books WHERE id = $1 RETURNING *;"#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(book)
    }
}

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
#[skip_serializing_none]
pub struct Book {
    pub id: Option<Uuid>,
    pub title: String,
    pub author: String,
    pub year: i32,
    pub updated: Option<NaiveDateTime>,
}
// impl Book {
//     pub fn new(title: String, author: String, year: i32) -> Self {
//         Self {
//             id: None,
//             title,
//             author,
//             year,
//             updated: None,
//         }
//     }
// }
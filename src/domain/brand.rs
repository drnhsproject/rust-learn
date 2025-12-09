use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Brand {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

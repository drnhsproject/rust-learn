use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Seller {
    id: i32,
    name: String,
}

use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: String,
}

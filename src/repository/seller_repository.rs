use sqlx::{Error, PgPool, Row, postgres::PgRow};

async fn insert_auto_increment(pool: &PgPool) -> Result<PgRow, Error> {
    let id = sqlx::query("INSERT INTO sellers (name) VALUES ($1) RETURNING id;")
        .bind("GATCHA")
        .fetch_one(pool)
        .await?;
    Ok(id)
}

async fn insert_auto_increment_with_transaction(pool: &PgPool) -> Result<i32, Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("INSERT INTO sellers (name) VALUES ($1)")
        .bind("reborn")
        .execute(&mut *transaction)
        .await?;

    let result: PgRow = sqlx::query("SELECT LASTVAL() AS id")
        .fetch_one(&mut *transaction)
        .await?;

    let id: i32 = result.get_unchecked("id");
    transaction.commit().await?;
    Ok(id)
}

#[cfg(test)]
mod tests {
    use crate::database_pool::get_pool;

    use super::*;
    use sqlx::{
        Pool, Row,
        postgres::{PgRow, Postgres},
    };

    #[tokio::test]
    async fn test_insert_auto_increment() -> Result<(), Error> {
        let pool: Pool<Postgres> = get_pool().await?;
        let result: PgRow = insert_auto_increment(&pool).await?;

        let id: i32 = result.get("id");
        println!("id: {}", id);
        Ok(())
    }

    #[tokio::test]
    async fn test_insert_auto_increment_with_transaction() -> Result<(), Error> {
        let pool: Pool<Postgres> = get_pool().await?;
        let result: i32 = insert_auto_increment_with_transaction(&pool).await?;

        println!("id: {}", result);
        Ok(())
    }
}

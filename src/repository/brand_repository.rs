use chrono::Local;
use sqlx::postgres::PgPool;
use sqlx::{Error, Postgres, Transaction};

use crate::domain::brand::Brand;

async fn insert(pool: &PgPool) -> Result<(), Error> {
    sqlx::query("INSERT INTO brands (id, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)")
        .bind("2")
        .bind("google")
        .bind("google adalah sebuah merek")
        .bind(Local::now().naive_local())
        .bind(Local::now().naive_local())
        .execute(pool)
        .await?;
    Ok(())
}

async fn insert_with_transaction(pool: &PgPool) -> Result<(), Error> {
    let mut transaction: Transaction<Postgres> = pool.begin().await?;
    sqlx::query("INSERT INTO brands (id, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)")
        .bind("5")
        .bind("godox")
        .bind("godox adalah sebuah merek")
        .bind(Local::now().naive_local())
        .bind(Local::now().naive_local())
        .execute(&mut *transaction)
        .await?;

    sqlx::query("INSERT INTO brands (id, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)")
        .bind("6")
        .bind("xiaomi")
        .bind("xiaomi adalah sebuah merek")
        .bind(Local::now().naive_local())
        .bind(Local::now().naive_local())
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

async fn fetch_all_with_auto_mapping(pool: &PgPool) -> Result<Vec<Brand>, Error> {
    sqlx::query_as("SELECT * FROM brands").fetch_all(pool).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database_pool::get_pool;
    use sqlx::Error;

    #[tokio::test]
    async fn test_insert() -> Result<(), Error> {
        let pool = get_pool().await?;

        insert(&pool).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_with_transaction() -> Result<(), Error> {
        let pool = get_pool().await?;

        insert_with_transaction(&pool).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_all_with_auto_mapping() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result: Vec<Brand> = fetch_all_with_auto_mapping(&pool).await?;

        for brand in result {
            println!("{:?}", brand);
        }

        Ok(())
    }
}

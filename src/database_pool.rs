use std::time::Duration;

use sqlx::{
    Error,
    postgres::{PgPool, PgPoolOptions},
};

pub async fn get_pool() -> Result<PgPool, Error> {
    let url = "postgres://admin:password@localhost:5432/rust_learn";
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(60))
        .connect(url)
        .await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_pool() -> Result<(), Error> {
        let pool = get_pool().await?;
        pool.close().await;

        Ok(())
    }
}

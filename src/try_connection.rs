#[cfg(test)]
mod tests {
    use sqlx::{Connection, Error, PgConnection};

    #[tokio::test]
    async fn test_manual_connection() -> Result<(), Error> {
        let url = "postgres://admin:password@localhost:5432/rust_learn";
        let connection = PgConnection::connect(url).await?;

        connection.close().await?;

        Ok(())
    }
}

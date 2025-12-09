use crate::domain::category::Category;
use futures::stream::BoxStream;
use sqlx::{
    Error, Row,
    postgres::{PgPool, PgRow},
};

async fn insert(pool: &PgPool) -> Result<(), Error> {
    sqlx::query("INSERT INTO category (id, name, description) VALUES (2, 'memasak', 'memasak adalah sebuah kegiatan')")
        .execute(pool)
        .await?;
    Ok(())
}

async fn insert_bind(pool: &PgPool) -> Result<(), Error> {
    sqlx::query("INSERT INTO category (id, name, description) VALUES ($1, $2, $3)")
        .bind(3)
        .bind("jedak-jeduk")
        .bind("jedak-jeduk adalah sebuah kegiatan")
        .execute(pool)
        .await?;
    Ok(())
}

async fn find_one_or_null(pool: &PgPool) -> Result<Option<PgRow>, Error> {
    sqlx::query("SELECT * FROM category WHERE id = $1")
        .bind("9")
        .fetch_optional(pool)
        .await
}

async fn find_one(pool: &PgPool) -> Result<PgRow, Error> {
    sqlx::query("SELECT * FROM category WHERE id = $1")
        .bind("2")
        .fetch_one(pool)
        .await
}

async fn fetch_all(pool: &PgPool) -> Result<Vec<PgRow>, Error> {
    sqlx::query("SELECT * FROM category").fetch_all(pool).await
}

fn fetch_all_stream(pool: &PgPool) -> BoxStream<'_, Result<PgRow, Error>> {
    sqlx::query("SELECT * FROM category").fetch(pool)
}

async fn fetch_all_with_mapping(pool: &PgPool) -> Result<Vec<Category>, Error> {
    sqlx::query("SELECT * FROM category")
        .map(|row: PgRow| Category {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
        })
        .fetch_all(pool)
        .await
}

async fn fetch_all_with_auto_mapping(pool: &PgPool) -> Result<Vec<Category>, Error> {
    sqlx::query_as("SELECT * FROM category")
        .fetch_all(pool)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{database_pool::get_pool, domain::category::Category};
    use futures::TryStreamExt;
    use sqlx::{Error, Pool, Postgres, Row, postgres::PgRow};

    #[tokio::test]
    async fn test_category_repository() -> Result<(), Error> {
        let pool = get_pool().await?;

        insert(&pool).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_prepare_statement() -> Result<(), Error> {
        let pool = get_pool().await?;

        insert_bind(&pool).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_find_one_or_null() -> Result<(), Error> {
        let pool: Pool<Postgres> = get_pool().await?;

        let result: Option<PgRow> = find_one_or_null(&pool).await?;

        if let Some(row) = result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("Id: {}, Name: {}, Description: {}", id, name, description);
        } else {
            println!("data not found");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_find_one() -> Result<(), Error> {
        let pool: Pool<Postgres> = get_pool().await?;

        let result: PgRow = find_one(&pool).await?;

        let id: String = result.get("id");
        let name: String = result.get("name");
        let description: String = result.get("description");
        println!("Id: {}, Name: {}, Description: {}", id, name, description);

        Ok(())
    }

    #[tokio::test]
    async fn test_find_all() -> Result<(), Error> {
        let pool: Pool<Postgres> = get_pool().await?;

        let result: Vec<PgRow> = fetch_all(&pool).await?;

        for row in result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("Id: {}, Name: {}, Description: {}", id, name, description);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_find_all_stream() -> Result<(), Error> {
        let pool: Pool<Postgres> = get_pool().await?;

        let mut result = fetch_all_stream(&pool);

        while let Some(row) = result.try_next().await? {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("Id: {}, Name: {}, Description: {}", id, name, description);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_result_mapping() -> Result<(), Error> {
        let pool: Pool<Postgres> = get_pool().await?;

        let result: Vec<Category> = fetch_all_with_mapping(&pool).await?;

        for category in result {
            println!("{:?}", category);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_result_auto_mapping() -> Result<(), Error> {
        let pool: Pool<Postgres> = get_pool().await?;

        let result: Vec<Category> = fetch_all_with_auto_mapping(&pool).await?;

        for category in result {
            println!("{:?}", category);
        }

        Ok(())
    }
}

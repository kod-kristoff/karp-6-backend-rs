use sqlx::sqlite::SqlitePool;

#[cfg(test)]
type Pool = SqlitePool;


#[cfg(test)]
pub mod test {
    use super::*;

    async fn migrate_and_config_db(url: &str) -> Pool {
        use sqlx::sqlite::SqlitePoolOptions;

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(url)
            .await
            .expect("Failed to connect to test pool.");

        pool
    }
}

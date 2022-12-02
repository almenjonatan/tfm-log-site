#[cfg(test)]
mod tests {

    use crate::db::db_api::create_pool;

    #[tokio::test]
    async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        // let pool = create_pool().await;
        // let res = sqlx::query!("SELECT id FROM game_data");

        Ok(())
    }
}

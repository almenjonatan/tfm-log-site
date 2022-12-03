#[cfg(test)]
pub mod tests {
    use crate::{server, models::game::Game};
    use serde_json::json;

    #[tokio::test]
    async fn test_post() {
        // tokio::spawn(async {
        //     server::run().await;
        // });
    

        // let client = reqwest::Client::new();
        // let res = client
        //     .post("http://localhost:3000/upload")
        //     .json(&payload)
        //     .send()
        //     .await
        //     .unwrap();

        // assert_eq!(res.status(), 200);
    }
}

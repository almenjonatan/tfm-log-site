#[cfg(test)]
pub mod tests {
    use crate::server;
    use core::time::Duration;
    use serde_json::json;
    use std::thread;

    #[tokio::test]
    async fn test_post() {
        tokio::spawn(async {
            server::run().await;
        });

        let payload = json!({
            "id": 123,
            "players": [
                {
                    "id": 111,
                    "name": "Jonatan"
                },
                {
                    "id": 222,
                    "name": "anthracite"
                }
            ],
            "game": {
                "id": 555,
                "time": 1234 
            }
        });

        // println!("{:#?}", payload);

        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:3000/upload")
            .json(&payload)
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), 200);
    }
}

use anyhow::Result;
use serde_json::json;
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?difficulty=easy&board=1,2,3,4,5,6").await?.print().await?;

    hc.do_post("/ai", json!({"difficulty": "easy", "board": [[0, 1, 2, 3, 4, 5, 6],[0, 1, 2, 3, 4, 5, 6],[0, 1, 2, 3, 4, 5, 6],[0, 1, 2, 3, 4, 5, 6],[0, 1, 2, 3, 4, 5, 6],[0, 1, 2, 3, 4, 5, 6]]})).await?.print().await?;
    Ok(()) 
}

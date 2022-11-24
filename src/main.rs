use serde_json::{Value, json};
use shaco::rest;

#[tokio::main]
async fn main() {
    let page = json!({
        "name": "Rune Page Using Shaco!",
        "primaryStyleId": 8100,
        "subStyleId": 8300,
        "selectedPerkIds":  [8135, 8120, 8126, 8112, 8306, 8321]
    });
    push_runes_to_client(page).await;
}

/// Attempts to push runes to the LoL Client Via the LCU API
/// this will eventually end up wrapped in some sort of struct
/// that handles checking if the LCU exists
///
/// Requires JSON as an argument
pub async fn push_runes_to_client(page: Value) -> i64 {
    let pages_endpoint = String::from("/lol-perks/v1/pages");
    if let Ok(client) = rest::RESTClient::new() {
        if client
            .put(pages_endpoint.clone(), page.clone())
            .await
            .is_ok()
        {
            return 1;
        } else if let Ok(response) = client.get("/lol-perks/v1/currentpage".to_string()).await {
            let Some(id) = &response["id"].as_i64() else {
                panic!();
            };
            if client
                .delete(format!("{0}/{1}", pages_endpoint, id))
                .await
                .is_ok()
            {
                if client.put(pages_endpoint, page).await.is_ok() {
                    return 1;
                } else {
                    panic!()
                    }
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}
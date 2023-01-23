use serde_json::{json, Value};
use samira::rest;

#[tokio::main]
async fn main() {
    let client = rest::LCUClient::new().unwrap();
    println!("{:?}", client.get("/lol-summoner/v1/current-summoner").await);
    let page = json!({
        "name": "Rune Page Using Shaco!",
        "primaryStyleId": 8100,
        "subStyleId": 8300,
        "selectedPerkIds":  [8135, 8120, 8126, 8112, 8306, 8321]
    });
    println!("{}", push_runes_to_client(page).await);

    let page = json!(
        {
          "associatedChampions": [
            
          ],
          "associatedMaps": [
            
          ],
          "blocks": [
            {
              "items": [
                {
                  "count": 1,
                  "id": "3153"
                },
                {
                  "count": 1,
                  "id": "6673"
                },
                {
                  "count": 1,
                  "id": "3006"
                },
                {
                  "count": 1,
                  "id": "3091"
                },
                {
                  "count": 1,
                  "id": "3085"
                },
                {
                  "count": 1,
                  "id": "3072"
                },
                {
                  "count": 1,
                  "id": "3363"
                }
              ],
              "type": "Final Build"
            }
          ],
          "title": "Ahri Build",
        }
    );
    push_items_to_client(page).await;
}

/// Attempts to push runes to the LoL Client Via the LCU API
/// this will eventually end up wrapped in some sort of struct
/// that handles checking if the LCU exists
///
/// Requires JSON as an argument
pub async fn push_runes_to_client(page: Value) -> i64 {
    let pages_endpoint = String::from("/lol-perks/v1/pages");
    match rest::LCUClient::new() {
        Ok(client) => {
            match client.get("/lol-perks/v1/currentpage").await {
                Ok(response) => {
                    let response = response.unwrap();
                    println!("{:?}", &response);
                    let Some(id) = &response["id"].as_i64() else {
                        panic!();
                    };
                    println!("{}", id);
                    if client
                        .delete(&format!("/lol-perks/v1/page/{}", id))
                        .await
                        .is_ok()
                    {
                        if client.post(&pages_endpoint, page).await.is_ok() {
                            return 2;
                        } else {
                            panic!()
                        }
                    } else {
                        panic!()
                    }
                },
                Err(err) => panic!("{:?}", err),
            }
        }
        Err(err) => panic!("{:?}", err)
    }
}

/// Attempts to push an item set to the client via the LCU API
/// this will eventually end up wrapped in some form of struct
/// that handles checking if the LCU is open
/// 
/// Requires JSON as an argument
pub async fn push_items_to_client(page: Value) -> i64 {
    if let Ok(client) = rest::LCUClient::new() {
        let a = client
            .get("/lol-summoner/v1/current-summoner")
            .await;
        match a {
            Ok(a) => {
                let a = a.unwrap();
                println!("{}", a);
                let endpoint_a = format!("/lol-item-sets/v1/item-sets/{}/sets", a["summonerId"]);
                let a = client.get(&endpoint_a).await;
                match a {
                    Ok(json) => {
                        let mut a = json.unwrap().clone();
                        a["itemSets"].as_array_mut().unwrap().push(page);
                        let a = client.put(&endpoint_a, a).await;          
                        println!("{:?}", a);
                        return 1;

                    },
                    Err(_) => panic!(),
                }
            }
            Err(a) => {
                println!("{:?}", a);
                panic!()
            }
        }
    } else {
        panic!()
    }
}

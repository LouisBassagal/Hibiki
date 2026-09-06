use tauri_plugin_http::reqwest;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeImage {
    pub extra_large: Option<String>,
    pub large: Option<String>,
    pub medium: Option<String>,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeMedia {
    pub id: i32,
    pub title: AnimeTitle,
    #[serde(rename = "coverImage")]
    pub cover_image: AnimeImage,
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,
}

#[tauri::command]
pub async fn fetch_popular_animes(page: u32) -> Result<Vec<AnimeMedia>, String> {
    let client = reqwest::Client::new();

    let query = r#"
        query ($page: Int, $perPage: Int) {
            Page(page: $page, perPage: $perPage) {
                media(type: ANIME, sort: POPULARITY_DESC, isAdult: false) {
                    id
                    title {
                        romaji
                        english
                    }
                    coverImage {
                        extraLarge
                        large
                        medium
                        color
                    }
                    bannerImage
                }
            }
        }
    "#;

    let body = json!({
        "query": query,
        "variables": {
            "page": page,
            "perPage": 50
        }
    });

    let res = client
        .post("https://graphql.anilist.co")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(body.to_string())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // 2. Récupération du texte brut de la réponse
    let response_text = res.text().await.map_err(|e| e.to_string())?;

    println!("Message from Rust: {}", response_text);

    // 3. Conversion du texte en JSON via serde_json
    let response_json: serde_json::Value =
        serde_json::from_str(&response_text).map_err(|e| e.to_string())?;

    // 4. Extraction et désérialisation de la liste d'animes
    let media_list = response_json["data"]["Page"]["media"].clone();
    let animes: Vec<AnimeMedia> = serde_json::from_value(media_list).map_err(|e| e.to_string())?;

    Ok(animes)
}
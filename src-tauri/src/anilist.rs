use chrono::{Datelike, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

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

fn current_season() -> (&'static str, i32) {
    let now = Utc::now();
    let season = match now.month() {
        1..=3 => "WINTER",
        4..=6 => "SPRING",
        7..=9 => "SUMMER",
        _ => "FALL",
    };

    (season, now.year())
}

#[tauri::command]
pub async fn fetch_popular_animes(page: u32, count: u32) -> Result<Vec<AnimeMedia>, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|error| error.to_string())?;

    let query = r#"
        query ($page: Int, $perPage: Int) {
            Page(page: $page, perPage: $perPage) {
                media(
                    type: ANIME
                    sort: POPULARITY_DESC
                    isAdult: false
                ) {
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
            "perPage": count
        }
    });

    let response = client
        .post("https://graphql.anilist.co")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|error| format!("Erreur lecture réponse: {error}"))?;

    if !status.is_success() {
        let msg = format!("AniList HTTP {status}: {response_text}");
        eprintln!("{msg}");
        return Err(msg);
    }

    let response_json: serde_json::Value = serde_json::from_str(&response_text)
        .map_err(|error| format!("JSON invalide: {error}; réponse: {response_text}"))?;

    if let Some(errors) = response_json.get("errors") {
        return Err(errors.to_string());
    }

    serde_json::from_value(response_json["data"]["Page"]["media"].clone())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn fetch_current_season_animes(page: u32, count: u32) -> Result<Vec<AnimeMedia>, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|error| error.to_string())?;
    let (season, season_year) = current_season();

    let query = r#"
        query ($page: Int, $perPage: Int, $season: MediaSeason!, $seasonYear: Int!) {
            Page(page: $page, perPage: $perPage) {
                media(
                    type: ANIME
                    season: $season
                    seasonYear: $seasonYear
                    sort: POPULARITY_DESC
                    isAdult: false
                ) {
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
            "perPage": count,
            "season": season,
            "seasonYear": season_year
        }
    });

    let response = client
        .post("https://graphql.anilist.co")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|error| format!("Erreur lecture réponse: {error}"))?;

    if !status.is_success() {
        let msg = format!("AniList HTTP {status}: {response_text}");
        eprintln!("{msg}");
        return Err(msg);
    }

    let response_json: serde_json::Value = serde_json::from_str(&response_text)
        .map_err(|error| format!("JSON invalide: {error}; réponse: {response_text}"))?;

    if let Some(errors) = response_json.get("errors") {
        return Err(errors.to_string());
    }

    serde_json::from_value(response_json["data"]["Page"]["media"].clone())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn fetch_best_rated_animes(page: u32, count: u32) -> Result<Vec<AnimeMedia>, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|error| error.to_string())?;

    let query = r#"
        query ($page: Int, $perPage: Int) {
            Page(page: $page, perPage: $perPage) {
                media(
                    type: ANIME
                    sort: SCORE_DESC
                    isAdult: false
                    averageScore_greater: 0
                ) {
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
            "perPage": count
        }
    });

    let response = client
        .post("https://graphql.anilist.co")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|error| format!("Erreur lecture réponse: {error}"))?;

    if !status.is_success() {
        let msg = format!("AniList HTTP {status}: {response_text}");
        eprintln!("{msg}");
        return Err(msg);
    }

    let response_json: serde_json::Value = serde_json::from_str(&response_text)
        .map_err(|error| format!("JSON invalide: {error}; réponse: {response_text}"))?;

    if let Some(errors) = response_json.get("errors") {
        return Err(errors.to_string());
    }

    serde_json::from_value(response_json["data"]["Page"]["media"].clone())
        .map_err(|error| error.to_string())
}

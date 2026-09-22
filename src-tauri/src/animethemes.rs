use serde::{Deserialize, Serialize};
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeThemesAnime {
    pub slug: String,
    pub year: Option<i32>,
    pub season: Option<String>,
    pub images: ImageConnection,
    pub animethemes: Vec<AnimeTheme>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageConnection {
    pub nodes: Vec<Image>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub link: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeTheme {
    pub id: i32,
    #[serde(rename = "type")]
    pub theme_type: String, // "OP" ou "ED"
    pub sequence: Option<i32>,
    pub slug: String,
    pub song: Option<Song>,
    pub animethemeentries: Vec<ThemeEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub title: Option<SongTitle>,
    pub performances: Vec<Performance>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SongTitle {
    pub romaji: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Performance {
    pub artist: Artist,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Artist {
    pub name: ArtistName,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtistName {
    pub main: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeEntry {
    pub version: Option<i32>,
    pub episodes: Option<String>,
    pub spoiler: bool,
    pub nsfw: bool,
    pub videos: VideoConnection,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoConnection {
    pub nodes: Vec<Video>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Video {
    pub basename: String,
    pub filename: String,
    pub nc: bool,
    pub tags: Option<String>,
    pub uncen: bool,
    pub source: Option<String>,
    pub resolution: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct FindAnimeResponse {
    #[serde(rename = "findAnimeByExternalSite")]
    find_anime_by_external_site: Vec<AnimeThemesAnime>,
}

pub async fn fetch_themes_by_anilist_id_cmd(anilist_id: i32) -> Result<Vec<AnimeThemesAnime>, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("AnimeThemesMobile/0.1.0")
        .build()
        .map_err(|e| e.to_string())?;

    let query = r#"
         query ($anilistId: [Int!]) {
            findAnimeByExternalSite(site: ANILIST, id: $anilistId) {
                slug
                year
    			images {
                  nodes {
                    link               
                  }
                }
                season
                animethemes {
                    id
                    type
                    sequence
                    slug
                    song {
                        title { romaji }
                      performances {
                        artist {
                          name {
                            main
                          }
                        }
                      }
                    }
                    animethemeentries {
                        version
                        episodes
                        spoiler
                        nsfw
                      	videos {
                          nodes {
                            basename
                            filename
                            nc
                            tags
                            uncen
                            source
                            resolution
                          }
                        }
                    }
                }
            }
        }
    "#;

    let body = json!({
        "query": query,
        "variables": { "anilistId": [anilist_id] }
    });

    let response = client
        .post("https://graphql.animethemes.moe/")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    let text = response.text().await.map_err(|e| e.to_string())?;

    if !status.is_success() {
        return Err(format!("AnimeThemes HTTP {status}: {text}"));
    }

    let json_value: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| format!("JSON invalide: {e}; réponse: {text}"))?;

    if let Some(errors) = json_value.get("errors") {
        return Err(errors.to_string());
    }

    let parsed: FindAnimeResponse = serde_json::from_value(json_value["data"].clone())
        .map_err(|e| e.to_string())?;

    Ok(parsed.find_anime_by_external_site)
}

#[tauri::command]
pub async fn fetch_themes_by_anilist_id(
    anilist_id: i32,
) -> Result<AnimeThemesAnime, String> {
    fetch_themes_by_anilist_id_cmd(anilist_id)
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| "Aucun anime trouve dans AnimeThemes".to_string())
}
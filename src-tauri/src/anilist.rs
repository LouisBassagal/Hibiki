use chrono::{Datelike, Utc};
use rand::seq::{IndexedRandom, IteratorRandom};
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
enum RandomStrategy {
    AllTimePopular,
    RandomSeasonPopular,
    TopRatedAllTime,
    CurrentTrending,
}

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
    pub description: Option<String>,
    #[serde(rename = "isFavourite")]
    pub is_favourite: Option<bool>,
    #[serde(rename = "mediaListEntry")]
    pub media_list_entry: Option<AnimeMediaListEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimeMediaListEntry {
    pub status: Option<String>,
    pub score: Option<f32>,
    pub progress: Option<i32>,
    pub started_at: Option<AnimeFuzzyDate>,
    pub completed_at: Option<AnimeFuzzyDate>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimeFuzzyDate {
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewerAvatar {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewerAnimeStatistics {
    pub count: Option<i32>,
    pub episodes_watched: Option<i32>,
    pub minutes_watched: Option<i32>,
    pub mean_score: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewerStatistics {
    pub anime: Option<ViewerAnimeStatistics>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewerAnimeFavorites {
    pub nodes: Option<Vec<AnimeMedia>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewerFavorites {
    pub anime: Option<ViewerAnimeFavorites>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
    pub id: i32,
    pub name: String,
    pub avatar: Option<ViewerAvatar>,
    pub banner_image: Option<String>,
    pub site_url: Option<String>,
    pub statistics: Option<ViewerStatistics>,
    pub favourites: Option<ViewerFavorites>,
}

#[tauri::command]
pub async fn fetch_viewer(token: &str) -> Result<Viewer, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|error| error.to_string())?;

    let query = r#"
        query {
            Viewer {
                id
                name
                avatar {
                    large
                    medium
                }
                bannerImage
                siteUrl
                statistics {
                    anime {
                        count
                        episodesWatched
                        minutesWatched
                        meanScore
                    }
                }
                favourites {
                    anime {
                        nodes {
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
                            description
                        }
                    }
                }
            }
        }
    "#;

    let body = json!({ "query": query });

    let response = client
        .post("https://graphql.anilist.co")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .bearer_auth(token)
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

    serde_json::from_value(response_json["data"]["Viewer"].clone())
        .map_err(|error| error.to_string())
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
                    description
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
                    description
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
                    description
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
pub async fn fetch_random_known_anime() -> Result<AnimeMedia, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|error| error.to_string())?;

    let strategies = [
        RandomStrategy::AllTimePopular,
        RandomStrategy::RandomSeasonPopular,
        RandomStrategy::TopRatedAllTime,
        RandomStrategy::CurrentTrending,
    ];

    let strategy = strategies.choose(&mut rand::rng()).copied().unwrap();

    let (query, variables) = build_query_for_strategy(strategy);

    let body = json!({
        "query": query,
        "variables": variables
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
        return Err(format!("AniList HTTP {status}: {response_text}"));
    }

    let response_json: serde_json::Value =
        serde_json::from_str(&response_text).map_err(|error| format!("JSON invalide: {error}"))?;

    if let Some(errors) = response_json.get("errors") {
        return Err(errors.to_string());
    }

    let media_list: Vec<AnimeMedia> =
        serde_json::from_value(response_json["data"]["Page"]["media"].clone())
            .map_err(|error| error.to_string())?;

    media_list
        .into_iter()
        .choose(&mut rand::rng())
        .ok_or_else(|| "Aucun anime trouvé".to_string())
}

fn build_query_for_strategy(strategy: RandomStrategy) -> (&'static str, serde_json::Value) {
    let mut rng = rand::rng();
    let random_page = rng.random_range(1..=25);

    match strategy {
        RandomStrategy::AllTimePopular => {
            let query = r#"
                query ($page: Int, $perPage: Int) {
                    Page(page: $page, perPage: $perPage) {
                        media(
                            type: ANIME
                            sort: POPULARITY_DESC
                            isAdult: false
                            popularity_greater: 50000
                        ) {
                            id
                            title { romaji english }
                            coverImage { extraLarge large medium color }
                            bannerImage
                            description
                        }
                    }
                }
            "#;
            (query, json!({ "page": random_page, "perPage": 20 }))
        }

        RandomStrategy::RandomSeasonPopular => {
            let query = r#"
                query ($page: Int, $perPage: Int, $season: MediaSeason, $year: Int) {
                    Page(page: $page, perPage: $perPage) {
                        media(
                            type: ANIME
                            season: $season
                            seasonYear: $year
                            sort: POPULARITY_DESC
                            isAdult: false
                            popularity_greater: 15000
                        ) {
                            id
                            title { romaji english }
                            coverImage { extraLarge large medium color }
                            bannerImage
                            description
                        }
                    }
                }
            "#;

            let seasons = ["WINTER", "SPRING", "SUMMER", "FALL"];
            let season = seasons.choose(&mut rng).unwrap();
            let year = rng.random_range(2000..=2025);

            (
                query,
                json!({
                    "page": 1,
                    "perPage": 20,
                    "season": season,
                    "year": year
                }),
            )
        }

        RandomStrategy::TopRatedAllTime => {
            let query = r#"
                query ($page: Int, $perPage: Int) {
                    Page(page: $page, perPage: $perPage) {
                        media(
                            type: ANIME
                            sort: SCORE_DESC
                            isAdult: false
                            popularity_greater: 30000
                        ) {
                            id
                            title { romaji english }
                            coverImage { extraLarge large medium color }
                            bannerImage
                            description
                        }
                    }
                }
            "#;
            (query, json!({ "page": random_page, "perPage": 20 }))
        }

        RandomStrategy::CurrentTrending => {
            let query = r#"
                query ($page: Int, $perPage: Int) {
                    Page(page: $page, perPage: $perPage) {
                        media(
                            type: ANIME
                            sort: TRENDING_DESC
                            isAdult: false
                        ) {
                            id
                            title { romaji english }
                            coverImage { extraLarge large medium color }
                            bannerImage
                            description
                        }
                    }
                }
            "#;
            (query, json!({ "page": 1, "perPage": 20 }))
        }
    }
}

#[tauri::command]
pub async fn fetch_anime_by_name(
    search: &str,
    page: u32,
    count: u32,
) -> Result<Vec<AnimeMedia>, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|error| error.to_string())?;

    let query = r#"
        query ($search: String!, $page: Int, $perPage: Int) {
            Page(page: $page, perPage: $perPage) {
                media(
                    search: $search
                    type: ANIME
                    sort: SCORE_DESC
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
                    description
                }
            }
        }
    "#;

    let body = json!({
        "query": query,
        "variables": {
            "search": search,
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
pub async fn fetch_anime_by_id(id: i32, token: Option<String>) -> Result<AnimeMedia, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|error| error.to_string())?;

    let query = r#"
        query ($id: Int!) {
            Media(
                id: $id
                type: ANIME
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
                description
                isFavourite
                mediaListEntry {
                    status
                    score
                    progress
                    startedAt {
                        year
                        month
                        day
                    }
                    completedAt {
                        year
                        month
                        day
                    }
                }
            }
        }
    "#;

    let body = json!({
        "query": query,
        "variables": {
            "id": id,
        }
    });

    let mut request = client
        .post("https://graphql.anilist.co")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json");

    if let Some(token) = token.as_deref() {
        request = request.bearer_auth(token);
    }

    let response = request
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

    serde_json::from_value(response_json["data"]["Media"].clone())
        .map_err(|error| error.to_string())
}

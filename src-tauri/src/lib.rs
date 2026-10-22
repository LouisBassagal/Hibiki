// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod anilist;
mod animethemes;
mod utils;

use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "init_schema",
        sql: include_str!("../migrations/0001_init.sql"),
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_http::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(
                    "sqlite:anime-openings.db?mode=rwc&foreign_keys=true",
                    migrations,
                )
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            anilist::fetch_popular_animes,
            anilist::fetch_current_season_animes,
            anilist::fetch_best_rated_animes,
            anilist::fetch_random_known_anime,
            anilist::fetch_anime_by_name,
            anilist::fetch_anime_by_id,
            animethemes::fetch_themes_by_anilist_id,
            utils::lock_orientation,
            utils::set_fullscreen,
            utils::quit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

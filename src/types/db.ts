// Access layer for the local SQLite database.
// All app functions should go through here rather than calling
// Database directly, to keep SQL queries centralized.
//
// Schema v2: identity (anime, theme) is separated from the metadata cache
// (anime_meta_cache, theme_meta_cache). Favorites/history/playlists only
// ever reference identity — see 0001_init.sql for the full reasoning.

import Database from "@tauri-apps/plugin-sql";

let dbInstance: Database | null = null;

export async function getDb(): Promise<Database> {
  if (!dbInstance) {
    dbInstance = await Database.load("sqlite:anime-openings.db");
    await dbInstance.execute("PRAGMA foreign_keys = ON");
  }
  return dbInstance;
}

// ---------- Types ----------

export interface AnimeInput {
  anilist_id: number;
  animethemes_id: number | null;
  animethemes_slug: string | null;
  title_romaji: string;
  title_english: string | null;
  cover_url: string | null;
  season: string | null;
  season_year: number | null;
  synopsis: string | null;
}

export interface ThemeInput {
  theme_id: number;
  anilist_id: number;
  type: "OP" | "ED";
  sequence: number | null;
  slug: string | null;
  song_title: string | null;
  song_artist: string | null;
  video_url: string;
  audio_url: string | null;
}

// Flattened view as returned by identity + cache joins. Metadata fields
// are nullable: the cache may have been purged, in which case the app
// should refetch before showing details / starting playback.
export interface ThemeWithMeta {
  theme_id: number;
  anilist_id: number;
  type: "OP" | "ED";
  sequence: number | null;
  slug: string | null;
  song_title: string | null;
  song_artist: string | null;
  video_url: string | null;
  audio_url: string | null;
  title_romaji: string | null;
  title_english: string | null;
  cover_url: string | null;
}

export interface Playlist {
  id: number;
  name: string;
  created_at: string;
}

// ---------- Anime / theme identity + cache (upsert after API calls) ----------

// Creates the anime's identity (if needed), then upserts its metadata.
// Both tables are always written together here for convenience, but
// nothing technically prevents anime_meta_cache from being cleared later
// without touching `anime`.
export async function cacheAnime(anime: AnimeInput): Promise<void> {
  const db = await getDb();
  await db.execute(
    `INSERT INTO anime (anilist_id, animethemes_id, animethemes_slug)
     VALUES ($1, $2, $3)
     ON CONFLICT(anilist_id) DO UPDATE SET
       animethemes_id = COALESCE(excluded.animethemes_id, anime.animethemes_id),
       animethemes_slug = COALESCE(excluded.animethemes_slug, anime.animethemes_slug)`,
    [anime.anilist_id, anime.animethemes_id, anime.animethemes_slug]
  );

  await db.execute(
    `INSERT INTO anime_meta_cache
      (anilist_id, title_romaji, title_english, cover_url, season, season_year, synopsis)
     VALUES ($1, $2, $3, $4, $5, $6, $7)
     ON CONFLICT(anilist_id) DO UPDATE SET
       title_romaji = excluded.title_romaji,
       title_english = excluded.title_english,
       cover_url = excluded.cover_url,
       season = excluded.season,
       season_year = excluded.season_year,
       synopsis = excluded.synopsis,
       cached_at = datetime('now')`,
    [
      anime.anilist_id,
      anime.title_romaji,
      anime.title_english,
      anime.cover_url,
      anime.season,
      anime.season_year,
      anime.synopsis,
    ]
  );
}

export async function markThemesFetched(anilistId: number): Promise<void> {
  const db = await getDb();
  await db.execute(`UPDATE anime SET themes_fetched = 1 WHERE anilist_id = $1`, [anilistId]);
}

export async function cacheTheme(theme: ThemeInput): Promise<void> {
  const db = await getDb();
  await db.execute(
    `INSERT INTO theme (theme_id, anilist_id, type, sequence, slug)
     VALUES ($1, $2, $3, $4, $5)
     ON CONFLICT(theme_id) DO NOTHING`,
    [theme.theme_id, theme.anilist_id, theme.type, theme.sequence, theme.slug]
  );

  await db.execute(
    `INSERT INTO theme_meta_cache (theme_id, song_title, song_artist, video_url, audio_url)
     VALUES ($1, $2, $3, $4, $5)
     ON CONFLICT(theme_id) DO UPDATE SET
       song_title = excluded.song_title,
       song_artist = excluded.song_artist,
       video_url = excluded.video_url,
       audio_url = excluded.audio_url,
       cached_at = datetime('now')`,
    [theme.theme_id, theme.song_title, theme.song_artist, theme.video_url, theme.audio_url]
  );
}

// Theme + metadata if present in cache. If video_url comes back null, the
// cache has been purged: the app should refetch from AnimeThemes before
// offering playback.
export async function getTheme(themeId: number): Promise<ThemeWithMeta | null> {
  const db = await getDb();
  const rows = await db.select<ThemeWithMeta[]>(
    `SELECT t.theme_id, t.anilist_id, t.type, t.sequence, t.slug,
            tm.song_title, tm.song_artist, tm.video_url, tm.audio_url,
            am.title_romaji, am.title_english, am.cover_url
     FROM theme t
     LEFT JOIN theme_meta_cache tm ON tm.theme_id = t.theme_id
     LEFT JOIN anime_meta_cache am ON am.anilist_id = t.anilist_id
     WHERE t.theme_id = $1`,
    [themeId]
  );
  return rows[0] ?? null;
}

// ---------- Favorites ----------

export async function addFavorite(themeId: number): Promise<void> {
  const db = await getDb();
  await db.execute(
    `INSERT OR IGNORE INTO favorites (theme_id) VALUES ($1)`,
    [themeId]
  );
}

export async function removeFavorite(themeId: number): Promise<void> {
  const db = await getDb();
  await db.execute(`DELETE FROM favorites WHERE theme_id = $1`, [themeId]);
}

export async function isFavorite(themeId: number): Promise<boolean> {
  const db = await getDb();
  const rows = await db.select<{ theme_id: number }[]>(
    `SELECT theme_id FROM favorites WHERE theme_id = $1`,
    [themeId]
  );
  return rows.length > 0;
}

export async function getFavorites(): Promise<ThemeWithMeta[]> {
  const db = await getDb();
  return db.select(
    `SELECT t.theme_id, t.anilist_id, t.type, t.sequence, t.slug,
            tm.song_title, tm.song_artist, tm.video_url, tm.audio_url,
            am.title_romaji, am.title_english, am.cover_url
     FROM favorites f
     JOIN theme t ON t.theme_id = f.theme_id
     LEFT JOIN theme_meta_cache tm ON tm.theme_id = t.theme_id
     LEFT JOIN anime_meta_cache am ON am.anilist_id = t.anilist_id
     ORDER BY f.added_at DESC`
  );
}

// ---------- History ----------

export async function addToHistory(themeId: number): Promise<void> {
  const db = await getDb();
  await db.execute(`INSERT INTO history (theme_id) VALUES ($1)`, [themeId]);
}

export async function getHistory(limit = 50): Promise<(ThemeWithMeta & { played_at: string })[]> {
  const db = await getDb();
  return db.select(
    `SELECT t.theme_id, t.anilist_id, t.type, t.sequence, t.slug,
            tm.song_title, tm.song_artist, tm.video_url, tm.audio_url,
            am.title_romaji, am.title_english, am.cover_url,
            h.played_at
     FROM history h
     JOIN theme t ON t.theme_id = h.theme_id
     LEFT JOIN theme_meta_cache tm ON tm.theme_id = t.theme_id
     LEFT JOIN anime_meta_cache am ON am.anilist_id = t.anilist_id
     ORDER BY h.played_at DESC
     LIMIT $1`,
    [limit]
  );
}

export async function clearHistory(): Promise<void> {
  const db = await getDb();
  await db.execute(`DELETE FROM history`);
}

// ---------- Playlists ----------

export async function createPlaylist(name: string): Promise<number> {
  const db = await getDb();
  const result = await db.execute(
    `INSERT INTO playlists (name) VALUES ($1)`,
    [name]
  );
  return result.lastInsertId as number;
}

export async function renamePlaylist(id: number, name: string): Promise<void> {
  const db = await getDb();
  await db.execute(`UPDATE playlists SET name = $1 WHERE id = $2`, [name, id]);
}

export async function deletePlaylist(id: number): Promise<void> {
  const db = await getDb();
  await db.execute(`DELETE FROM playlists WHERE id = $1`, [id]);
}

export async function getPlaylists(): Promise<Playlist[]> {
  const db = await getDb();
  return db.select(`SELECT * FROM playlists ORDER BY created_at DESC`);
}

export async function addToPlaylist(playlistId: number, themeId: number): Promise<void> {
  const db = await getDb();
  const [{ maxPos } = { maxPos: 0 }] = await db.select<{ maxPos: number }[]>(
    `SELECT COALESCE(MAX(position), 0) as maxPos FROM playlist_items WHERE playlist_id = $1`,
    [playlistId]
  );
  await db.execute(
    `INSERT INTO playlist_items (playlist_id, theme_id, position) VALUES ($1, $2, $3)`,
    [playlistId, themeId, maxPos + 1]
  );
}

export async function removeFromPlaylist(playlistItemId: number): Promise<void> {
  const db = await getDb();
  await db.execute(`DELETE FROM playlist_items WHERE id = $1`, [playlistItemId]);
}

export async function getPlaylistItems(
  playlistId: number
): Promise<(ThemeWithMeta & { item_id: number; position: number })[]> {
  const db = await getDb();
  return db.select(
    `SELECT t.theme_id, t.anilist_id, t.type, t.sequence, t.slug,
            tm.song_title, tm.song_artist, tm.video_url, tm.audio_url,
            am.title_romaji, am.title_english, am.cover_url,
            pi.id as item_id, pi.position
     FROM playlist_items pi
     JOIN theme t ON t.theme_id = pi.theme_id
     LEFT JOIN theme_meta_cache tm ON tm.theme_id = t.theme_id
     LEFT JOIN anime_meta_cache am ON am.anilist_id = t.anilist_id
     WHERE pi.playlist_id = $1
     ORDER BY pi.position ASC`,
    [playlistId]
  );
}

export async function reorderPlaylistItem(itemId: number, newPosition: number): Promise<void> {
  const db = await getDb();
  await db.execute(
    `UPDATE playlist_items SET position = $1 WHERE id = $2`,
    [newPosition, itemId]
  );
}

// ---------- Cache cleanup ----------
//
// theme_meta_cache / anime_meta_cache are a real cache: no user data
// references them, so this cleanup needs no prior checks. A row deleted
// here just means "refetch on next access" — identity (theme/anime, and
// therefore favorites/history/playlists) is never affected.

export async function purgeStaleMetaCache(olderThanDays = 30): Promise<{ themes: number; animes: number }> {
  const db = await getDb();

  const themeResult = await db.execute(
    `DELETE FROM theme_meta_cache WHERE cached_at < datetime('now', '-' || $1 || ' days')`,
    [olderThanDays]
  );
  const animeResult = await db.execute(
    `DELETE FROM anime_meta_cache WHERE cached_at < datetime('now', '-' || $1 || ' days')`,
    [olderThanDays]
  );

  return { themes: themeResult.rowsAffected, animes: animeResult.rowsAffected };
}

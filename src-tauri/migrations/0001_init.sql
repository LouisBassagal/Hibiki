-- Initial migration: schema for the anime openings player app.
--
-- Design principle: we separate the IDENTITY of an anime/theme (permanent,
-- tiny, never purged) from its enriched METADATA (cover, synopsis, video
-- URL, song title...), which is a real cache, freely refreshable/purgeable,
-- and referenced by NO user data. Favorites/history/playlists only ever
-- point to the identity tables, which are never purged automatically.
--
-- Result: purging the metadata cache can never break a playlist, a
-- favorite, or history — there isn't even a link between the two at that
-- level, so there's no need for RESTRICT or application-level checks
-- before deletion.
--
-- Matching pivot: anilist_id is the direct pivot key between the two APIs.
-- AnimeThemes exposes a native filter (/anime?filter[site]=AniList&filter[external_id]=X)
-- that returns, among other things, the internal AnimeThemes anime_id.

-- ============================================================
-- IDENTITY (permanent — never purged automatically)
-- ============================================================

-- Minimal identity of an anime. Created once on first contact with the
-- anime (search, detail screen, season browsing...), only holds what's
-- needed as a pivot between the APIs. Negligible footprint even with
-- thousands of rows.
CREATE TABLE IF NOT EXISTS anime (
    anilist_id      INTEGER PRIMARY KEY,     -- pivot: native AniList ID
    animethemes_id  INTEGER,                 -- internal AnimeThemes ID once matched (NULL until resolved)
    animethemes_slug TEXT,                   -- AnimeThemes slug, handy for direct links
    themes_fetched  INTEGER NOT NULL DEFAULT 0,  -- 0/1: have we already attempted to fetch themes for this anime?
    first_seen_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Minimal identity of a theme (OP/ED). This is what favorites/history/
-- playlists actually reference. Contains ONLY what identifies the theme,
-- nothing refreshable — so nothing that would ever justify deleting it.
CREATE TABLE IF NOT EXISTS theme (
    theme_id        INTEGER PRIMARY KEY,     -- AnimeThemes id
    anilist_id      INTEGER NOT NULL,
    type            TEXT NOT NULL,           -- 'OP' or 'ED'
    sequence        INTEGER,                 -- OP1, OP2...
    slug            TEXT,
    first_seen_at   TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (anilist_id) REFERENCES anime(anilist_id) ON DELETE CASCADE
);

-- ============================================================
-- METADATA CACHE (freely refreshable/purgeable)
-- ============================================================

-- Enriched metadata for an anime (comes from AniList and can change, or
-- that we simply want to be able to re-fetch). 1:1 with `anime`, but can be
-- absent (anime identified but metadata not yet/no longer cached): in that
-- case the app knows it needs to refetch before showing the full detail screen.
CREATE TABLE IF NOT EXISTS anime_meta_cache (
    anilist_id      INTEGER PRIMARY KEY,
    title_romaji    TEXT NOT NULL,
    title_english   TEXT,
    cover_url       TEXT,
    season          TEXT,        -- WINTER / SPRING / SUMMER / FALL
    season_year     INTEGER,
    synopsis        TEXT,
    cached_at       TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (anilist_id) REFERENCES anime(anilist_id) ON DELETE CASCADE
);

-- Enriched metadata for a theme (comes from AnimeThemes and can change,
-- notably the video URL if their CDN moves). 1:1 with `theme`.
-- Can be absent: the app refetches before playback in that case.
CREATE TABLE IF NOT EXISTS theme_meta_cache (
    theme_id        INTEGER PRIMARY KEY,
    song_title      TEXT,
    song_artist     TEXT,
    video_url       TEXT NOT NULL,           -- .webm link
    audio_url       TEXT,                    -- audio link if available
    cached_at       TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (theme_id) REFERENCES theme(theme_id) ON DELETE CASCADE
);

-- ============================================================
-- USER DATA (permanent, only ever references identity)
-- ============================================================

-- User's favorites. References theme (identity), never theme_meta_cache:
-- purging the metadata cache never touches this.
CREATE TABLE IF NOT EXISTS favorites (
    theme_id        INTEGER PRIMARY KEY,
    added_at        TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (theme_id) REFERENCES theme(theme_id) ON DELETE CASCADE
);

-- Listening history (we keep every play, not just the last one).
CREATE TABLE IF NOT EXISTS history (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    theme_id        INTEGER NOT NULL,
    played_at       TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (theme_id) REFERENCES theme(theme_id) ON DELETE CASCADE
);

-- User-created playlists.
CREATE TABLE IF NOT EXISTS playlists (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT NOT NULL,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Playlist contents (junction table, with ordering).
CREATE TABLE IF NOT EXISTS playlist_items (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    playlist_id     INTEGER NOT NULL,
    theme_id        INTEGER NOT NULL,
    position        INTEGER NOT NULL,
    added_at        TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
    FOREIGN KEY (theme_id) REFERENCES theme(theme_id) ON DELETE CASCADE
);

-- ============================================================
-- Indexes
-- ============================================================

CREATE INDEX IF NOT EXISTS idx_theme_anilist_id ON theme(anilist_id);
CREATE INDEX IF NOT EXISTS idx_history_played_at ON history(played_at DESC);
CREATE INDEX IF NOT EXISTS idx_playlist_items_playlist_id ON playlist_items(playlist_id, position);
CREATE INDEX IF NOT EXISTS idx_anime_animethemes_id ON anime(animethemes_id);

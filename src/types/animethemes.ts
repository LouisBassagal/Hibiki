export interface AnimeThemesAnime {
  slug: string
  year: number | null
  season: string | null
  images: ImageConnection
  animethemes: AnimeTheme[]
}

export interface ImageConnection {
  nodes: Image[]
}

export interface Image {
  link: string
}

export interface AnimeTheme {
  id: number
  type: string
  sequence: number | null
  slug: string
  song: Song | null
  animethemeentries: ThemeEntry[]
}

export interface Song {
  title: SongTitle | null
  performances: Performance[]
}

export interface SongTitle {
  romaji: string | null
}

export interface Performance {
  artist: Artist
}

export interface Artist {
  name: ArtistName
}

export interface ArtistName {
  main: string
}

export interface ThemeEntry {
  version: number | null
  episodes: string | null
  spoiler: boolean
  nsfw: boolean
  videos: VideoConnection
}

export interface VideoConnection {
  nodes: Video[]
}

export interface Video {
  basename: string
  filename: string
  nc: boolean
  link: string
  tags: string | null
  uncen: boolean
  source: string | null
  resolution: number | null
}

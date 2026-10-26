// ==========================================
// ENUMS & TYPES DE BASE
// ==========================================

export type MediaType = 'ANIME' | 'MANGA'

export type MediaFormat =
  | 'TV'
  | 'TV_SHORT'
  | 'MOVIE'
  | 'SPECIAL'
  | 'OVA'
  | 'ONA'
  | 'MUSIC'
  | 'MANGA'
  | 'NOVEL'
  | 'ONE_SHOT'

export type MediaStatus =
  | 'FINISHED'
  | 'RELEASING'
  | 'NOT_YET_RELEASED'
  | 'CANCELLED'
  | 'HIATUS'

export type MediaListStatus =
  | 'CURRENT'
  | 'PLANNING'
  | 'COMPLETED'
  | 'DROPPED'
  | 'PAUSED'
  | 'REPEATING'

export type MediaSeason = 'WINTER' | 'SPRING' | 'SUMMER' | 'FALL'

export type CharacterRole = 'MAIN' | 'SUPPORTING' | 'BACKGROUND'

// ==========================================
// SOUS-STRUCTURES (IMAGES, TITRES, DATES...)
// ==========================================

export interface MediaTitle {
  romaji?: string | null
  english?: string | null
  native?: string | null
  userPreferred?: string | null
}

export interface MediaCoverImage {
  extraLarge?: string | null
  large?: string | null
  medium?: string | null
  color?: string | null
}

export interface FuzzyDate {
  year?: number | null
  month?: number | null
  day?: number | null
}

export interface MediaTrailer {
  id?: string | null
  site?: string | null
  thumbnail?: string | null
}

export interface MediaTag {
  id: number
  name: string
  description?: string | null
  category?: string | null
  rank?: number | null
  isGeneralSpoiler?: boolean | null
  isMediaSpoiler?: boolean | null
  isAdult?: boolean | null
}

export interface CharacterName {
  first?: string | null
  middle?: string | null
  last?: string | null
  full?: string | null
  native?: string | null
  userPreferred?: string | null
}

export interface CharacterImage {
  large?: string | null
  medium?: string | null
}

export interface Character {
  id: number
  name?: CharacterName | null
  image?: CharacterImage | null
  description?: string | null
  gender?: string | null
  age?: string | null
}

export interface CharacterEdge {
  node?: Character | null
  id?: number | null
  role?: CharacterRole | null
  voiceActors?: VoiceActor[] | null
}

export interface CharacterConnection {
  edges?: CharacterEdge[] | null
  nodes?: Character[] | null
}

export interface VoiceActor {
  id: number
  name?: CharacterName | null
  languageV2?: string | null
  image?: CharacterImage | null
}

// ==========================================
// MEDIA (ANIME / MANGA)
// ==========================================

export interface Media {
  id: number
  idMal?: number | null
  title?: MediaTitle | null
  type?: MediaType | null
  format?: MediaFormat | null
  status?: MediaStatus | null
  description?: string | null
  startDate?: FuzzyDate | null
  endDate?: FuzzyDate | null
  season?: MediaSeason | null
  seasonYear?: number | null
  episodes?: number | null
  duration?: number | null
  chapters?: number | null
  volumes?: number | null
  countryOfOrigin?: string | null
  isLicensed?: boolean | null
  source?: string | null
  hashtag?: string | null
  trailer?: MediaTrailer | null
  updatedAt?: number | null
  coverImage?: MediaCoverImage | null
  bannerImage?: string | null
  genres?: string[] | null
  synonyms?: string[] | null
  averageScore?: number | null
  meanScore?: number | null
  popularity?: number | null
  isLocked?: boolean | null
  trending?: number | null
  favorites?: number | null
  tags?: MediaTag[] | null
  characters?: CharacterConnection | null
  isAdult?: boolean | null
  siteUrl?: string | null
  mediaListEntry?: MediaListEntry | null
}

export interface MediaListEntry {
  status?: MediaListStatus | null
  score?: number | null
  progress?: number | null
  startedAt?: FuzzyDate | null
  completedAt?: FuzzyDate | null
}

// ==========================================
// VIEWER (UTILISATEUR ANILIST)
// ==========================================

export interface ViewerAvatar {
  large?: string | null
  medium?: string | null
}

export interface ViewerAnimeStatistics {
  count?: number | null
  episodesWatched?: number | null
  minutesWatched?: number | null
  meanScore?: number | null
}

export interface ViewerStatistics {
  anime?: ViewerAnimeStatistics | null
}

export interface ViewerAnimeFavorites {
  nodes?: Media[] | null
}

export interface ViewerFavorites {
  anime?: ViewerAnimeFavorites | null
}

export interface Viewer {
  id: number
  name: string
  avatar?: ViewerAvatar | null
  bannerImage?: string | null
  siteUrl?: string | null
  statistics?: ViewerStatistics | null
  favourites?: ViewerFavorites | null
}

// ==========================================
// PAGINATION & REPONSES GRAPHQL
// ==========================================

export interface PageInfo {
  total?: number | null
  perPage?: number | null
  currentPage?: number | null
  lastPage?: number | null
  hasNextPage?: boolean | null
}

export interface AniListMediaPage {
  pageInfo?: PageInfo | null
  media?: Media[] | null
}

export interface AniListGraphQLResponse<T> {
  data: T
  errors?: Array<{
    message: string
    status?: number
    locations?: Array<{ line: number; column: number }>
  }>
}

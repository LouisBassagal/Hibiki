import { Media } from './anilist'
import { SongTitle, Video } from './animethemes'

export interface VideoExtended extends Video {
  title: SongTitle | undefined
  media: Media | undefined
}

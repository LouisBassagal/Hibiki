import { SongTitle, Video } from './animethemes'

export interface VideoExtended extends Video {
  title: SongTitle | undefined
}

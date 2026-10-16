import { VideoExtended } from './extended'

export interface Playlist {
  id: number
  name: string
  cover: Blob | null
  items: Array<VideoExtended>
}

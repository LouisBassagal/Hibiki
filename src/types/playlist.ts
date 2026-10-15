import { VideoExtended } from './extended'

export interface Playlist {
  id: number
  name: string
  items: Array<VideoExtended>
}

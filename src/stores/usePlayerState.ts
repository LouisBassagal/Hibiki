import { VideoExtended } from '@/types/extended'
import { defineStore } from 'pinia'

export const usePlayerStore = defineStore('player', {
  state: () => ({
    currentEntry: null as VideoExtended | null,
    // queue: Array<VideoExtended>,
    mode: 'hidden' as 'hidden' | 'mini' | 'full',
  }),

  actions: {
    play(entry: VideoExtended) {
      this.currentEntry = entry
      this.mode = 'full'
    },
    minimize() {
      this.mode = 'mini'
    },
    maximize() {
      this.mode = 'full'
    },
    close() {
      this.mode = 'hidden'
      this.currentEntry = null
    },
  },
})

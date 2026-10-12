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
      console.log('JE PASSE DANS LE PLAYER STATE')
      this.currentEntry = entry
      this.mode = 'mini'
    },
    minimize() {
      this.mode = 'mini'
    },
    expand() {
      this.mode = 'full'
    },
    close() {
      this.mode = 'hidden'
      this.currentEntry = null
    },
  },
})

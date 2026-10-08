import { VideoExtended } from '@/types/extended'
import { defineStore } from 'pinia'

export const usePlayerStore = defineStore('player', {
  state: () => ({
    currentEntry: null as VideoExtended | null,
    // queue: [] as ThemeEntry[],
    mode: 'hidden' as 'hidden' | 'mini' | 'full',
    isPlaying: false,
  }),
  actions: {
    play(entry: VideoExtended) {
      this.currentEntry = entry
      this.mode = 'full'
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

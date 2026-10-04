import { Video } from '@/types/animethemes'
import { defineStore } from 'pinia'

export const usePlayerStore = defineStore('player', {
  state: () => ({
    currentEntry: null as Video | null,
    // queue: [] as ThemeEntry[],
    mode: 'hidden' as 'hidden' | 'mini' | 'full',
    isPlaying: false,
  }),
  actions: {
    play(entry: Video) {
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

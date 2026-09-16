import { Media } from '@/types/anilist'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAnimeStore = defineStore('anime', () => {
  const anime = ref<Media | null>(null)

  const setAnime = (media: Media) => (anime.value = media)

  return { anime, setAnime }
})

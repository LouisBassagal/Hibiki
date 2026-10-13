import { VideoExtended } from '@/types/extended'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

export const usePlayerStore = defineStore('player', () => {
  const queue = ref<Array<VideoExtended>>([])
  const index = ref<number>(-1)
  const mode = ref<'hidden' | 'full' | 'mini'>('hidden')

  const current = computed(() => queue.value[index.value] ?? undefined)
  const hasNext = computed(() => index.value <= queue.value.length)
  const hasPrevious = computed(() => index.value > 0)

  const start = (entry: VideoExtended) => {
    queue.value.push(entry)
    index.value = 0
    mode.value = 'full'
  }

  const enqueue = (entry: VideoExtended) => {
    queue.value.push(entry)
  }

  const minimize = () => {
    mode.value = 'mini'
  }

  const maximize = () => {
    mode.value = 'full'
  }

  const close = () => {
    mode.value = 'hidden'
    index.value = -1
    queue.value = []
  }

  const next = () => {
    if (!hasNext) return
    index.value += 1
  }

  const previous = () => {
    if (!hasPrevious) return
    index.value -= 1
  }

  return {
    current,
    mode,
    hasNext,
    hasPrevious,
    start,
    enqueue,
    minimize,
    maximize,
    close,
    next,
    previous,
  }
})

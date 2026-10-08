<script setup lang="ts">
import { usePlayerStore } from '@/stores/usePlayerState'
import { invoke } from '@tauri-apps/api/core'
import { onMounted, onUnmounted } from 'vue'

const playerStore = usePlayerStore()
const { currentEntry } = playerStore

onMounted(async () => {
  await invoke('lock_orientation', { orientation: 'unlock' }).catch((err) => {
    console.error('Failed to lock orientation:', err)
  })
})

onUnmounted(async () => {
  await invoke('lock_orientation', { orientation: 'portrait' }).catch((err) => {
    console.error('Failed to lock orientation:', err)
  })
})
</script>

<template>
  <section
    class="mt-3 overflow-hidden rounded-sm bg-black shadow-xl shadow-black/30"
  >
    <VideoPlayer :video="currentEntry || undefined" />
  </section>
</template>

<script setup lang="ts">
import '@fontsource-variable/sora'
import '@fontsource-variable/plus-jakarta-sans'
import '~/src/global.css'
import { onBackButtonPress } from '@tauri-apps/api/app'
import { invoke, PluginListener } from '@tauri-apps/api/core'
import { onMounted, onUnmounted } from 'vue'

import { usePlayerStore } from './stores/usePlayerState'

onMounted(async () => {
  await invoke('lock_orientation', { orientation: 'portrait' }).catch((err) => {
    console.error('Failed to lock orientation:', err)
  })
})

let listener: PluginListener

onMounted(async () => {
  listener = await onBackButtonPress((payload) => {
    if (usePlayerStore().mode === 'full') return usePlayerStore().minimize()
    console.log(payload.canGoBack)
    if (payload.canGoBack) return window.history.back()
    else invoke('quit')
  })
})

onUnmounted(() => {
  listener.unregister()
})
</script>

<template>
  <Layout>
    <RouterView />
  </Layout>
  <VideoContainer />
</template>

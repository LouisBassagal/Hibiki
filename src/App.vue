<script setup lang="ts">
import '@fontsource-variable/sora'
import '@fontsource-variable/plus-jakarta-sans'
import '~/src/global.css'
import { invoke } from '@tauri-apps/api/core'
import { onMounted } from 'vue'

onMounted(async () => {
  await invoke('lock_orientation', { orientation: 'portrait' }).catch((err) => {
    console.error('Failed to lock orientation:', err)
  })
})
</script>

<template>
  <Layout>
    <RouterView />
  </Layout>
  <VideoContainer />
</template>

<script setup lang="ts">
import '@fontsource-variable/sora'
import '@fontsource-variable/plus-jakarta-sans'
import '~/src/global.css'
import { PhCheck, PhX } from '@phosphor-icons/vue'
import { onBackButtonPress } from '@tauri-apps/api/app'
import { invoke, PluginListener } from '@tauri-apps/api/core'
import { onMounted, onUnmounted, useTemplateRef } from 'vue'
import 'vue-sonner/style.css'
import { Toaster } from 'vue-sonner'

import { usePlayerStore } from './stores/usePlayerState'

const toastLayerRef = useTemplateRef('layer')

let listener: PluginListener

onMounted(async () => {
  listener = await onBackButtonPress((payload) => {
    if (usePlayerStore().mode === 'full') return usePlayerStore().minimize()
    console.log(payload.canGoBack)
    if (payload.canGoBack) return window.history.back()
    else invoke('quit')
  })

  await invoke('lock_orientation', { orientation: 'portrait' }).catch((err) => {
    console.error('Failed to lock orientation:', err)
  })

  toastLayerRef.value?.showPopover()
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
  <div ref="layer" popover="manual" class="toast-layer">
    <Toaster
      position="top-center"
      theme="dark"
      :duration="2500"
      :toast-options="{
        unstyled: true,
        classes: {
          toast:
            'bg-surface flex flex-row p-4 font-semibold items-center gap-2.5 rounded-sm',
          title: 'text-text font-text',
        },
      }"
    >
      <template #success-icon>
        <PhCheck :size="24" class="text-text" />
      </template>
      <template #error-icon>
        <PhX :size="24" class="text-text" />
      </template>
    </Toaster>
  </div>
</template>

<style scoped>
.toast-layer {
  inset: 0 auto auto 0;
  width: 0;
  height: 0;
  margin: 0;
  padding: 0;
  border: 0;
  overflow: visible;
  background: transparent;
}
</style>

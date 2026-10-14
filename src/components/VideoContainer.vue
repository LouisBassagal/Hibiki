<script setup lang="ts">
import { usePlayerStore } from '@/stores/usePlayerState'
import { useDraggable } from '@vueuse/core'
import { storeToRefs } from 'pinia'
import { useTemplateRef, watch, ref } from 'vue'

const containerRef = useTemplateRef('container')

const playerStore = usePlayerStore()
const isDragDisabled = ref(false)
const { current, mode } = storeToRefs(playerStore)

const { style } = useDraggable(containerRef, {
  initialValue: { x: 0, y: 50 },
  disabled: isDragDisabled,
})

watch(mode, (newMode) => {
  if (newMode === 'full') isDragDisabled.value = true
  if (newMode === 'mini') isDragDisabled.value = false
})
</script>

<template>
  <div
    v-if="mode !== 'hidden'"
    ref="container"
    class="video-container fixed top-0 bg-white w-60 overflow-hidden touch-none select-none z-50"
    :class="{
      'w-60 rounded-sm': mode === 'mini',
    }"
    :style="mode === 'mini' ? style : ''"
  >
    <VideoPlayer :video="current || undefined" />
  </div>
</template>

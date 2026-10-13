<script setup lang="ts">
import { usePlayerStore } from '@/stores/usePlayerState'
import { useDraggable } from '@vueuse/core'
import { storeToRefs } from 'pinia'
import { useTemplateRef } from 'vue'

const containerRef = useTemplateRef('container')

const playerStore = usePlayerStore()
const { current, mode } = storeToRefs(playerStore)

const { style } = useDraggable(containerRef, {
  initialValue: { x: 0, y: 50 },
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
    :style="style"
  >
    <VideoPlayer :video="current || undefined" />
  </div>
</template>

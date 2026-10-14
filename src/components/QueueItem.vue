<script setup lang="ts">
import { VideoExtended } from '@/types/extended'
import { PhMusicNotes } from '@phosphor-icons/vue'

const emits = defineEmits(['click'])
const props = defineProps<{
  video: VideoExtended
  isCurrent: boolean
}>()
</script>

<template>
  <button
    type="button"
    @click="emits('click')"
    class="touchable p-2.5 flex bg-surface flex-row justify-between rounded-sm w-full"
    :class="{ 'ring-1 ring-primary animate-pulse': isCurrent }"
  >
    <span class="flex flex-col text-sm text-left">
      <span class="text-text font-bold">{{ video.title?.romaji }}</span>
      <span class="text-text/75 text-sm">{{ video.media?.title?.romaji }}</span>
    </span>
    <PhMusicNotes
      :size="24"
      class="self-center shrink-0"
      :class="{ 'text-primary': isCurrent, 'text-text/50': !isCurrent }"
    />
  </button>
</template>

<script setup lang="ts">
import { AnimeTheme } from '@/types/animethemes'
import { PhCaretRight } from '@phosphor-icons/vue'

defineProps<{
  theme: AnimeTheme | null
}>()

const emits = defineEmits(['click'])
</script>

<template>
  <button
    class="bg-surface relative text-left font-text flex flex-col font-semibold w-full rounded-sm p-2.5 text-text touchable"
    @click="emits('click')"
  >
    <span>{{ theme?.song?.title?.romaji ?? '' }}</span>
    <span class="text-text/75">{{ theme?.slug }}</span>
    <PhCaretRight
      :size="32"
      class="absolute right-2.5 bottom-1/2 translate-y-1/2 text-primary"
    />
  </button>
</template>

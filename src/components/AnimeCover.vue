<script setup lang="ts">
import { Media } from '@/types/anilist.ts'

defineProps<{
  media: Media
}>()
</script>

<template>
  <button
    class="w-30.5 aspect-a4 rounded-md shrink-0 overflow-clip relative active:scale-95 active:opacity-75 transition-all"
    @click="$emit('click')"
  >
    <img
      class="h-full w-full object-cover"
      :src="media?.coverImage?.large"
      :alt="media?.title?.romaji"
    />

    <span
      class="absolute bottom-0 leading-4 left-0 line-clamp-5 max-h-20 overflow-hidden right-0 text-left rounded-md backdrop-blur-sm min-h-7.5 px-2.5 py-1.5 text-white font-bold font-text text-xs"
    >
      {{ media?.title?.romaji }}
    </span>
  </button>
</template>

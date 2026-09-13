<script setup lang="ts">
import { Media } from '@/types/anilist.ts'

defineProps<{
  media: Media
}>()
</script>

<template>
  <button class="w-30.5 aspect-a4 rounded-md shrink-0 overflow-clip relative">
    <img
      class="h-full w-full object-cover"
      :src="
        media?.coverImage?.medium ??
        'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx185874-aU3e6tBT6wwA.jpg'
      "
      :alt="media?.title?.english ?? 'BLEACH: Sennen Kessen-hen - Kashin-tan'"
    />

    <span
      class="absolute bottom-0 left-0 right-0 text-left rounded-md backdrop-blur-sm min-h-7.5 px-2.5 py-1.5 text-white font-bold font-text text-xs"
    >
      {{ media?.title?.english ?? 'BLEACH: Sennen Kessen-hen - Kashin-tan' }}
    </span>
  </button>
</template>

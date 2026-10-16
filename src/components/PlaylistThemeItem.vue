<script setup lang="ts">
import { VideoExtended } from '@/types/extended'

defineProps<{
  video: VideoExtended
}>()
</script>

<template>
  <button class="w-full flex flex-row rounded-sm p-2.5">
    <img
      :src="video?.media?.coverImage?.medium ?? ''"
      class="h-full aspect-a4"
    />

    <span class="flex flex-col">
      <span class="text-text font-semibold"> {{ video.title?.romaji }} </span>
      <span class="text-text/75">
        {{ video.media?.title?.romaji }}
      </span>
    </span>
  </button>
</template>

<script setup lang="ts">
import { Media } from '@/types/anilist.ts'

defineProps<{
  media: Media
}>()
</script>

<template>
  <div
    :style="{ '--shadow-color': media.coverImage?.color ?? '#000000' }"
    class="w-fit h-fit shrink-0 text-white overflow-hidden rounded-lg drop-shadow-[0_20px_20px_var(--shadow-color)] aspect-a4"
  >
    <img
      class="h-28 object-cover w-full text-amber-50 aspect-a4"
      :src="media?.coverImage?.medium ?? ''"
      :alt="media?.title?.english ?? ''"
    />
  </div>
</template>

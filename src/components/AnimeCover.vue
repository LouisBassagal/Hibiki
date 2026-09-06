<script setup lang="ts">
import { Media } from '@/types/anilist.ts'

defineProps<{
  media: Media
}>()
</script>

<template>
  <div
    :style="{ '--shadow-color': media.coverImage?.color ?? '#000000' }"
    class="zoom-50 shrink-0 text-white overflow-hidden rounded-lg drop-shadow-[0_40px_40px_var(--shadow-color)]"
  >
    <img
      class="object-fill w-full text-amber-50 h-full min-w-32 shrink-0"
      :src="media?.coverImage?.medium ?? ''"
      :alt="media?.title?.english ?? ''"
    />
  </div>
</template>

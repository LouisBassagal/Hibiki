<script setup lang="ts">
import { ViewerAnimeStatistics } from '@/types/anilist'

defineProps<{
  statistics: ViewerAnimeStatistics | undefined | null
}>()
</script>

<template>
  <div
    class="grid grid-cols-2 md:grid-cols-4 rounded-sm bg-surface divide-background divide-x-2 divide-y-2 md:divide-y-0"
  >
    <div class="px-5 py-5">
      <p class="font-title text-3xl font-bold">
        {{ statistics?.count }}
      </p>
      <span class="text-text font-text font-semibold">
        {{ $t('profile.viewedAnime') }}
      </span>
    </div>

    <div class="px-5 py-5">
      <p class="font-title text-3xl font-bold">
        {{ statistics?.episodesWatched }}
      </p>
      <span class="text-text font-text font-semibold">
        {{ $t('profile.viewedEpisode') }}
      </span>
    </div>

    <div class="px-5 py-5">
      <p class="font-title text-3xl font-bold">
        {{ statistics?.meanScore }}
      </p>
      <span class="text-text font-text font-semibold">
        {{ $t('profile.averageScore') }}
      </span>
    </div>

    <div class="px-5 py-5">
      <p class="font-title text-3xl font-bold">
        {{ statistics?.minutesWatched }}
      </p>
      <span class="text-text font-text font-semibold">
        {{ $t('profile.viewedMinutes') }}
      </span>
    </div>
  </div>
</template>

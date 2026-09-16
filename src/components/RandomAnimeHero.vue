<script setup lang="ts">
import { Media } from '@/types/anilist'

const props = defineProps<{
  media: Media
}>()

defineEmits<{
  (e: 'click'): void
}>()
</script>

<template>
  <div>
    <h2 class="text-text font-text font-bold text-2xl mb-4">
      {{ $t('home.randomAnime') }}
    </h2>

    <template v-if="props.media">
      <button
        class="w-full h-50 rounded-md overflow-clip relative active:scale-95 active:opacity-75 transition-all"
        @click="$emit('click')"
      >
        <img
          class="w-full h-full object-cover"
          :src="
            props.media?.bannerImage ?? props.media?.coverImage?.large ?? ''
          "
          :alt="
            props.media?.title?.romaji ??
            props.media?.title?.english ??
            props.media?.title?.native ??
            ''
          "
        />
        <span
          class="flex flex-col absolute line-clamp-3 text-left bottom-0 left-0 w-full justify-center align-middle rounded-md px-10 h-1/3 bg-surface/50 backdrop-blur-xs text-text font-title font-bold text-lg"
        >
          {{
            props.media?.title?.romaji ??
            props.media?.title?.english ??
            props.media?.title?.native ??
            ''
          }}
        </span>
      </button>
    </template>

    <template v-else>
      <div
        class="w-full h-50 rounded-md bg-surface/50 backdrop-blur-xs animate-pulse overflow-clip relative"
      />
    </template>
  </div>
</template>

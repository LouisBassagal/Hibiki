<script setup lang="ts">
import { Media } from '@/types/anilist.ts'

withDefaults(
  defineProps<{
    media: Media
    size?: 'sm' | 'md'
  }>(),
  {
    size: 'md',
  }
)
</script>

<template>
  <button
    type="button"
    class="aspect-a4 rounded-md shrink-0 overflow-clip relative touchable"
    @click="$emit('click')"
    :class="size === 'md' ? 'w-30.5' : 'w-26'"
  >
    <img
      class="h-full w-full object-cover"
      :src="media?.coverImage?.large ?? media?.coverImage?.medium ?? ''"
      :alt="
        media?.title?.romaji ??
        media?.title?.english ??
        media?.title?.native ??
        ''
      "
    />

    <span
      class="absolute bottom-0 bg-surface/50 leading-4 left-0 line-clamp-5 max-h-20 overflow-hidden right-0 text-left rounded-md backdrop-blur-sm min-h-7.5 px-2.5 py-1.5 text-white font-bold font-text text-xs"
    >
      {{
        media?.title?.romaji ??
        media?.title?.english ??
        media?.title?.native ??
        ''
      }}
    </span>
  </button>
</template>

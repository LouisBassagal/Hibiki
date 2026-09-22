<script setup lang="ts">
import { useAnimeStore } from '@/stores/AnimeStore'
import { Media } from '@/types/anilist.ts'
import { useRouter } from 'vue-router'

const props = withDefaults(
  defineProps<{
    media: Media
    size?: 'sm' | 'md'
  }>(),
  {
    size: 'md',
  }
)

const emits = defineEmits(['click'])

const { setAnime } = useAnimeStore()

const router = useRouter()

const onClick = () => {
  emits('click')
  setAnime(props.media)
  router.push({ name: 'anime', params: { id: props.media.id } })
}
</script>

<template>
  <button
    type="button"
    class="aspect-a4 rounded-md shrink-0 overflow-clip relative touchable"
    @click="onClick"
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

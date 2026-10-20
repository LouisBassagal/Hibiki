<script setup lang="ts">
import { useAnimeStore } from '@/stores/AnimeStore'
import { Media } from '@/types/anilist.ts'
import { useRouter } from 'vue-router'

const props = defineProps<{
  media: Media
}>()

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
    class="aspect-a4 rounded-md shrink-0 overflow-clip relative touchable w-36"
    @click="onClick"
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
      class="absolute bottom-0 left-0 bg-surface/50 w-full backdrop-blur-sm px-1.5 py-1.5"
    >
      <span
        class="line-clamp-5 overflow-hidden right-0 text-left rounded-md text-white font-bold font-text text-xs"
      >
        {{
          media?.title?.romaji ??
          media?.title?.english ??
          media?.title?.native ??
          ''
        }}
      </span>
    </span>
  </button>
</template>

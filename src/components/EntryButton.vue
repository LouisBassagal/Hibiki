<script setup lang="ts">
import { usePlayerStore } from '@/stores/usePlayerState'
import { SongTitle, ThemeEntry, Video } from '@/types/animethemes'
import { VideoExtended } from '@/types/extended.ts'

// import { PhHeart, PhPlus } from '@phosphor-icons/vue'
import StandardChip from './StandardChip.vue'

const props = defineProps<{
  title: SongTitle | undefined
  entry: ThemeEntry | undefined
  video: Video | undefined
}>()

const emits = defineEmits(['click'])

// const router = useRouter()
const playerStore = usePlayerStore()

const onSelectTheme = () => {
  if (!props.video) return
  const extended: VideoExtended = { ...props.video, title: props?.title }
  playerStore.play(extended)
  emits('click')
}
</script>

<template>
  <div
    class="relative bg-surface w-full flex flex-row text-left p-2.5 gap-2.5 rounded-sm text-text font-semibold font-text"
  >
    <button
      type="button"
      class="touchable w-full rounded-sm"
      @click="onSelectTheme"
    >
      <span class="flex flex-row gap-2.5 text-lg mb-2">
        {{ title?.romaji }}
        <span class="text-text/75">v{{ entry?.version }}</span>
      </span>
      <span class="flex flex-row gap-1">
        <StandardChip :content="video?.resolution + 'p'" />
        <StardardChip v-if="video?.nc" :content="'NC'" />
        <StandardChip v-if="entry?.spoiler" :content="'Spoiler'" />
        <StandardChip v-if="entry?.nsfw" :content="'NSFW'" />
      </span>
    </button>

    <!-- <div
      class="flex flex-row gap-2.5 text-text/75 right-2.5 justify-center items-center"
    >
      <button type="button" @click.stop class="touchable h-fit rounded-sm">
        <PhPlus :size="28" />
      </button>
      <button
        type="button"
        @click.stop
        class="touchable h-fit rounded-sm text-red-300"
      >
        <PhHeart :size="28" />
      </button>
    </div> -->
  </div>
</template>

<script setup lang="ts">
import { SongTitle, ThemeEntry, Video } from '@/types/animethemes'
import { PhDotsThreeVertical } from '@phosphor-icons/vue'
import { onUnmounted, ref, useTemplateRef } from 'vue'

const props = defineProps<{
  title: SongTitle | undefined
  entry: ThemeEntry | undefined
  video: Video | undefined
}>()

const emits = defineEmits<{ click: []; menu: [el: HTMLElement] }>()

const menuButtonRef = useTemplateRef('menuButton')
const isThemeMenuOpen = ref(false)

onUnmounted(() => {
  isThemeMenuOpen.value = false
})
</script>

<template>
  <div
    class="relative bg-surface w-full flex flex-row text-left p-2.5 gap-2.5 rounded-sm text-text font-semibold font-text"
  >
    <button
      type="button"
      class="touchable w-full rounded-sm"
      @click="emits('click')"
    >
      <span class="flex flex-row gap-2.5 text-lg mb-2 text-left">
        {{ title?.romaji }}
        <span class="text-text/75">v{{ entry?.version }}</span>
      </span>
      <span class="flex flex-row gap-1">
        <StandardChip :content="video?.resolution + 'p'" />
        <StandardChip v-if="video?.nc" :content="'NC'" />
        <StandardChip v-if="entry?.spoiler" :content="'Spoiler'" />
        <StandardChip v-if="entry?.nsfw" :content="'NSFW'" />
      </span>
    </button>

    <div
      class="flex flex-row gap-2.5 text-text/75 right-2.5 justify-center items-center"
    >
      <button
        ref="menuButton"
        type="button"
        @click.stop="emits('menu', menuButtonRef as HTMLElement)"
        class="touchable h-fit rounded-sm"
      >
        <PhDotsThreeVertical :size="28" />
      </button>
    </div>
  </div>
</template>

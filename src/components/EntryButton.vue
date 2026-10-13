<script setup lang="ts">
import { usePlayerStore } from '@/stores/usePlayerState'
import { SongTitle, ThemeEntry, Video } from '@/types/animethemes'
import { VideoExtended } from '@/types/extended.ts'
import { notify } from '@/utils/notify'
import { useFloating, offset, flip, shift, autoUpdate } from '@floating-ui/vue'
import { PhDotsThreeVertical } from '@phosphor-icons/vue'
import { onClickOutside } from '@vueuse/core'
import { onUnmounted, ref, useTemplateRef } from 'vue'
import { useI18n } from 'vue-i18n'

const props = defineProps<{
  title: SongTitle | undefined
  entry: ThemeEntry | undefined
  video: Video | undefined
}>()

const emits = defineEmits(['click'])

const playerStore = usePlayerStore()
const isThemeMenuOpen = ref(false)

const menuButtonRef = useTemplateRef('menuButton')
const themeMenuRef = useTemplateRef('themeMenu')

const { t } = useI18n()

const { floatingStyles } = useFloating(menuButtonRef, themeMenuRef, {
  open: isThemeMenuOpen,
  strategy: 'fixed',
  placement: 'left',
  middleware: [offset(4), flip(), shift({ padding: 8 })],
  whileElementsMounted: autoUpdate,
})

const onSelectTheme = () => {
  if (!props.video) return
  const extended: VideoExtended = { ...props.video, title: props?.title }
  playerStore.start(extended)
  emits('click')
}

const onAddToQueue = () => {
  if (!props.video) return
  const extended: VideoExtended = { ...props.video, title: props?.title }
  playerStore.enqueue(extended)
  isThemeMenuOpen.value = false
  notify.success(t('toast.addToQueue'))
}

onClickOutside(themeMenuRef, () => (isThemeMenuOpen.value = false))

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
      @click="onSelectTheme"
    >
      <span class="flex flex-row gap-2.5 text-lg mb-2 text-left">
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

    <div
      class="flex flex-row gap-2.5 text-text/75 right-2.5 justify-center items-center"
    >
      <button
        ref="menuButton"
        type="button"
        @click.stop="isThemeMenuOpen = !isThemeMenuOpen"
        class="touchable h-fit rounded-sm"
      >
        <PhDotsThreeVertical :size="28" />
      </button>
    </div>

    <Transition mode="in-out" name="reveal">
      <div
        v-if="isThemeMenuOpen"
        ref="themeMenu"
        :style="floatingStyles"
        class="text-text text-bold bg-background backdrop-blur-md shadow-black rounded-sm p-3.5"
      >
        <button type="button" class="touchable" @click="onAddToQueue">
          <span> {{ $t('tooltip.addToQueue') }} </span>
        </button>
      </div>
    </Transition>
  </div>
</template>

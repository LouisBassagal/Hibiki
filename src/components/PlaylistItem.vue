<script setup lang="ts">
import { Playlist } from '@/types/playlist'
import { PhPlaylist } from '@phosphor-icons/vue'
import { onLongPress } from '@vueuse/core'
import { computed, onUnmounted, useTemplateRef } from 'vue'

const props = defineProps<{
  playlist: Playlist
}>()

const emits = defineEmits<{
  click: []
  longPress: [el: HTMLElement]
}>()

const buttonRef = useTemplateRef('button')

const cover = computed(() => {
  if (props.playlist?.cover) return URL.createObjectURL(props.playlist?.cover)
  return ''
})

const onMouseUpCallback = (
  _duration: number,
  _distance: number,
  isLongPress: boolean
) => {
  if (!isLongPress) emits('click')
}

onLongPress(
  buttonRef,
  () => emits('longPress', buttonRef.value as HTMLElement),
  {
    modifiers: {
      prevent: true,
    },
    onMouseUp: onMouseUpCallback,
  }
)

onUnmounted(() => {
  URL.revokeObjectURL(cover.value)
})
</script>

<template>
  <button ref="button" class="shrink-0 touchable flex flex-col text-left">
    <span
      class="size-37.5 relative bg-primary/50 rounded-sm flex justify-center items-center mb-1 overflow-hidden"
    >
      <PhPlaylist v-if="!playlist?.cover" :size="64" />
      <img class="absolute inset-0 pointer-events-none" :src="cover" />
    </span>
    <span class="text-text font-bold pointer-events-none">{{
      playlist.name
    }}</span>
    <span class="text-text/75 pointer-events-none">
      {{ $t('general.themes', playlist.items.length) }}
    </span>
  </button>
</template>

<script setup lang="ts">
import { Playlist } from '@/types/playlist'
import { PhPlaylist } from '@phosphor-icons/vue'
import { onLongPress } from '@vueuse/core'
import { useTemplateRef } from 'vue'

defineProps<{
  playlist: Playlist
}>()

const emits = defineEmits<{
  click: []
  longPress: [el: HTMLElement]
}>()

const buttonRef = useTemplateRef('button')

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
</script>

<template>
  <button ref="button" class="shrink-0 touchable flex flex-col text-left">
    <span
      class="size-37.5 bg-primary/50 rounded-sm flex justify-center items-center"
    >
      <PhPlaylist :size="64" />
    </span>
    <span class="text-text font-bold">{{ playlist.name }}</span>
  </button>
</template>

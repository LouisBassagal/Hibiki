<script setup lang="ts">
import { onLongPress } from '@vueuse/core'
import { useTemplateRef } from 'vue'
defineProps<{
  name: string
}>()

const emits = defineEmits<{
  click: []
  longPress: [el: HTMLElement]
}>()

const buttonRef = useTemplateRef('button')

const onMouseUpCallback = (
  duration: number,
  distance: number,
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
    <span class="size-37.5 bg-surface rounded-sm animate-pulse"></span>
    <span class="text-text font-bold">{{ name }}</span>
  </button>
</template>

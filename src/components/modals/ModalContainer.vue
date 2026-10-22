<script setup lang="ts">
import { PhX } from '@phosphor-icons/vue'
import { useTemplateRef, watch } from 'vue'

const emits = defineEmits(['close'])
const dialogRef = useTemplateRef('dialog')
const isOpen = defineModel('isOpen')

watch(isOpen, (newValue) => {
  if (newValue) dialogRef.value?.showModal()
  else dialogRef.value?.close()
})
</script>

<template>
  <dialog
    ref="dialog"
    class="relative m-auto w-[min(342px,calc(100vw-2rem))] rounded-2xl bg-background p-5 ring ring-terciary/50 backdrop:backdrop-blur-sm backdrop:bg-background/25 md:w-110 backdrop:pointer-events-none"
  >
    <button
      type="button"
      class="absolute top-5 right-5 touchable"
      @click="emits('close')"
    >
      <PhX :size="24" class="text-text" />
    </button>
    <slot />
  </dialog>
</template>

<style scoped>
dialog {
  /* état fermé */
  opacity: 0;
  translate: 0 16px;
  scale: 0.5;

  transition:
    opacity 0.25s ease,
    translate 0.25s ease,
    scale 0.25s ease,
    overlay 0.25s ease allow-discrete,
    display 0.25s ease allow-discrete;
}

dialog[open] {
  opacity: 1;
  translate: 0 0;
  scale: 1;
}

@starting-style {
  dialog[open] {
    opacity: 0;
    translate: 0 16px;
    scale: 0.5;
  }
}

@media (prefers-reduced-motion: reduce) {
  dialog,
  dialog::backdrop {
    transition-duration: 0.01ms;
  }
}
</style>

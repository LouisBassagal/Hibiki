<script setup lang="ts">
import ArrowRight from '@primeicons/vue/arrow-right'

defineEmits<{
  (e: 'click'): void
}>()
</script>

<template>
  <button
    @click="$emit('click')"
    class="w-full px-15 h-50 rounded-md overflow-clip relative max-h-20 bg-surface/50 backdrop-blur-xs text-text font-text font-bold text-lg active:scale-95 active:opacity-75 transition-all"
  >
    <span class="absolute -bottom-1 -left-1 text-primary/50">
      <slot name="icon" />
    </span>

    <span
      class="flex flex-col text-right w-full justify-center align-middle h-full"
    >
      <slot />
    </span>

    <span class="absolute top-1/2 -translate-y-1/2 right-4 text-primary">
      <ArrowRight :size="35" />
    </span>
  </button>
</template>

<script setup lang="ts">
defineEmits<{
  (e: 'click'): void
}>()
</script>

<template>
  <button
    @click="$emit('click')"
    class="w-full px-15 h-50 rounded-md overflow-clip relative max-h-20 bg-surface/50 backdrop-blur-xs text-text font-title font-bold text-lg"
  >
    <span class="absolute -bottom-1 -left-1">
      <slot name="icon" />
    </span>

    <span
      class="flex flex-col text-right w-full justify-center align-middle h-full"
    >
      <slot />
    </span>
  </button>
</template>

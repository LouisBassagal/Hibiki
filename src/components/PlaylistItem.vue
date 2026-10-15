<script setup lang="ts">
defineProps<{
  name: string
}>()

const emits = defineEmits(['click'])
</script>

<template>
  <button
    class="shrink-0 touchable flex flex-col text-left"
    @click="emits('click')"
  >
    <span class="size-37.5 bg-surface rounded-sm animate-pulse"></span>
    <span class="text-text font-bold">{{ name }}</span>
  </button>
</template>

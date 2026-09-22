<script lang="ts" setup>
defineProps<{
  content: string | undefined | null
}>()
</script>

<template>
  <div v-show="content" class="p-1 bg-primary rounded-sm text-xs">
    {{ content }}
  </div>
</template>

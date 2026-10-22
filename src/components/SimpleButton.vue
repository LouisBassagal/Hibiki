<script setup lang="ts">
withDefaults(
  defineProps<{
    disabled?: boolean
  }>(),
  {
    disabled: false,
  }
)

const emits = defineEmits(['click'])
</script>

<template>
  <button
    @click="emits('click')"
    :disabled="disabled"
    type="button"
    class="px-5 py-2.5 w-full rounded-sm font-semibold touchable"
  >
    <slot />
  </button>
</template>

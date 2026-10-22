<script setup lang="ts">
defineProps<{
  title: string
  subtitle: string
}>()
const emits = defineEmits(['accept', 'cancel', 'close'])

const isOpen = defineModel('isOpen')
</script>

<template>
  <ModalContainer v-model:is-open="isOpen" @close="emits('close')">
    <div class="mb-2.5 text-text">
      <h2 class="text-text font-bold text-lg pr-12">
        {{ title }}
      </h2>
      <span class="text-text/75 text-sm">{{ subtitle }}</span>
    </div>

    <div class="flex flex-row justify-end gap-2.5 text-text font-semibold">
      <button
        class="px-5 py-2.5 rounded-sm font-semibold touchable"
        @click="emits('cancel')"
      >
        {{ $t('general.no') }}
      </button>

      <button
        class="px-5 py-2.5 bg-primary rounded-sm font-semibold touchable"
        @click="emits('accept')"
      >
        {{ $t('general.yes') }}
      </button>
    </div>
  </ModalContainer>
</template>

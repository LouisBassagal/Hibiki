<script setup lang="ts">
import { PhCaretDown } from '@phosphor-icons/vue'
import { useTemplateRef } from 'vue'

const isOptionListOpen = defineModel<boolean>('isOpen', { default: false })
const containerRef = useTemplateRef('container')
const selected = defineModel<string>('name', { required: true })
</script>

<template>
  <div
    ref="container"
    class="relative flex flex-row items-center justify-between gap-2.5 bg-background h-fit rounded-sm ring-1 ring-terciary/50 px-2.5 py-1 min-w-36"
  >
    {{ selected }}
    <PhCaretDown
      class="transition-all duration-300"
      :class="{ 'rotate-180': isOptionListOpen }"
    />

    <button
      type="button"
      class="absolute inset-0"
      @click="!isOptionListOpen ? (isOptionListOpen = true) : null"
    />

    <ContextMenuContainer
      v-model:is-open="isOptionListOpen"
      :reference="containerRef ?? undefined"
    >
      <slot />
    </ContextMenuContainer>
  </div>
</template>

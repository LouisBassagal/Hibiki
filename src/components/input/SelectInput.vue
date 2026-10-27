<script setup lang="ts" generic="T extends string | number">
import { PhCaretDown, PhCheck } from '@phosphor-icons/vue'
import { computed, ref, useTemplateRef } from 'vue'

const props = defineProps<{
  options: { value: T; label: string }[]
  placeholder?: string
  label?: string
}>()

const model = defineModel<T>()
const isOpen = ref(false)
const containerRef = useTemplateRef<HTMLElement>('container')
const buttonRef = useTemplateRef('button')
const menuWidth = computed(() => {
  if (buttonRef.value?.offsetWidth) return buttonRef.value.offsetWidth - 20
  return undefined
})

const selectedLabel = computed(
  () =>
    props.options.find((o) => o.value === model.value)?.label ??
    props.placeholder ??
    ''
)

const choose = (value: T) => {
  model.value = value
  isOpen.value = false
  console.log(buttonRef.value?.offsetWidth)
}
</script>

<template>
  <div class="flex flex-col text-text font-text font-semibold gap-1">
    <label v-if="label">{{ label }}</label>
    <div ref="container" class="relative min-w-36">
      <button
        type="button"
        ref="button"
        class="flex w-full h-10 items-center justify-between gap-2.5 rounded-sm bg-surface px-2.5 py-1 text-left ring-1 ring-terciary/50"
        aria-haspopup="listbox"
        :aria-expanded="isOpen"
        @click="isOpen = true"
      >
        <span :class="{ 'text-text/50': model === undefined }">
          {{ selectedLabel }}
        </span>
        <PhCaretDown
          class="shrink-0 transition-transform duration-300"
          :class="{ 'rotate-180': isOpen }"
        />
      </button>

      <ContextMenuContainer
        v-model:is-open="isOpen"
        :teleport="false"
        :reference="containerRef ?? undefined"
      >
        <div
          role="listbox"
          class="flex flex-col"
          :style="{ minWidth: menuWidth + 'px' }"
        >
          <button
            v-for="option in options"
            :key="option.value"
            type="button"
            role="option"
            :aria-selected="option.value === model"
            class="touchable flex items-center justify-between gap-2.5 px-2.5 py-2 text-left"
            @click="choose(option.value)"
          >
            {{ option.label }}
            <PhCheck
              v-if="option.value === model"
              :size="18"
              class="text-primary"
            />
          </button>
        </div>
      </ContextMenuContainer>
    </div>
  </div>
</template>

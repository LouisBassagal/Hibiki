<script setup lang="ts">
import { autoUpdate, flip, offset, shift, useFloating } from '@floating-ui/vue'
import type { ReferenceElement } from '@floating-ui/vue'
import { onClickOutside } from '@vueuse/core'
import { computed, useTemplateRef } from 'vue'

const props = withDefaults(
  defineProps<{
    reference: ReferenceElement | undefined
    teleport?: boolean
  }>(),
  {
    teleport: true,
  }
)
const isOpen = defineModel('isOpen', { default: false })

const floating = useTemplateRef('floating')
const reference = computed(() => props.reference)

const { floatingStyles } = useFloating(reference, floating, {
  placement: 'bottom',
  middleware: [offset(6), flip(), shift({ padding: 8 })],
  whileElementsMounted: autoUpdate,
})

onClickOutside(floating, () => {
  isOpen.value = false
})
</script>

<template>
  <Teleport to="body" :disabled="!teleport">
    <Transition name="reveal">
      <div
        v-if="isOpen"
        ref="floating"
        role="menu"
        :style="floatingStyles"
        class="bg-surface rounded-sm text-text p-2.5 ring ring-terciary/50"
      >
        <slot />
      </div>
    </Transition>
  </Teleport>
</template>

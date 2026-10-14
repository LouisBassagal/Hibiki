<script setup lang="ts">
import { usePlayerStore } from '@/stores/usePlayerState'
import { PhX } from '@phosphor-icons/vue'
import { onClickOutside } from '@vueuse/core'
import { storeToRefs } from 'pinia'
import { useTemplateRef } from 'vue'

const queuePanelRef = useTemplateRef('queuePanel')
const isOpen = defineModel('isOpen')
const { queue } = storeToRefs(usePlayerStore())

onClickOutside(queuePanelRef, () => (isOpen.value = false))
</script>

<template>
  <div
    ref="queuePanel"
    class="absolute right-0 top-0 bottom-0 bg-background w-75 rounded-l-sm p-5"
  >
    <div class="flex flex-row w-full">
      <div class="flex flex-col w-full">
        <span class="font-bold text-lg text-text">{{
          $t('queue.panelTitle')
        }}</span>
        <span class="text-text/75">{{
          $t('queue.panelSubtitle', queue.length)
        }}</span>
      </div>

      <button type="button" @click.stop="isOpen = false">
        <PhX :size="24" />
      </button>
    </div>
  </div>
</template>

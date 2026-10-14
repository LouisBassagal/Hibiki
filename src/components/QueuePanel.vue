<script setup lang="ts">
import { usePlayerStore } from '@/stores/usePlayerState'
import { PhX } from '@phosphor-icons/vue'
import { onClickOutside } from '@vueuse/core'
import { storeToRefs } from 'pinia'
import { useTemplateRef } from 'vue'

const queuePanelRef = useTemplateRef('queuePanel')
const isOpen = defineModel('isOpen')
const { queue, index } = storeToRefs(usePlayerStore())

const onItemClicked = (idx: number) => {
  index.value = idx
  isOpen.value = false
}

onClickOutside(queuePanelRef, () => (isOpen.value = false))
</script>

<template>
  <div
    ref="queuePanel"
    class="absolute flex flex-col right-0 top-0 bottom-0 bg-background w-75 rounded-l-sm p-5"
  >
    <div class="flex flex-row w-full mb-2.5">
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

    <div
      class="flex flex-col gap-2.5 overflow-y-auto py-2.5 px-0.5 overscroll-contain"
    >
      <QueueItem
        v-for="(item, idx) in queue"
        :video="item"
        :is-current="index === idx"
        @click="onItemClicked(idx)"
      />
    </div>
  </div>
</template>

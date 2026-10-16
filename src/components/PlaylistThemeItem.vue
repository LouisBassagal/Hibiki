<script setup lang="ts">
import { VideoExtended } from '@/types/extended'
import { PhTrashSimple } from '@phosphor-icons/vue'

const emits = defineEmits(['click', 'delete'])

defineProps<{
  video: VideoExtended
  isEditMode: boolean
}>()
</script>

<template>
  <div class="relative">
    <button
      :disabled="isEditMode"
      type="button"
      @click="emits('click')"
      class="w-full flex flex-row rounded-sm p-2.5 bg-surface h-22"
      :class="{ touchable: !isEditMode }"
    >
      <img
        :src="video?.media?.coverImage?.medium ?? ''"
        class="h-full aspect-a4 overflow-hidden rounded-sm mr-2.5 shrink-0"
      />

      <span class="flex flex-col text-left justify-center w-full pr-10">
        <span class="text-text font-semibold"> {{ video.title?.romaji }} </span>
        <span class="text-text/75 text-sm">
          {{ video.media?.title?.romaji }}
        </span>
      </span>
    </button>

    <button
      v-if="isEditMode"
      class="absolute touchable text-text right-2.5 top-1/2 -translate-y-1/2"
      type="button"
      @click="emits('delete')"
    >
      <PhTrashSimple :size="24" />
    </button>
  </div>
</template>

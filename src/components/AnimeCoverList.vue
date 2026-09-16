<script setup lang="ts">
import type { Media } from '@/types/anilist.ts'
import { computed } from 'vue'

const props = defineProps<{
  title: string
  medias: Media[]
}>()

const hasMedia = computed(() => props.medias.length !== 0)
</script>

<template>
  <div class="flex flex-col gap-2.5">
    <h2 class="text-text font-text font-bold text-2xl">{{ title }}</h2>
    <div class="flex flex-row gap-2.5 overflow-x-auto px-2 py-3">
      <Transition name="reveal" mode="out-in">
        <div v-if="hasMedia" key="media" class="flex gap-2.5">
          <AnimeCover v-for="media in medias" :key="media.id" :media="media" />
        </div>

        <!-- Skeleton loading -->
        <div v-else key="skeleton" class="flex flex-row gap-2.5">
          <AnimeCoverSkeleton v-for="n in 3" :key="n" />
        </div>
      </Transition>
    </div>
  </div>
</template>

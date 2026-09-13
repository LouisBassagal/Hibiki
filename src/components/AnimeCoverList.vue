<script setup lang="ts">
import type { Media } from '@/types/anilist.ts'
import { computed } from 'vue'

const props = defineProps<{
  title: string
  media: Media[]
}>()

const isLoading = computed(() => props.media.length !== 0)
</script>

<template>
  <div class="flex flex-col gap-2.5">
    <h2 class="text-text font-text font-bold text-2xl">{{ title }}</h2>
    <Transition>
      <div class="flex flex-row gap-2.5 overflow-x-auto px-2 py-3">
        <template v-if="isLoading">
          <AnimeCover
            v-for="i in 5"
            :key="i"
            :media="{ coverImage: { color: '#000000' } }"
          />
        </template>

        <!-- For skeleton loading -->
        <template v-else>
          <AnimeCoverSkeleton v-for="n in 3" :key="n" />
        </template>
      </div>
    </Transition>
  </div>
</template>

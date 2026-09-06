<script setup lang="ts">
import AnimeCover from '@/components/AnimeCover.vue'
import { Media } from '@/types/anilist.ts'
import { motion } from 'motion-v'

defineProps<{
  direction: 'left' | 'right'
  medias: Media[]
}>()

const transition = {
  duration: 5,
  ease: 'linear',
  repeat: Infinity,
}
</script>

<template>
  <motion.div
    :transition="transition"
    :initial="{ x: '0%' }"
    :animate="{ x: direction === 'left' ? '-200%' : '200%' }"
  >
    <div class="flex flex-column gap-5">
      <template v-for="media in medias" :key="media.id">
        <AnimeCover :media="media" />
      </template>
    </div>
  </motion.div>
</template>

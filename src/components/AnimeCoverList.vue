<script setup lang="ts">
import AnimeCover from '@/components/AnimeCover.vue'
import type { Media } from '@/types/anilist.ts'
import { motion, useAnimationControls } from 'motion-v'
import { onMounted, onUnmounted, useTemplateRef, watch } from 'vue'

const props = withDefaults(
  defineProps<{
    medias: Media[]
    direction?: 'left' | 'right'
  }>(),
  {
    direction: 'left',
  }
)

const speed = 60
const controls = useAnimationControls()
const listRef = useTemplateRef('anime-cover-list')

const loop = (): void => {
  const element = listRef.value

  if (!element) return

  const distance = element.offsetWidth
  const duration = distance / speed
  const sign = props.direction == 'left' ? -1 : 1

  controls.set({
    x: '0px',
  })
  controls.start({
    x: distance * sign,
    transition: { duration, ease: 'linear', repeat: Infinity },
  })
}

onMounted(() => {
  window.addEventListener('resize', loop)
  loop()
})

onUnmounted(() => {
  window.removeEventListener('resize', loop)
  controls.stop()
})

watch(() => [props.medias, props.direction], loop)
</script>

<template>
  <div class="w-full">
    <motion.div :animate="controls" class="relative w-max flex flex-row">
      <template v-if="direction === 'right'">
        <div class="absolute flex gap-5 -left-full pr-5">
          <template v-for="media in medias" :key="'left-' + media.id">
            <AnimeCover :media="media" />
          </template>
        </div>
      </template>

      <div ref="anime-cover-list" class="flex gap-5 pr-5">
        <template v-for="media in medias" :key="'middle-' + media.id">
          <AnimeCover :media="media" />
        </template>
      </div>

      <template v-if="direction === 'left'">
        <div class="left-full flex gap-5">
          <template v-for="media in medias" :key="'right-' + media.id">
            <AnimeCover :media="media" />
          </template>
        </div>
      </template>
    </motion.div>
  </div>
</template>

<script setup lang="ts">
import { Media } from '@/types/anilist'
import { PhArrowRight, PhShuffle } from '@phosphor-icons/vue'

const props = defineProps<{
  media: Media | null
}>()

defineEmits<{
  (e: 'click'): void
}>()
</script>

<template>
  <div>
    <h2 class="text-text font-text font-bold text-2xl mb-4">
      {{ $t('home.randomAnime') }}
    </h2>

    <Transition name="reveal" mode="out-in">
      <template v-if="props.media">
        <button
          type="button"
          class="w-full h-50 md:h-62.5 rounded-md overflow-clip relative touchable"
          @click="$emit('click')"
        >
          <img
            v-if="props.media?.bannerImage || props.media.coverImage?.large"
            class="w-full h-full object-cover"
            :src="
              props.media?.bannerImage ?? props.media?.coverImage?.large ?? ''
            "
            :alt="
              props.media?.title?.romaji ??
              props.media?.title?.english ??
              props.media?.title?.native ??
              ''
            "
          />

          <span v-else class="w-full h-full bg-surface" />

          <span
            class="flex flex-col absolute inset-0 bg-surface/50 backdrop-blur-xs text-text font-bold font-title justify-end items-center text-2xl py-2.5"
          >
            <span
              class="flex flex-row absolute text-text text-sm font-text top-2.5 left-2.5 bg-primary px-2.5 py-1 rounded-sm gap-2.5 justify-center items-center"
            >
              <PhShuffle :size="24" />
              {{ $t('home.random') }}
            </span>

            <span class="absolute right-2.5 bottom-2.5">
              <PhArrowRight :size="48" class="text-text" />
            </span>

            <span>
              {{
                props.media?.title?.romaji ??
                props.media?.title?.english ??
                props.media?.title?.native ??
                ''
              }}
            </span>
          </span>
        </button>
      </template>

      <template v-else>
        <div
          class="w-full h-50 rounded-md bg-surface/50 backdrop-blur-xs animate-pulse overflow-clip relative"
        />
      </template>
    </Transition>
  </div>
</template>

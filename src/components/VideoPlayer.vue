<script setup lang="ts">
import { usePlayerStore } from '@/stores/usePlayerState'
import { VideoExtended } from '@/types/extended'
import { secondsToTime } from '@/utils/math'
import {
  PhPlay,
  PhPause,
  PhCornersIn,
  PhCornersOut,
  PhCircleNotch,
  PhX,
  PhPlaylist,
  PhSkipForward,
  PhSkipBack,
} from '@phosphor-icons/vue'
import { invoke } from '@tauri-apps/api/core'
import { useMediaControls } from '@vueuse/core'
import { storeToRefs } from 'pinia'
import { computed, onUnmounted, useTemplateRef } from 'vue'
import { ref, watch } from 'vue'

const props = defineProps<{
  video: VideoExtended | undefined
  poster?: string
}>()

const { mode, hasNext, hasPrevious } = storeToRefs(usePlayerStore())

// CONST
const BLACK_POSTER =
  'data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"><rect width="1" height="1" fill="black"/></svg>'

// References
const videoRef = useTemplateRef<HTMLVideoElement>('video')

const source = computed(() => props.video?.link ?? '')

const { playing, currentTime, duration, waiting } = useMediaControls(videoRef, {
  src: source,
})

// Values
const showControls = ref(true)
const isFullscreen = ref(false)
const isQueueOpen = ref(false)
const progress = computed(() => {
  return duration.value ? (currentTime.value / duration.value) * 100 : 0
})

const onFullscreenEnter = async () => {
  isFullscreen.value = true
  await invoke('lock_orientation', { orientation: 'sensor_landscape' }).catch(
    (err) => {
      console.error('Failed to lock orientation:', err)
    }
  )
  await invoke('set_fullscreen', { hide: true }).catch((err) => {
    console.error('Failed to hide android layout:', err)
  })
}

const onFullscreenExit = async () => {
  isFullscreen.value = false
  await invoke('lock_orientation', { orientation: 'portrait' }).catch((err) => {
    console.error('Failed to lock orientation:', err)
  })
  await invoke('set_fullscreen', { hide: false }).catch((err) => {
    console.error('Failed to hide android layout:', err)
  })
}

const closeControls = () => {
  if (isQueueOpen) isQueueOpen.value = false
  showControls.value = false
}

const onPlay = () => {
  console.log(source)
  playing.value = true
}

onUnmounted(async () => {
  onFullscreenExit()
})

watch(
  mode,
  (newValue) => {
    if (newValue === 'full') onFullscreenEnter()
    if (newValue === 'mini') onFullscreenExit()
  },
  { immediate: true }
)

watch(currentTime, (newValue) => {
  if (newValue < duration.value || hasNext) return
  usePlayerStore().next()
})
</script>

<template>
  <div
    id="player"
    ref="player"
    class="player text-text"
    :class="{ 'player--fullscreen': isFullscreen }"
  >
    <video
      ref="video"
      class="media"
      :poster="poster || BLACK_POSTER"
      playsinline
      preload="auto"
      @click="showControls = true"
      @canplay="onPlay"
    />

    <!-- Loader -->
    <div
      v-if="waiting"
      class="absolute flex justify-center items-center inset-0 pointer-events-none z-10"
    >
      <PhCircleNotch class="animate-spin" :size="isFullscreen ? 72 : 42" />
    </div>

    <Transition name="reveal" mode="out-in">
      <div
        class="bg-background/50 absolute inset-0"
        v-show="showControls"
        @click="closeControls"
      >
        <!-- Previous and Next buttons -->
        <div
          class="absolute flex flex-row w-full justify-center top-1/2 -translate-y-1/2"
          :class="{ 'gap-30': isFullscreen, 'gap-10': !isFullscreen }"
        >
          <button
            type="button"
            @click.stop="usePlayerStore().previous()"
            :disabled="!hasPrevious"
            class="touchable disabled:opacity-20"
          >
            <PhSkipBack :size="24" />
          </button>
          <button
            type="button"
            @click.stop="usePlayerStore().next()"
            :disabled="!hasNext"
            class="touchable disabled:opacity-20"
          >
            <PhSkipForward :size="24" />
          </button>
        </div>

        <div
          class="absolute w-full flex flex-row justify-between top-3 font-title font-bold"
          :class="{ 'px-5': isFullscreen, 'px-2.5': !isFullscreen }"
        >
          <span v-if="isFullscreen">
            {{ video?.title?.romaji }}
          </span>
          <div v-else />

          <button
            v-if="isFullscreen"
            @click.stop="isQueueOpen = true"
            class="touchable"
          >
            <PhPlaylist :size="24" />
          </button>
          <button
            v-else
            class="touchable"
            type="button"
            @click="usePlayerStore().close()"
          >
            <PhX :size="24" />
          </button>
        </div>

        <!-- Bottom part of player -->
        <div
          class="controls absolute flex flex-col bottom-0 left-0 right-0 text-text"
          :class="{ 'mx-5 py-5': isFullscreen, 'mx-1 py-2': !isFullscreen }"
        >
          <div class="flex flex-col w-full">
            <!-- Top part of the controls -->
            <div
              class="relative flex flex-row items-center w-full justify-between px-2.5 mb-2"
            >
              <div class="flex flex-row">
                <button
                  type="button"
                  class="touchable"
                  @click.stop="playing = !playing"
                >
                  <PhPlay :size="24" v-if="!playing" />
                  <PhPause :size="24" v-else />
                </button>
              </div>

              <div class="flex flex-row">
                <span
                  class="text-text relative text-sm font-text self-center mr-2.5 before:content-[00:00]"
                >
                  {{ secondsToTime(currentTime) }} /
                  <span class="font-bold">
                    {{ secondsToTime(duration) }}
                  </span>
                </span>

                <button
                  type="button"
                  class="touchable"
                  @click.stop="
                    !isFullscreen
                      ? usePlayerStore().maximize()
                      : usePlayerStore().minimize()
                  "
                >
                  <PhCornersOut v-if="!isFullscreen" :size="24" />
                  <PhCornersIn v-else :size="24" />
                </button>
              </div>
            </div>

            <!-- Bottom part of controls : Player track -->
            <div
              class="bottom-full justify-center w-full rounded-sm flex flex-row grow self-center mx-3"
            >
              <input
                type="range"
                min="0"
                :max="duration"
                step="0.01"
                v-model="currentTime"
                class="video-slider w-full mx-3"
                :style="{
                  '--fill': `${progress}%`,
                }"
              />
            </div>
          </div>
        </div>
      </div>
    </Transition>
    <Transition name="slide">
      <QueuePanel v-show="isQueueOpen" v-model:is-open="isQueueOpen" />
    </Transition>
  </div>
</template>

<style scoped>
.player {
  position: relative;
  width: 100%;
  aspect-ratio: 16 / 9;
  background: #000;
  overflow: hidden;
}

.player--fullscreen {
  position: fixed;
  inset: 0;
  z-index: 9999;
  width: 100vw;
  height: 100dvh;
  aspect-ratio: auto;
}

.media {
  display: block;
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.controls.hidden {
  opacity: 0;
  pointer-events: none;
}

.volume {
  writing-mode: vertical-lr;
  rotate: 180deg;
}

.volume-slider {
  -webkit-appearance: none;
  appearance: none;
  background: transparent;
  direction: ltr;
}

.volume-slider::-webkit-slider-runnable-track {
  width: 7px;
  height: 100%;
  border-radius: 5px;
  background: linear-gradient(
    to bottom,
    var(--color-primary) var(--fill),
    rgb(255 255 255 / 0.3) var(--fill)
  );
}

.volume-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 0px;
  height: 0px;
  border-radius: 5px;
  background: var(--color-primary);
}

.video-slider {
  -webkit-appearance: none;
  appearance: none;
  background: transparent;
}

.video-slider::-webkit-slider-runnable-track {
  height: 4px;
  border-radius: 999px;
  background: linear-gradient(
    to right,
    var(--color-primary) var(--fill),
    rgb(255 255 255 / 0.3) var(--fill)
  );
}

.video-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 0px;
  height: 0px;
  border-radius: 50%;
  background: var(--color-primary);
  margin-top: -5px;
}

.slide-enter-active,
.slide-leave-active {
  transition:
    opacity 0.3s ease,
    translate 0.3s ease;
}

.slide-enter-from,
.slide-leave-to {
  opacity: 0;
  translate: 100% 0;
}
</style>

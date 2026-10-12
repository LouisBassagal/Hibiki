<script setup lang="ts">
import { VideoExtended } from '@/types/extended'
import { secondsToTime } from '@/utils/math'
import {
  PhPlay,
  PhPause,
  PhCornersIn,
  PhCornersOut,
  PhCircleNotch,
} from '@phosphor-icons/vue'
import { invoke } from '@tauri-apps/api/core'
import { useMediaControls, onClickOutside, useFullscreen } from '@vueuse/core'
import { computed, onUnmounted, useTemplateRef } from 'vue'
import { ref } from 'vue'

const props = defineProps<{
  video: VideoExtended | undefined
  poster?: string
}>()

// CONST
const BLACK_POSTER =
  'data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"><rect width="1" height="1" fill="black"/></svg>'

// References
const videoRef = useTemplateRef<HTMLVideoElement>('video')
const volumePopup = useTemplateRef<HTMLDivElement>('volumePopup')
const player = useTemplateRef<HTMLDivElement>('player')

const { playing, currentTime, duration, waiting } = useMediaControls(videoRef, {
  src: props.video?.link,
})
const { isFullscreen, enter, exit } = useFullscreen(player)

// Values
const isVolumePopupVisible = ref(false)
const showControls = ref(true)
const progress = computed(() => {
  return duration.value ? (currentTime.value / duration.value) * 100 : 0
})

onClickOutside(volumePopup, () => {
  isVolumePopupVisible.value = false
})

const onFullscreenEnter = async () => {
  await invoke('lock_orientation', { orientation: 'sensor_landscape' }).catch(
    (err) => {
      console.error('Failed to lock orientation:', err)
    }
  )
  await invoke('set_fullscreen', { hide: true }).catch((err) => {
    console.error('Failed to hide android layout:', err)
  })
  await enter()
}

const onFullscreenExit = async () => {
  await invoke('lock_orientation', { orientation: 'portrait' }).catch((err) => {
    console.error('Failed to lock orientation:', err)
  })
  await invoke('set_fullscreen', { hide: false }).catch((err) => {
    console.error('Failed to hide android layout:', err)
  })
  await exit()
}

const onPlay = () => {
  playing.value = true
}

onUnmounted(async () => {
  onFullscreenExit()
})
</script>

<template>
  <div id="player" ref="player" class="player text-text">
    <video
      ref="video"
      class="media"
      :src="`${video?.link}#t=0.001`"
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
      <PhCircleNotch class="animate-spin" :size="72" />
    </div>

    <Transition name="reveal" mode="out-in">
      <div
        class="bg-background/50 absolute inset-0"
        v-show="showControls"
        @click="showControls = false"
      >
        <div
          class="absolute flex top-3 font-title font-bold"
          :class="{ 'mx-5': isFullscreen, 'mx-2.5': !isFullscreen }"
        >
          {{ video?.title?.romaji }}
        </div>

        <!-- Bottom part of player -->
        <div
          class="controls absolute flex flex-col bottom-0 left-0 right-0 my-5 text-text"
          :class="{ 'mx-5': isFullscreen, 'mx-1': !isFullscreen }"
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
                  @click="
                    !isFullscreen ? onFullscreenEnter() : onFullscreenExit()
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
</style>

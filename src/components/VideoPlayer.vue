<script setup lang="ts">
import { secondsToTime } from '@/utils/math'
import {
  PhPlay,
  PhPause,
  PhArrowsOut,
  PhSpeakerSimpleHigh,
  PhSpeakerSimpleLow,
  PhSpeakerSimpleSlash,
} from '@phosphor-icons/vue'
import { invoke } from '@tauri-apps/api/core'
import { useTimeoutFn, useMediaControls, onClickOutside } from '@vueuse/core'
import { onMounted, useTemplateRef } from 'vue'
import { ref } from 'vue'

const props = defineProps<{ src: string; poster?: string }>()

const video = useTemplateRef<HTMLVideoElement>('video')
const { playing, volume, currentTime, duration } = useMediaControls(video, {
  src: props.src,
})
const { start, stop } = useTimeoutFn(() => {
  showControls.value = false
}, 3000)
const isVolumePopupVisible = ref(false)
const volumePopup = useTemplateRef<HTMLDivElement>('volumePopup')
const showControls = ref(true)

onClickOutside(volumePopup, () => {
  isVolumePopupVisible.value = false
})

const wakeControls = () => {
  showControls.value = true
  start()
}
</script>

<template>
  <div id="player" class="player text-text">
    <video
      ref="video"
      class="media"
      :src="src"
      :poster="poster"
      playsinline
      @click="wakeControls()"
    />

    <Transition name="reveal" mode="out-in">
      <div
        class="controls absolute flex flex-row bottom-0 left-0 right-0 rounded-sm m-1 text-text"
        @pointerdown="stop()"
        @pointerup="start()"
        v-show="showControls"
      >
        <div
          class="absolute inset-0 rounded-sm backdrop-blur-sm bg-background/20"
          aria-hidden="true"
        />

        <div class="relative flex flex-row items-center w-full">
          <button type="button" class="touchable" @click="playing = !playing">
            <PhPlay :size="24" v-if="!playing" />
            <PhPause :size="24" v-else />
          </button>

          <div class="relative">
            <button
              type="button"
              class="touchable"
              @click="isVolumePopupVisible = !isVolumePopupVisible"
            >
              <PhSpeakerSimpleHigh :size="24" v-if="volume > 0.5" />
              <PhSpeakerSimpleLow :size="24" v-else-if="volume > 0" />
              <PhSpeakerSimpleSlash :size="24" v-else />
            </button>

            <Transition name="reveal" mode="out-in">
              <div
                ref="volumePopup"
                v-show="isVolumePopupVisible"
                class="volume absolute bottom-full left-1/2 -translate-x-1/2 mb-2 flex items-center justify-center p-2 rounded-sm backdrop-blur-sm bg-background/20"
              >
                <input
                  type="range"
                  min="0"
                  max="1"
                  step="0.01"
                  v-model="volume"
                  class="w-full volume-slider volume-slider--vertical h-22 bg-surface/50 accent-primary"
                  :style="{ '--fill': `${volume * 100}%` }"
                />
              </div>
            </Transition>
          </div>

          <div
            class="bottom-full justify-center rounded-sm bg-background/20 flex flex-row grow self-center mx-2.5"
          >
            <input
              type="range"
              min="0"
              :max="duration"
              step="0.01"
              v-model="currentTime"
              class="video-slider w-full h-1 bg-transparent accent-primary"
              :style="{
                '--fill': `${(currentTime / duration) * 100}%`,
              }"
            />
          </div>

          <span class="text-text text-sm font-text self-center">
            {{ secondsToTime(currentTime) }} /
            {{ secondsToTime(duration) }}
          </span>

          <button type="button" class="touchable">
            <PhArrowsOut :size="24" />
          </button>
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

.player.is-fullscreen {
  position: fixed;
  inset: 0;
  z-index: 9999;
  width: 100%;
  height: 100vh;
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

.controls button {
  display: grid;
  place-items: center;
  width: 40px;
  height: 40px;
  border: 0;
  background: none;
  color: inherit;
  cursor: pointer;
}

.time {
  font-size: 13px;
  font-variant-numeric: tabular-nums;
  min-width: 36px;
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
  height: 96px;
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
  width: 100%;
  height: 100%;
  border-radius: 5px;
  background: linear-gradient(
    to right,
    var(--color-primary) var(--fill),
    rgb(255 255 255 / 0.3) var(--fill)
  );
}

.video-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 0px;
  height: 0px;
  border-radius: 5px;
  background: var(--color-primary);
}
</style>

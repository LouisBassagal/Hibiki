<script setup lang="ts">
import { useDialog } from '@/composables/useDialog'
import { usePlayerStore } from '@/stores/usePlayerState'
import { Media } from '@/types/anilist'
import { AnimeTheme, Video } from '@/types/animethemes'
import { VideoExtended } from '@/types/extended'
import { notify } from '@/utils/notify'
import { PhX } from '@phosphor-icons/vue'
import { toRef, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const props = defineProps<{
  media: Media | undefined
  theme: AnimeTheme | undefined | null
  isOpen: boolean
}>()

const emits = defineEmits<{
  'update:isOpen': [value: boolean]
}>()

const { t } = useI18n()
const { dialog, onClose } = useDialog(toRef(props, 'isOpen'), (value) =>
  emits('update:isOpen', value)
)

const selectedVideo = ref<Video | undefined>(undefined)
const selectedVideoElement = ref<HTMLElement | undefined>(undefined)
const isContextMenuOpen = ref(false)

const onContextMenuOpen = (reference: HTMLElement, video: Video) => {
  isContextMenuOpen.value = true
  selectedVideo.value = video
  selectedVideoElement.value = reference
}

const onBackdropClick = (event: MouseEvent) => {
  const rect = dialog.value!.getBoundingClientRect()
  const clickedInside =
    event.clientY >= rect.top &&
    event.clientY <= rect.bottom &&
    event.clientX >= rect.left &&
    event.clientX <= rect.right
  if (!clickedInside) dialog.value!.close()
}

const onAddToQueue = () => {
  if (!selectedVideo.value) return

  const extended: VideoExtended = {
    ...selectedVideo.value,
    title: props.theme?.song?.title ?? undefined,
    media: props.media,
  }
  usePlayerStore().enqueue(extended)
  notify.success(t('toast.addToQueue'))
  isContextMenuOpen.value = false
}

const onSelectTheme = (video: Video) => {
  if (!video) return
  const extended: VideoExtended = {
    ...video,
    title: props.theme?.song?.title ?? undefined,
    media: props.media,
  }
  usePlayerStore().start(extended)
  isContextMenuOpen.value = false
  onClose()
}
</script>

<template>
  <dialog
    ref="dialog"
    class="theme-dialog relative text-text flex flex-col font-semibold rounded-sm max-h-full bg-background backdrop:backdrop-blur-sm w-full max-w-full rounded-t-2xl border-none p-4 pb-safe-b"
    @close="onClose"
    @click="onBackdropClick"
  >
    <h1 class="font-title text-2xl mb-5">{{ $t('modal.title') }}</h1>
    <div
      class="flex flex-col gap-2.5 max-h-full overflow-y-scroll py-2.5 grow-0"
    >
      <template v-for="entry in theme?.animethemeentries">
        <EntryButton
          v-for="video in entry.videos.nodes"
          :entry="entry"
          :video="video"
          :title="theme?.song?.title ?? undefined"
          @click="onSelectTheme(video)"
          @menu="onContextMenuOpen($event, video)"
        />
      </template>
    </div>

    <button
      type="button"
      class="absolute right-4 top-4 touchable"
      @click="dialog?.close()"
    >
      <PhX :size="30" />
    </button>

    <ContextMenuContainer
      v-model:is-open="isContextMenuOpen"
      :reference="selectedVideoElement"
      :teleport="false"
    >
      <div class="flex flex-col gap-2.5 text-left">
        <button
          type="button"
          class="touchable w-full text-left"
          @click="onAddToQueue"
        >
          <span>{{ t('theme.menu.addToQueue') }}</span>
        </button>
        <button type="button" class="touchable w-full text-left">
          <span>{{ t('theme.menu.addToPlaylist') }}</span>
        </button>
      </div>
    </ContextMenuContainer>
  </dialog>
</template>

<style scoped>
dialog.theme-dialog {
  margin: 0;
  position: fixed;
  inset: auto 0 0 0;
  width: 100%;
  max-width: 100%;
  height: 66dvh;
  max-height: 66dvh;
  border: none;
  border-radius: 16px 16px 0 0;
  padding: 1rem;
  padding-bottom: calc(1rem + env(safe-area-inset-bottom, 0px));

  transform: translateY(100%);
  transition:
    transform 0.3s ease,
    overlay 0.3s allow-discrete,
    display 0.3s allow-discrete;
}

dialog.theme-dialog[open] {
  transform: translateY(0);
}

@starting-style {
  dialog.theme-dialog[open] {
    transform: translateY(100%);
  }
}

dialog.theme-dialog::backdrop {
  background: rgba(0, 0, 0, 0.4);
  transition:
    background-color 0.3s ease,
    overlay 0.3s allow-discrete,
    display 0.3s allow-discrete;
}

@starting-style {
  dialog.theme-dialog[open]::backdrop {
    background: rgba(0, 0, 0, 0);
  }
}
</style>

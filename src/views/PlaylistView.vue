<script setup lang="ts">
import db from '@/plugins/db'
import { usePlayerStore } from '@/stores/usePlayerState'
import { notify } from '@/utils/notify'
import {
  PhEmpty,
  PhPencil,
  PhPlaylist,
  PhUploadSimple,
  PhX,
} from '@phosphor-icons/vue'
import { from, useObservable } from '@vueuse/rxjs'
import { liveQuery } from 'dexie'
import { computed, onUnmounted, ref, useTemplateRef, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'

const isEditMode = ref(false)
const playlistName = ref()

const { t } = useI18n()
const router = useRouter()
const imageInputRef = useTemplateRef('imageInput')

const cover = computed(() => {
  URL.revokeObjectURL(cover.value)
  if (playlist.value?.cover) return URL.createObjectURL(playlist.value?.cover)
  return ''
})

const playlist = useObservable(
  from(
    liveQuery(() =>
      db.playlists
        .where('id')
        .equals(Number(router.currentRoute.value.params.id))
        .first()
    )
  )
)

const onImageChange = async (e: Event) => {
  const input = e.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  if (!playlist?.value && !playlist.value?.id) return

  await db.playlists
    .where('id')
    .equals(playlist.value?.id)
    .modify((p) => {
      p.cover = file
    })
    .then(() => {
      notify.success(t('toast.changePlaylistImage'))
    })
    .catch(() => {
      notify.error(t('toast.genericError'))
    })
  input.value = ''
  console.log(playlist)
}

const onNameChange = async () => {
  if (!playlist.value?.id) return
  await db.playlists
    .where('id')
    .equals(playlist.value?.id)
    .modify((p) => (p.name = playlistName.value))
    .then(() => {
      notify.success(t('toast.changePlaylistName'))
    })
    .catch(() => {
      notify.error(t('toast.genericError'))
    })
}

const onPlaylistThemeSelected = (index: number) => {
  if (!playlist.value?.items) return
  usePlayerStore().startPlaylist(playlist?.value.items, index)
}

const onRemoveThemeFromPlaylist = async (index: number) => {
  if (!playlist.value?.id) return
  await db.playlists
    .where('id')
    .equals(playlist.value?.id)
    .modify((p) => {
      p.items.splice(index, 1)
    })
    .then(() => {
      notify.success(t('toast.removeThemeFromPlaylist'))
    })
    .catch(() => {
      notify.error(t('toast.genericError'))
    })
}

watch(isEditMode, (newValue) => {
  if (newValue) playlistName.value = playlist.value?.name
})

onUnmounted(() => {
  URL.revokeObjectURL(cover.value)
})
</script>

<template>
  <div
    class="flex flex-col overflow-y-auto md:overflow-y-hidden grow md:flex md:flex-row"
  >
    <section
      class="flex flex-col mb-2.5 md:flex-col md:justify-center md:items-center md:px-10 md:w-1/3"
    >
      <div
        class="relative w-full bg-primary flex justify-center items-center aspect-square overflow-hidden md:rounded-sm"
      >
        <img
          v-if="playlist?.cover"
          class="absolute size-full object-cover"
          :src="cover"
        />
        <PhPlaylist :size="128" class="text-text" />
        <Transition name="reveal">
          <div
            v-if="isEditMode"
            class="absolute flex justify-center items-center inset-0 bg-background/50 backdrop-blur-sm"
          >
            <button
              type="button"
              class="touchable"
              @click="imageInputRef?.click()"
            >
              <PhUploadSimple :size="68" class="text-text" />
            </button>
            <input
              type="file"
              ref="imageInput"
              accept="image/*"
              multiple="false"
              class="sr-only"
              @change="onImageChange"
            />
          </div>
        </Transition>
      </div>
    </section>

    <section class="flex flex-col px-5 grow md:py-2.5">
      <div class="flex flex-row justify-between mb-2.5">
        <span
          v-if="!isEditMode"
          class="text-text font-bold font-title text-2xl"
        >
          {{ playlist?.name }}
        </span>

        <form v-else @submit.prevent="onNameChange">
          <input
            class="text-text font-bold font-title text-2xl ring-2 ring-terciary/50 rounded-sm"
            type="text"
            v-model="playlistName"
          />
        </form>

        <button type="button" @click="isEditMode = !isEditMode">
          <PhX v-if="isEditMode" :size="24" class="text-text touchable" />
          <PhPencil v-else :size="24" class="text-text touchable" />
        </button>
      </div>
      <div
        class="flex flex-col gap-2.5 grow text-text/75 justify-center items-center md:overflow-y-auto"
      >
        <div
          class="flex flex-col items-center"
          v-if="playlist?.items.length === 0"
        >
          <PhEmpty class="self-center" :size="48" />
          <span class="font-semibold"> {{ $t('playlist.emptyTitle') }} </span>
          <span class="text-text/50 text-sm">
            {{ $t('playlist.emptySubtitle') }}
          </span>
        </div>
        <div class="py-5 w-full flex justify-start h-full flex-col gap-2.5">
          <PlaylistThemeItem
            v-for="(video, index) in playlist?.items"
            :video="video"
            :is-edit-mode="isEditMode"
            @click="onPlaylistThemeSelected(index)"
            @delete="onRemoveThemeFromPlaylist(index)"
          />
        </div>
      </div>
    </section>
  </div>
</template>

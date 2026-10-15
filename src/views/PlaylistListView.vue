<script setup lang="ts">
import db from '@/plugins/db'
import { usePlayerStore } from '@/stores/usePlayerState'
import { Playlist } from '@/types/playlist'
import { notify } from '@/utils/notify'
import { PhPlay, PhPlus, PhTrash } from '@phosphor-icons/vue'
import { from, useObservable } from '@vueuse/rxjs'
import { liveQuery } from 'dexie'
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const userInput = ref('')
const playlistName = ref('')
const isNewPlaylistModalOpen = ref(false)

const isPlaylistContextMenuOpen = ref(false)
const selectedPlaylist = ref<Playlist | null>(null)
const selectedPlaylistReference = ref<HTMLElement | undefined>(undefined)
const { startPlaylist } = usePlayerStore()

const { t } = useI18n()

const playlists = useObservable(
  from(liveQuery(() => db.playlists.orderBy('name').toArray()))
)

const searchedPlaylists = computed(() => {
  if (userInput.value.trim() === '') return playlists.value
  return playlists.value?.filter((p) =>
    p.name.toLowerCase().includes(userInput.value.toLowerCase())
  )
})

const onOpenPlaylistModal = async () => {
  isNewPlaylistModalOpen.value = true
}

const onCreateNewPlaylist = async () => {
  isNewPlaylistModalOpen.value = false
  await db.playlists
    .add({ name: playlistName.value, items: [] })
    .then(() => {
      notify.success(t('toast.newPlaylist'))
    })
    .catch(() => {
      notify.error(t('toast.newPlaylistError'))
    })
    .finally(() => {
      playlistName.value = ''
    })
}

const onCancelPlaylist = () => {
  playlistName.value = ''
  isNewPlaylistModalOpen.value = false
}

const onDeletePlaylist = async () => {
  if (selectedPlaylist.value) {
    await db.playlists.delete(selectedPlaylist.value?.id)
    selectedPlaylist.value = null
    selectedPlaylistReference.value = undefined
    isPlaylistContextMenuOpen.value = false
  }
}

const onPlayPlaylist = () => {
  if (selectedPlaylist.value) {
    startPlaylist(selectedPlaylist.value?.items)
    selectedPlaylist.value = null
    selectedPlaylistReference.value = undefined
    isPlaylistContextMenuOpen.value = false
  }
}

const onPlaylistLongPress = (reference: HTMLElement, playlist: Playlist) => {
  selectedPlaylist.value = playlist
  selectedPlaylistReference.value = reference
  isPlaylistContextMenuOpen.value = true
}
</script>

<template>
  <div class="flex flex-col relative grow bg-background px-4 gap-5 text-text">
    <section id="search-bar" class="sticky left-0 top-0 z-10">
      <form @submit.prevent class="flex flex-row gap-2.5">
        <div
          class="flex flex-row justify-center items-center gap-2.5 backdrop-blur-sm rounded-sm bg-surface/50 w-full h-10 px-2.5 overflow-hidden text-text"
        >
          <PhMagnifyingGlass :size="24" />
          <input
            class="w-full"
            v-model="userInput"
            :placeholder="$t('playlists.placeholder')"
          />
        </div>

        <button
          type="button"
          class="shrink-0 touchable"
          @click="onOpenPlaylistModal"
        >
          <PhPlus :size="24" />
        </button>
      </form>
    </section>

    <section
      class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] place-items-center gap-y-5"
    >
      <TransitionGroup name="reveal">
        <PlaylistItem
          class="self-center"
          v-for="playlist in searchedPlaylists"
          :name="playlist.name"
          :key="playlist.id"
          @long-press="onPlaylistLongPress($event, playlist)"
          @click=""
        />
      </TransitionGroup>
    </section>

    <!-- Modal to get the name of the new playlist -->
    <ModalContainer
      class="text-text font-text"
      v-model:is-open="isNewPlaylistModalOpen"
      @close="onCancelPlaylist"
    >
      <div class="mb-2.5">
        <h2 class="text-text font-bold text-lg">
          {{ $t('playlists.modal.title') }}
        </h2>
        <span class="text-text/75 text-sm">{{
          $t('playlists.modal.subtitle')
        }}</span>
      </div>

      <div class="w-full flex flex-col gap-2 mb-5">
        <label for="playlist-name" class="font-semibold">{{
          $t('playlists.modal.label')
        }}</label>
        <input
          v-model="playlistName"
          id="playlist-name"
          autofocus
          class="ring-2 ring-primary rounded-sm w-full bg-surface h-10 px-2.5"
        />
      </div>

      <div class="flex flex-row justify-end gap-2.5">
        <button
          class="px-5 py-2.5 rounded-sm font-semibold touchable"
          @click="onCancelPlaylist"
        >
          {{ $t('playlists.modal.cancel') }}
        </button>

        <button
          class="px-5 py-2.5 bg-primary rounded-sm font-semibold touchable"
          @click="onCreateNewPlaylist"
        >
          {{ $t('playlists.modal.create') }}
        </button>
      </div>
    </ModalContainer>

    <ContextMenuContainer
      :reference="selectedPlaylistReference"
      v-model:is-open="isPlaylistContextMenuOpen"
    >
      <div class="flex flex-col h-full w-full gap-2.5">
        <button
          type="button"
          class="flex flex-row gap-2.5 items-center touchable"
          @click="onPlayPlaylist"
        >
          <PhPlay :size="24" />
          <span class="font-text font-semibold"
            >{{ $t('playlists.menu.play') }}
          </span>
        </button>

        <button
          type="button"
          class="flex flex-row gap-2.5 items-center touchable"
          @click="onDeletePlaylist"
        >
          <PhTrash :size="24" />
          <span class="font-text font-semibold"
            >{{ $t('playlists.menu.delete') }}
          </span>
        </button>
      </div>
    </ContextMenuContainer>
  </div>
</template>

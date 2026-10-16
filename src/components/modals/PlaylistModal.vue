<script setup lang="ts">
import db from '@/plugins/db'
import { VideoExtended } from '@/types/extended'
import { Playlist } from '@/types/playlist'
import { plain } from '@/utils/plain'
import { from, useObservable } from '@vueuse/rxjs'
import { liveQuery } from 'dexie'

const props = defineProps<{
  video: VideoExtended | undefined
}>()

const emits = defineEmits(['close'])

const isOpen = defineModel('isOpen')

const playlists = useObservable(
  from(liveQuery(() => db.playlists.orderBy('name').toArray()))
)

const onAddToPlaylist = async (playlist: Playlist) => {
  if (!props.video) return
  const pl = plain(props.video)

  await db.playlists
    .where('id')
    .equals(playlist.id)
    .modify((p) => {
      p.items.push(pl)
    })
}
</script>

<template>
  <ModalContainer v-model:is-open="isOpen" @close="emits('close')">
    <div class="mb-2.5 text-text">
      <h2 class="text-text font-bold text-lg">
        {{ $t('modal.playlist.title') }}
      </h2>
      <span class="text-text/75 text-sm">
        {{ $t('modal.playlist.subtitle', { name: video?.title?.romaji }) }}
      </span>
    </div>

    <div class="flex flex-col max-h-75 overflow-y-auto gap-2.5 p-1">
      <PlaylistModalItem
        v-for="playlist in playlists"
        :playlist="playlist"
        @click="onAddToPlaylist(playlist)"
      />
    </div>
  </ModalContainer>
</template>

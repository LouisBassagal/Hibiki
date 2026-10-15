<script setup lang="ts">
import db from '@/plugins/db'
import { PhPlus } from '@phosphor-icons/vue'
import { from, useObservable } from '@vueuse/rxjs'
import { liveQuery } from 'dexie'
import { computed, ref } from 'vue'

const userInput = ref('')
const playlists = useObservable(
  from(liveQuery(() => db.playlists.orderBy('name').toArray()))
)

const searchedPlaylists = computed(() => {
  if (userInput.value.trim() === '') return playlists.value
  return playlists.value?.filter((p) =>
    p.name.toLowerCase().includes(userInput.value.toLowerCase())
  )
})

const onCreateNewPlaylist = async () => {
  await db.playlists.add({ name: 'test', items: [] })
  console.log()
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
          @click="onCreateNewPlaylist"
        >
          <PhPlus :size="24" />
        </button>
      </form>
    </section>

    <section
      class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] place-items-center gap-y-5"
    >
      <TransitionGroup mode="in-out" name="reveal">
        <PlaylistItem
          class="self-center"
          v-for="playlist in searchedPlaylists"
          :name="playlist.name"
        />
      </TransitionGroup>
    </section>
  </div>
</template>

<script setup lang="ts">
import type { Media } from '@/types/anilist'
import { PhCircleNotch, PhMagnifyingGlass } from '@phosphor-icons/vue'
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

const medias = ref<Media[] | null>([])
const userInput = ref<string>('')
const isLoading = ref<boolean>(false)

const onUserSearch = async () => {
  isLoading.value = true
  medias.value = await invoke<Media[]>('fetch_anime_by_name', {
    search: userInput.value,
    page: 1,
    count: 50,
  })
  isLoading.value = false
}
</script>

<template>
  <div class="flex flex-col relative grow bg-background px-4 gap-5 text-text">
    <section id="search-bar" class="sticky left-0 top-0 z-10">
      <form @submit.prevent="onUserSearch">
        <div
          class="flex flex-row justify-center items-center gap-2.5 backdrop-blur-sm rounded-sm bg-surface/50 w-full h-10 px-2.5 overflow-hidden text-text"
        >
          <PhMagnifyingGlass :size="24" />
          <input
            class="w-full"
            v-model="userInput"
            :placeholder="$t('search.placeholder')"
          />
        </div>
      </form>
    </section>
    <section id="anime-list" class="text-text h-full">
      <Transition name="reveal" mode="in-out">
        <Transition mode="out-in">
          <div
            v-if="isLoading"
            key="loading"
            class="w-full h-full flex flex-col justify-center items-center"
          >
            <PhCircleNotch :size="50" class="animate-spin" />
          </div>

          <div v-else key="anime-list" class="grid grid-cols-3 gap-4">
            <AnimeCover
              size="sm"
              v-for="media in medias"
              :key="media.id"
              :media="media"
            />
          </div>
        </Transition>
      </Transition>
    </section>
  </div>
</template>

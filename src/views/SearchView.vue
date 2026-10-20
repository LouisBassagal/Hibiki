<script setup lang="ts">
import type { Media } from '@/types/anilist'
import { PhCircleNotch, PhEmpty, PhMagnifyingGlass } from '@phosphor-icons/vue'
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const medias = ref<Media[]>([])
const userInput = ref<string>('')
const isLoading = ref<boolean>(false)
const isSearchDone = ref<boolean>(false)
const router = useRouter()
const route = useRoute()

const onUserSearch = async () => {
  router.replace({ name: 'search', query: { search: userInput.value } })
  onSearch(userInput.value)
}

const onRecentSearch = async () => {
  const search = route.query['search']
  if (!search) return

  userInput.value = search as string
  onSearch(userInput.value)
}

const onSearch = async (name: string) => {
  isLoading.value = true
  medias.value = await invoke<Media[]>('fetch_anime_by_name', {
    search: name,
    page: 1,
    count: 50,
  }).finally(() => {
    isSearchDone.value = true
    isLoading.value = false
  })
}

onMounted(() => {
  onRecentSearch()
})
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

          <div
            v-else-if="medias?.length > 0 && isSearchDone"
            key="anime-list"
            class="grid grid-cols-[repeat(auto-fill,minmax(145px,1fr))] gap-4 place-items-center"
          >
            <AnimeCover
              v-for="media in medias"
              :key="media.id"
              :media="media"
            />
          </div>

          <div
            v-else-if="medias?.length === 0 && isSearchDone"
            class="flex flex-col h-full w-full justify-center items-center"
          >
            <PhEmpty :size="68" />
            <span class="font-semibold text-text">{{
              $t('search.noResultTitle')
            }}</span>
            <span class="text-text/75 text-sm">{{
              $t('search.noResultSubtitle')
            }}</span>
          </div>
        </Transition>
      </Transition>
    </section>
  </div>
</template>

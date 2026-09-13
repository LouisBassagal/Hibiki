<script setup lang="ts">
import type { Media } from '@/types/anilist'
import Search from '@primeicons/vue/search'
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue'

const currentSeasonAnimes = ref<Media[]>([])
const popularAnimes = ref<Media[]>([])
const bestRatedAnimes = ref<Media[]>([])

const errorMessage = ref('')

const fetchAnimes = async () => {
  const results = await Promise.allSettled([
    invoke<Media[]>('fetch_popular_animes', { page: 1, count: 10 }),
    invoke<Media[]>('fetch_current_season_animes', { page: 1, count: 10 }),
    invoke<Media[]>('fetch_best_rated_animes', { page: 1, count: 10 }),
  ])
}

onMounted(async () => {
  await fetchAnimes()
})
</script>

<template>
  <main
    class="flex flex-col min-h-screen bg-background px-4 py-6 gap-5 text-terciary"
  >
    <RandomAnimeHero />
    <section id="buttons" class="flex flex-col gap-2.5">
      <HugeButton>
        <template #icon>
          <Search :size="70" class="-scale-x-100" />
        </template>
        <template #default>
          {{ $t('home.buttons.Search') }}
        </template>
      </HugeButton>

      <HugeButton>
        <template #icon>
          <AnilistIcon class="size-17.5" />
        </template>
        <template #default>
          {{ $t('home.buttons.MyList') }}
        </template>
      </HugeButton>
    </section>

    <section id="lists" class="flex flex-col gap-5">
      <AnimeCoverList :title="$t('home.popular')" :medias="popularAnimes" />
      <AnimeCoverList
        :title="$t('home.season')"
        :medias="currentSeasonAnimes"
      />
      <AnimeCoverList :title="$t('home.best')" :medias="bestRatedAnimes" />
    </section>
  </main>
</template>

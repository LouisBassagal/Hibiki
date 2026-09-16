<script setup lang="ts">
import type { Media } from '@/types/anilist'
import { PhMagnifyingGlass } from '@phosphor-icons/vue'
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue'

const currentSeasonAnimes = ref<Media[] | null>([])
const popularAnimes = ref<Media[] | null>([])
const bestRatedAnimes = ref<Media[] | null>([])
const randomAnime = ref<Media | null>(null)

const errorMessage = ref('')

const fetchAnimes = async () => {
  await Promise.allSettled([
    invoke<Media[]>('fetch_popular_animes', { page: 1, count: 20 }),
    invoke<Media[]>('fetch_current_season_animes', { page: 1, count: 20 }),
    invoke<Media[]>('fetch_best_rated_animes', { page: 1, count: 20 }),
    invoke<Media>('fetch_random_known_anime', { page: 1, count: 20 }),
  ]).then((results) => {
    const [popular, currentSeason, bestRated, random] = results

    if (popular.status === 'fulfilled') {
      popularAnimes.value = popular.value
    } else {
      errorMessage.value = popular.reason
    }

    if (currentSeason.status === 'fulfilled') {
      currentSeasonAnimes.value = currentSeason.value
    } else {
      errorMessage.value = currentSeason.reason
    }

    if (bestRated.status === 'fulfilled') {
      bestRatedAnimes.value = bestRated.value
    } else {
      errorMessage.value = bestRated.reason
    }

    if (random.status === 'fulfilled') {
      randomAnime.value = random.value
    } else {
      errorMessage.value = random.reason
    }
  })
}

onMounted(async () => {
  await fetchAnimes()
})
</script>

<template>
  <div
    class="flex flex-col min-h-screen bg-background px-4 py-6 gap-5 text-terciary"
  >
    <RandomAnimeHero :media="randomAnime" />
    <section id="buttons" class="flex flex-col gap-2.5">
      <HugeButton>
        <template #icon>
          <PhMagnifyingGlass :size="70" class="-scale-x-100" />
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
  </div>
</template>

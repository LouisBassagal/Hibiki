<script setup lang="ts">
import type { Media } from '@/types/anilist'
import type { AnimeThemesAnime, AnimeTheme } from '@/types/animethemes'
import { Markdown } from '@comark/vue'
import { PhCircleNotch } from '@phosphor-icons/vue'
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref, Teleport } from 'vue'
import { useRouter } from 'vue-router'

const media = ref<Media | null>(null)
const currentTab = ref<'overview' | 'themes'>('overview')
const themes = ref<AnimeTheme[] | null>(null)
const selectedTheme = ref<AnimeTheme | null>(null)
const isModalOpen = ref<boolean>(false)
const router = useRouter()
const isAnimeLoading = ref<boolean>(true)
const isThemesLoading = ref<boolean>(true)

const isTabOpen = (name: 'overview' | 'themes') => {
  if (name !== currentTab.value) return ''
  return 'before:opacity-100'
}

const onSelectTheme = (theme: AnimeTheme) => {
  selectedTheme.value = theme
  isModalOpen.value = true
}

onMounted(async () => {
  const animeId = Number(router.currentRoute.value.params.id)

  if (!animeId || !Number.isInteger(animeId)) router.replace({ name: 'home' })

  await invoke<Media>('fetch_anime_by_id', {
    id: animeId,
  })
    .then((result) => {
      media.value = result
    })
    .finally(() => {
      isAnimeLoading.value = false
    })

  await invoke<AnimeThemesAnime>('fetch_themes_by_anilist_id', {
    anilistId: media?.value?.id,
  })
    .then((results) => {
      themes.value = results.animethemes
    })
    .finally(() => {
      isThemesLoading.value = false
    })
})
</script>

<template>
  <div v-if="!isAnimeLoading" class="flex flex-col flex-1">
    <section id="cover" class="text-text">
      <div class="h-75 relative object-center">
        <img
          :src="media?.bannerImage ?? ''"
          class="h-full w-full object-cover object-center -z-10"
        />
        <div
          class="absolute inset-0 w-full backdrop-blur-sm -z-10"
          style="
            mask-image: linear-gradient(to bottom, transparent 0%, black 100%);
            -webkit-mask-image: linear-gradient(
              to bottom,
              transparent 0%,
              black 100%
            );
          "
        />
        <div
          class="absolute inset-0 bg-linear-to-b from-transparent to-background"
        />
        <div
          class="absolute bottom-0 font-title px-5 font-bold text-2xl -mt-10 z-50"
        >
          {{ media?.title?.romaji }}
        </div>
      </div>
    </section>

    <section id="buttons" class="my-5 px-5 text-text">
      <div
        class="w-full flex flex-row font-text text-lg font-semibold justify-evenly"
      >
        <button
          type="button"
          class="relative touchable before:absolute before:h-0.5 before:mt-1 before:transition-all before:duration-500 before:w-full before:bg-primary before:top-full before:rounded-full before:opacity-0"
          :class="isTabOpen('overview')"
          @click="currentTab = 'overview'"
        >
          {{ $t('anime.overview') }}
        </button>
        <button
          type="button"
          class="relative touchable before:absolute before:h-0.5 before:mt-1 before:transition-all before:duration-500 before:w-full before:bg-primary before:top-full before:rounded-full before:opacity-0"
          :class="isTabOpen('themes')"
          @click="currentTab = 'themes'"
        >
          {{ $t('anime.themes') }}
        </button>
      </div>
    </section>

    <Transition name="reveal" mode="out-in">
      <section v-if="currentTab === 'overview'" id="overview" class="px-5 mb-5">
        <div class="text-text font-text font-semibold text-sm">
          {{ $t('anime.description') }}
          <div class="bg-surface p-5 mt-2.5 rounded-sm">
            <Suspense>
              <Markdown :value="media?.description ?? ''" />
              <template #fallback>
                <span>{{ media?.description ?? '' }}</span>
              </template>
            </Suspense>
          </div>
        </div>
      </section>

      <section
        v-else
        id="themes"
        class="px-5 mb-5 text-text flex flex-col grow"
      >
        <Transition name="reveal" mode="out-in">
          <div v-if="!isThemesLoading" class="flex flex-col gap-2.5">
            <div v-for="theme in themes">
              <AnimeThemeButton @click="onSelectTheme(theme)" :theme="theme" />
            </div>
          </div>

          <div
            v-else
            class="w-full h-full flex-1 grow flex flex-col justify-center items-center"
          >
            <PhCircleNotch :size="32" class="animate-spin text-text" />
          </div>
        </Transition>
      </section>
    </Transition>

    <Teleport to="body">
      <ThemeEntriesModal v-model:is-open="isModalOpen" :theme="selectedTheme">
      </ThemeEntriesModal>
    </Teleport>
  </div>
  <div
    v-else
    class="flex flex-col h-full w-full grow text-text justify-center items-center"
  >
    <PhCircleNotch :size="52" class="animate-spin" />
  </div>
</template>

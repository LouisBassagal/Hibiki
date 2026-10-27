<script setup lang="ts">
import { useAnilistUser } from '@/stores/useAnilistUser'
import type { Media } from '@/types/anilist'
import type { AnimeThemesAnime, AnimeTheme } from '@/types/animethemes'
import { notify } from '@/utils/notify'
import { Markdown } from '@comark/vue'
import {
  PhBookmarkSimple,
  PhCircleNotch,
  PhEmpty,
  PhStar,
} from '@phosphor-icons/vue'
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'

const media = ref<Media | undefined>(undefined)
const currentTab = ref<'overview' | 'themes'>('overview')
const themes = ref<AnimeTheme[]>([])
const selectedTheme = ref<AnimeTheme | null>(null)
const isModalOpen = ref<boolean>(false)
const isEntryModalOpen = ref<boolean>(false)
const router = useRouter()
const isAnimeLoading = ref<boolean>(true)
const isThemesLoading = ref<boolean>(true)
const { token, isLoggedIn } = useAnilistUser()
const { t } = useI18n()

const isTabOpen = (name: 'overview' | 'themes') => {
  if (name !== currentTab.value) return ''
  return 'before:opacity-100'
}

const onSelectTheme = (theme: AnimeTheme) => {
  selectedTheme.value = theme
  isModalOpen.value = true
}

const onSetFavourite = () => {
  invoke('toggle_favourite', { id: media?.value?.id, token: token })
    .then(() => {
      if (media.value) {
        media.value.isFavourite = !media?.value?.isFavourite
        notify.success(
          media?.value?.isFavourite === true
            ? t('toast.favouriteAnime')
            : t('toast.unfavouriteAnime')
        )
      }
    })
    .catch(() => {
      notify.error(
        media?.value?.isFavourite === true
          ? t('toast.unfavouriteAnimeError')
          : t('toast.favouriteAnimeError')
      )
    })
    .finally(() => {})
}

const onSave = async () => {
  const animeId = Number(router.currentRoute.value.params.id)

  await invoke<Media>('fetch_anime_by_id', {
    id: animeId,
    token: token,
  }).then((result) => {
    media.value = result
  })
}

onMounted(async () => {
  const animeId = Number(router.currentRoute.value.params.id)

  if (!animeId || !Number.isInteger(animeId)) router.replace({ name: 'home' })

  await invoke<Media>('fetch_anime_by_id', {
    id: animeId,
    token: token,
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
          class="absolute inset-0 bg-linear-to-b from-transparent to-background"
        />
        <div
          class="absolute inset-0 bg-linear-to-t from-transparent to-background"
        />
        <div
          class="absolute bottom-0 -mt-10 z-50 flex flex-col md:flex-row w-full px-5 md:justify-between md:items-center"
        >
          <div class="font-title font-bold text-2xl">
            {{ media?.title?.romaji }}
          </div>

          <div v-if="isLoggedIn" class="flex flex-row gap-2.5">
            <button
              @click="isEntryModalOpen = true"
              class="bg-primary px-2.5 py-1 rounded-sm h-fit w-fit shrink-0 touchable font-text font-semibold flex flex-row justify-center items-center self-center md:self-auto mt-2.5 md:mt-0"
            >
              <PhBookmarkSimple :size="20" class="mr-1 shrink-0" />
              {{
                media?.mediaListEntry?.status
                  ? $t(
                      'general.' +
                        media?.mediaListEntry?.status.toLocaleLowerCase()
                    )
                  : $t('general.AddToList')
              }}
            </button>

            <button
              @click="onSetFavourite"
              class="bg-primary p-1 rounded-sm h-fit w-fit shrink-0 touchable font-text font-semibold flex flex-row justify-center items-center self-center md:self-auto mt-2.5 md:mt-0"
            >
              <PhStar
                :weight="media?.isFavourite === true ? 'fill' : 'regular'"
                :size="24"
              />
            </button>
          </div>
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
          <div
            v-if="!isThemesLoading && themes?.length > 0"
            class="flex flex-col gap-2.5"
          >
            <div v-for="theme in themes">
              <AnimeThemeButton @click="onSelectTheme(theme)" :theme="theme" />
            </div>
          </div>

          <div
            v-else-if="isThemesLoading"
            class="w-full h-full flex-1 grow flex flex-col justify-center items-center"
          >
            <PhCircleNotch :size="32" class="animate-spin text-text" />
          </div>

          <div
            v-else-if="!isThemesLoading && themes?.length === 0"
            class="w-full h-full grow flex flex-col justify-center items-center text-text"
          >
            <PhEmpty :size="64" />
            <span class="text-text"> {{ $t('anime.noThemeTitle') }} </span>
            <span class="text-text/75 text-sm">
              {{ $t('anime.noThemeSubtitle') }}
            </span>
          </div>
        </Transition>
      </section>
    </Transition>

    <Teleport to="body">
      <EntryModal
        :media="media"
        @save="onSave"
        @close="isEntryModalOpen = false"
        v-model:is-open="isEntryModalOpen"
      >
      </EntryModal>
      <ThemeEntriesModal
        v-model:is-open="isModalOpen"
        :theme="selectedTheme"
        :media="media"
      >
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

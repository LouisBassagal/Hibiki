<script setup lang="ts">
import { useAnilistUser } from '@/stores/useAnilistUser'
import { Viewer } from '@/types/anilist'
import {
  PhCircleNotch,
  PhGlobe,
  PhSignIn,
  PhSignOut,
  PhUser,
} from '@phosphor-icons/vue'
import { invoke } from '@tauri-apps/api/core'
import { storeToRefs } from 'pinia'
import { ref, watch } from 'vue'

const { logout, login } = useAnilistUser()
const { token, isLoggedIn } = storeToRefs(useAnilistUser())
const profile = ref<Viewer | null>()
const isProfileLoading = ref(true)

const onLogout = () => {
  profile.value = null
  logout()
}

watch(
  isLoggedIn,
  async () => {
    isProfileLoading.value = true
    profile.value = await invoke<Viewer>('fetch_viewer', {
      token: token.value,
    }).finally(() => {
      isProfileLoading.value = false
    })
  },
  { immediate: true }
)
</script>

<template>
  <div class="w-full h-full">
    <Transition name="reveal" mode="out-in">
      <div v-if="!isProfileLoading" class="w-full h-full">
        <section id="banner" class="mb-20 text-teal-50">
          <div class="relative h-75 bg-red-500">
            <img
              v-if="profile?.bannerImage"
              class="absolute w-full h-full object-cover"
              :src="profile?.bannerImage"
            />

            <div v-else class="absolute w-full h-full bg-surface" />

            <div
              class="flex flex-row h-fit bottom-0 left-10 translate-y-1/3 absolute rounded-sm gap-5 items-end"
            >
              <div
                class="size-37.5 bg-primary rounded-sm overflow-hidden shrink-0"
              >
                <img
                  v-if="profile?.avatar?.large"
                  :src="profile?.avatar?.large ?? undefined"
                />

                <div
                  v-else
                  class="w-full h-full flex justify-center items-center text-text"
                >
                  <PhUser :size="64" />
                </div>
              </div>
              <div
                v-if="profile"
                class="flex text-text text-xl font-title font-bold"
              >
                {{ profile.name }}
              </div>
            </div>
          </div>
        </section>

        <section
          class="flex flex-col mx-10 grow text-text font-text font-semibold gap-2.5"
        >
          <div v-if="isLoggedIn" class="flex flex-col gap-2.5">
            <AnimeStatistics :statistics="profile?.statistics?.anime" />

            <AnimeCoverList
              :title="$t('profile.favoriteAnime')"
              v-if="profile?.favourites?.anime?.nodes != null"
              :medias="profile?.favourites?.anime?.nodes"
            />
          </div>

          <div v-else class="mb-5">
            <div class="text-3xl">
              {{ $t('profile.notLoggedTitle') }}
            </div>
            <div class="text-text/75">
              {{ $t('profile.notLoggedSubtitle') }}
            </div>
          </div>

          <div
            v-if="isLoggedIn"
            class="flex flex-col w-full md:flex-row gap-2.5"
          >
            <SimpleButton class="bg-surface" @click="onLogout">
              <div class="flex flex-row items-center justify-center gap-2.5">
                <PhSignOut :size="24" />
                {{ $t('profile.logout') }}
              </div>
            </SimpleButton>
            <SimpleButton class="bg-surface">
              <div class="flex flex-row items-center justify-center gap-2.5">
                <PhGlobe :size="24" />
                {{ $t('profile.seeAnilist') }}
              </div>
            </SimpleButton>
          </div>

          <div v-else>
            <SimpleButton class="bg-surface" @click="login">
              <div class="flex flex-row items-center justify-center gap-2.5">
                <PhSignIn :size="24" />
                {{ $t('profile.login') }}
              </div>
            </SimpleButton>
          </div>
        </section>
      </div>

      <div
        v-else
        class="flex w-full h-full text-text justify-center items-center"
      >
        <PhCircleNotch :size="64" class="animate-spin" />
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import {
  // PhGearSix,
  PhHouseLine,
  PhList,
  // PhListDashes,
  PhMagnifyingGlass,
  // PhMusicNote,
  // PhShuffle,
  // PhUser,
  PhX,
} from '@phosphor-icons/vue'
import { onClickOutside } from '@vueuse/core'
import { ref, useTemplateRef } from 'vue'
import { RouterLink } from 'vue-router'

const menuDiv = useTemplateRef('menu')
const isOpen = ref(false)

onClickOutside(menuDiv, () => (isOpen.value = false))
</script>

<template>
  <div
    ref="menu"
    class="bg-surface/50 backdrop-blur-sm bottom-10 right-7.5 rounded-sm fixed transition-all duration-300 overflow-hidden"
    :class="isOpen ? 'size-60' : 'size-12'"
  >
    <Transition name="menu-reveal" mode="out-in">
      <button
        v-if="!isOpen"
        @click="isOpen = true"
        type="button"
        class="size-full flex justify-center items-center touchable"
      >
        <PhList class="text-text size-10" :size="50" />
      </button>

      <div
        v-else
        class="size-full p-3.5 grid grid-cols-3 text-text text-sm font-text font-bold"
      >
        <RouterLink
          to="/"
          class="flex flex-col justify-center items-center touchable"
          active-class="text-primary"
        >
          <PhHouseLine :size="32" />
          <span>
            {{ $t('menu.home') }}
          </span>
        </RouterLink>

        <RouterLink
          to="/search"
          class="flex flex-col justify-center items-center touchable"
          active-class="text-primary"
        >
          <PhMagnifyingGlass :size="32" />
          <span>
            {{ $t('menu.search') }}
          </span>
        </RouterLink>

        <RouterLink
          to="/playlists"
          class="flex flex-col justify-center items-center touchable"
          active-class="text-primary"
        >
          <PhMusicNote :size="32" />
          <span>
            {{ $t('menu.playlists') }}
          </span>
        </RouterLink>

        <RouterLink
          to="/profile"
          class="flex flex-col justify-center items-center touchable"
          active-class="text-primary"
        >
          <PhUser :size="32" />
          <span>
            {{ $t('menu.account') }}
          </span>
        </RouterLink>

        <div />

        <div />

        <!-- <RouterLink
          to="/search"
          class="flex flex-col justify-center items-center touchable"
          active-class="text-primary"
        >
          <PhListDashes :size="32" />
          <span>
            {{ $t('menu.list') }}
          </span>
        </RouterLink> -->

        <div />

        <!-- <RouterLink
          to="/search"
          class="flex flex-col justify-center items-center touchable"
          active-class="text-primary"
        >
          <PhShuffle :size="32" />
          <span>
            {{ $t('menu.random') }}
          </span>
        </RouterLink> -->

        <div />
        <!-- <RouterLink
          to="/search"
          class="flex flex-col justify-center items-center touchable"
          active-class="text-primary"
        >
          <PhGearSix :size="32" />
          <span>
            {{ $t('menu.settings') }}
          </span>
        </RouterLink> -->

        <div />

        <div />
        <div />

        <button
          class="flex justify-center items-center touchable"
          @click="isOpen = false"
        >
          <PhX :size="32" weight="regular" class="size-9 shrink-0 text-text" />
        </button>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.menu-reveal-enter-active,
.menu-reveal-leave-active {
  transition: all 0.3s ease-in-out;
}

.menu-reveal-enter-from,
.menu-reveal-leave-to {
  opacity: 0;
  transform: scale(0.5);
}
</style>

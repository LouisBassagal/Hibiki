<script setup lang="ts">
import { Playlist } from '@/types/playlist'
import { PhPlusCircle } from '@phosphor-icons/vue'

const emits = defineEmits(['click'])

const props = defineProps<{
  playlist: Playlist
}>()
</script>

<template>
  <button
    class="bg-surface text-text shrink-0 flex flex-row justify-between px-2.5 py-2 rounded-sm touchable ring-2 ring-terciary/50"
    @click="emits('click')"
  >
    <span class="flex flex-col text-left">
      <span class="font-semibold">{{ playlist.name }}</span>
      <span class="text-text/75 text-sm">
        {{ $t('general.themes', playlist.items.length) }}
      </span>
    </span>

    <span class="self-center">
      <PhPlusCircle :size="24" />
    </span>
  </button>
</template>

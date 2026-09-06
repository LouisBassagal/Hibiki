<script setup lang="ts">
import AnimeCoverList from '@/components/AnimeCoverList.vue'
import { Media } from '@/types/anilist.ts'
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue'

const medias = ref<Media[]>([])

onMounted(async () => {
  medias.value =
    (await invoke<Media[]>('fetch_popular_animes', {
      page: 1,
    })) ?? []
})
</script>

<template>
  <AnimeCoverList direction="left" :medias="medias" />
  <pre class="text-amber-50">
    {{ medias }}
  </pre>
</template>

<script setup lang="ts">
import db from '@/plugins/db'
import { Playlist } from '@/types/playlist'
import { PhEmpty, PhPencil, PhUploadSimple, PhX } from '@phosphor-icons/vue'
import { from, useObservable } from '@vueuse/rxjs'
import { liveQuery } from 'dexie'
import { computed, onMounted, ref, useTemplateRef } from 'vue'
import { useRouter } from 'vue-router'

const isEditMode = ref(false)

const router = useRouter()
const imageInputRef = useTemplateRef('imageInput')

const cover = computed(() => {
  if (playlist.value?.cover) return URL.createObjectURL(playlist.value?.cover)
  return ''
})

const onImageModified = async (e: Event) => {
  const input = e.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  if (!playlist?.value && !playlist.value?.id) return

  await db.playlists
    .where('id')
    .equals(playlist.value?.id)
    .modify((p) => {
      p.cover = file
    })
  input.value = ''
  console.log(playlist)
}

const playlist = useObservable(
  from(
    liveQuery(() =>
      db.playlists
        .where('id')
        .equals(Number(router.currentRoute.value.params.id))
        .first()
    )
  )
)
</script>

<template>
  <div class="flex flex-col overflow-y-auto grow">
    <section class="flex flex-col mb-2.5">
      <div class="relative w-full bg-gray-600/50 aspect-square overflow-hidden">
        <img v-if="playlist?.cover" class="absolute size-full" :src="cover" />
        <Transition name="reveal">
          <div
            v-if="isEditMode"
            class="absolute flex justify-center items-center inset-0 bg-background/50 backdrop-blur-sm"
          >
            <button
              type="button"
              class="touchable"
              @click="imageInputRef?.click()"
            >
              <PhUploadSimple :size="68" class="text-text" />
            </button>
            <input
              type="file"
              ref="imageInput"
              accept="image/*"
              multiple="false"
              class="sr-only"
              @change="onImageModified"
            />
          </div>
        </Transition>
      </div>
    </section>

    <section class="flex flex-col px-5 grow">
      <div class="flex flex-row justify-between">
        <span class="text-text font-bold font-title text-2xl p">
          {{ playlist?.name }}
        </span>

        <button type="button" @click="isEditMode = !isEditMode">
          <PhX v-if="isEditMode" :size="24" class="text-text touchable" />
          <PhPencil v-else :size="24" class="text-text touchable" />
        </button>
      </div>
      <div
        class="flex flex-col gap-2.5 grow text-text/75 justify-center items-center"
      >
        <div
          class="flex flex-col items-center"
          v-if="playlist?.items.length === 0"
        >
          <PhEmpty class="self-center" :size="48" />
          <span class="font-semibold"> Cette playlist est vide </span>
          <span class="text-text/50 text-sm">
            Peut-etre faudrait-il la remplir ?
          </span>
        </div>
        <PlaylistThemeItem v-for="video in playlist?.items" :video="video" />
      </div>
    </section>
  </div>
</template>

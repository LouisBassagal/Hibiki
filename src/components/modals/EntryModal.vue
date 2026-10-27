<script setup lang="ts">
import { useAnilistUser } from '@/stores/useAnilistUser'
import { Media, MediaListEntry } from '@/types/anilist'
import { notify } from '@/utils/notify'
import { invoke } from '@tauri-apps/api/core'
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const STATUSES = [
  'CURRENT',
  'PLANNING',
  'COMPLETED',
  'PAUSED',
  'DROPPED',
  'REPEATING',
] as const

type EntryDraft = Pick<
  MediaListEntry,
  'status' | 'score' | 'progress' | 'startedAt' | 'completedAt' | 'repeat'
>

const props = defineProps<{
  media: Media | undefined | null
}>()

const emits = defineEmits(['close', 'save'])

const isOpen = defineModel('isOpen')

const { token } = useAnilistUser()

const { t } = useI18n()

const toDraft = (e?: MediaListEntry | null): EntryDraft => ({
  status: e?.status ?? 'PLANNING',
  score: e?.score ?? 0,
  progress: e?.progress ?? 0,
  startedAt: { ...e?.startedAt },
  completedAt: { ...e?.completedAt },
  repeat: e?.repeat ?? 0,
})

const onSaveEntry = async () => {
  await invoke('update_entry', {
    id: props.media?.id,
    entry: draft.value,
    token: token,
  })
    .then(() => {
      notify.success(t('toast.updateEntry'))
      emits('save')
      isOpen.value = false
    })
    .catch(() => {
      notify.error(t('toast.updateEntryError'))
    })
}

const statusOptions = computed(() =>
  STATUSES.map((s) => ({ value: s, label: t(`general.${s.toLowerCase()}`) }))
)
const entry = computed(() => props.media?.mediaListEntry)

const draft = ref<EntryDraft>(toDraft(entry.value))

watch(isOpen, (newValue) => {
  if (newValue) draft.value = toDraft(entry.value)
})
</script>

<template>
  <ModalContainer v-model:is-open="isOpen" @close="emits('close')">
    <div class="flex flex-col">
      <div class="flex flex-row gap-2.5 mb-5">
        <img
          :src="media?.coverImage?.large ?? media?.coverImage?.medium ?? ''"
          class="aspect-a4 rounded-sm h-32"
        />
        <span class="text-text font-title font-bold text-xl self-end pb-5">
          {{ media?.title?.romaji }}
        </span>
      </div>

      <form class="grid md:grid-cols-3 grid-cols-2 gap-x-5 gap-y-5 w-full mb-5">
        <SelectInput
          :label="$t('modal.entry.status')"
          v-model="draft.status"
          :options="statusOptions"
        />
        <NumberInput
          v-model:input="draft.score"
          :label="$t('modal.entry.score')"
        />
        <NumberInput
          v-model:input="draft.progress"
          :label="$t('modal.entry.progress')"
        />

        <DateInput
          v-model:input="draft.startedAt"
          :label="$t('modal.entry.startDate')"
        />
        <DateInput
          v-model:input="draft.completedAt"
          :label="$t('modal.entry.finishDate')"
        />
        <NumberInput
          v-model:input="draft.repeat"
          :label="$t('modal.entry.rewatch')"
        />
      </form>
    </div>

    <div class="flex flex-row justify-end text-text font-text font-semibold">
      <button
        class="px-5 py-2.5 rounded-sm font-semibold touchable"
        @click="emits('close')"
      >
        {{ $t('general.cancel') }}
      </button>

      <button
        class="px-5 py-2.5 bg-primary rounded-sm font-semibold touchable"
        @click="onSaveEntry"
      >
        {{ $t('general.save') }}
      </button>
    </div>
  </ModalContainer>
</template>

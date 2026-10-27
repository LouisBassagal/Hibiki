<script setup lang="ts">
import { FuzzyDate } from '@/types/anilist'
import { PhCalendarDots } from '@phosphor-icons/vue'
import { ref, useTemplateRef, watch } from 'vue'

defineProps<{
  label?: string | undefined
}>()

const input = defineModel<FuzzyDate>('input')
const rawInput = ref('')
const inputRef = useTemplateRef('input')

watch(rawInput, (newValue) => {
  const [year, month, day] = newValue.split('-')
  input.value = {
    year: Number(year),
    month: Number(month),
    day: Number(day),
  }
})
</script>

<template>
  <div
    class="text-text h-fit w-full flex flex-col font-text font-semibold gap-1"
  >
    <label for="" v-if="label">{{ label }}</label>

    <div
      class="bg-surface ring-1 ring-terciary/50 rounded-sm py-2.5 flex flex-row items-center px-2.5"
    >
      <button
        type="button"
        class="w-full flex flex-row items-center justify-between"
        @click="inputRef?.showPicker()"
      >
        <span v-if="input?.day && input?.month && input?.year">
          {{
            $t('general.date', {
              day: input.day,
              month: input.month,
              year: input.year,
            })
          }}
        </span>

        <span v-else> --- </span>

        <PhCalendarDots />
      </button>
      <input
        ref="input"
        name=""
        type="date"
        class="w-full sr-only"
        v-model="rawInput"
      />
    </div>
  </div>
</template>

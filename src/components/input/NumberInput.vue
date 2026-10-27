<script setup lang="ts">
import { PhCaretDown, PhCaretUp } from '@phosphor-icons/vue'

withDefaults(
  defineProps<{
    label?: string | undefined
    min?: number
    max?: number
  }>(),
  {
    label: undefined,
    min: undefined,
    max: undefined,
  }
)

const input = defineModel<number>('input')
</script>

<template>
  <div
    class="relative text-text h-fit w-full flex flex-col font-text font-semibold gap-1"
  >
    <label for="" v-if="label">{{ label }}</label>

    <div
      class="bg-surface ring-1 ring-terciary/50 rounded-sm py-2.5 flex flex-row items-center pl-2.5"
    >
      <input
        name=""
        type="number"
        class="w-full"
        :min="min"
        :max="max"
        v-model="input"
      />

      <div class="flex flex-col px-2.5 absolute right-1">
        <button
          type="button"
          class="w-full flex justify-center"
          @click="input != undefined ? (input += 1) : null"
        >
          <PhCaretUp />
        </button>
        <button
          type="button"
          class="w-full flex justify-center"
          @click="input != undefined ? (input -= 1) : null"
        >
          <PhCaretDown />
        </button>
      </div>
    </div>
  </div>
</template>

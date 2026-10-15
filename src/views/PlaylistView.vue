<script setup lang="ts">
import { ref } from 'vue'
const userInput = ref('')
</script>

<template>
  <div class="flex flex-col relative grow bg-background px-4 gap-5 text-text">
    <section id="search-bar" class="sticky left-0 top-0 z-10">
      <form @submit.prevent>
        <div
          class="flex flex-row justify-center items-center gap-2.5 backdrop-blur-sm rounded-sm bg-surface/50 w-full h-10 px-2.5 overflow-hidden text-text"
        >
          <PhMagnifyingGlass :size="24" />
          <input
            class="w-full"
            v-model="userInput"
            :placeholder="$t('search.placeholder')"
          />
        </div>
      </form>
    </section>
  </div>
</template>

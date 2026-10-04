<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()
</script>

<template>
  <main
    class="relative w-full h-full grow overflow-x-hidden flex flex-col bg-background pt-safe-t pb-safe-b"
  >
    <Transition name="reveal" mode="out-in">
      <slot />
    </Transition>
    <FloatingMenu v-if="route.name !== 'player'" />
  </main>
</template>

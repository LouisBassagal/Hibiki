<script setup lang="ts">
import Search from '@primeicons/vue/search'
</script>

<template>
  <main
    class="flex flex-col min-h-screen bg-background px-4 py-6 gap-5 text-terciary"
  >
    <RandomAnimeHero />
    <section id="buttons" class="flex flex-col gap-2.5">
      <HugeButton>
        <template #icon>
          <Search :size="70" class="-scale-x-100" />
        </template>
        <template #default>
          {{ $t('home.buttons.Search') }}
        </template>
      </HugeButton>

      <HugeButton>
        <template #icon>
          <AnilistIcon class="size-17.5" />
        </template>
        <template #default>
          {{ $t('home.buttons.MyList') }}
        </template>
      </HugeButton>
    </section>
  </main>
</template>

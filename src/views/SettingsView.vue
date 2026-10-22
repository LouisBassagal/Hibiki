<script setup lang="ts">
import { useSettings } from '@/composables/useSettings'
import { PhCheck } from '@phosphor-icons/vue'
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const { locale, availableLocales, getLocaleMessage } = useI18n()

const languages = computed(() => {
  return availableLocales.map((code) => {
    return {
      code,
      name: getLocaleMessage(code).meta?.name ?? code,
    }
  })
})

const currentLanguage = computed(() => {
  return (
    languages.value.find((l) => {
      return l.code == locale.value
    })?.name ?? locale.value
  )
})

const isLanguageMenuOpen = ref(false)
</script>

<template>
  <section class="flex flex-col grow px-10 text-text font-text font-semibold">
    <h1 class="font-bold text-2xl mb-2.5">{{ $t('settings.settings') }}</h1>

    <div class="flex shrink-0 grow w-full">
      <div class="md:grid md:grid-cols-2 flex flex-col w-full gap-2.5">
        <div class="w-full">
          <span class="text-text/75">{{ $t('settings.general') }}</span>
          <div
            class="flex flex-row bg-surface rounded-sm w-full px-5 py-2.5 mt-2.5 justify-between items-center"
          >
            <div>
              <div>{{ $t('settings.languageTitle') }}</div>
              <div class="text-text/75 text-sm">
                {{ $t('settings.languageSubtitle') }}
              </div>
            </div>

            <CustomSelect
              v-model:name="currentLanguage"
              v-model:is-open="isLanguageMenuOpen"
            >
              <div class="flex flex-col">
                <button
                  v-for="language in languages"
                  :key="language.code"
                  type="button"
                  role="menuitemradio"
                  :aria-checked="locale === language.code"
                  class="px-1.5 py-1 flex flex-row text-left justify-between items-center gap-2.5"
                  @click="
                    () => {
                      locale = language.code
                      isLanguageMenuOpen = false
                      useSettings().saveSetting('language', language.code)
                    }
                  "
                >
                  {{ language.name }}
                  <PhCheck v-if="locale === language.code" />
                </button>
              </div>
            </CustomSelect>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

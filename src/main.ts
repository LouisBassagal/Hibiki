import PhosphorIcons from '@phosphor-icons/vue'
import { createPinia } from 'pinia'
import { createApp } from 'vue'
import { createI18n } from 'vue-i18n'

import en from '~/i18n/locales/en.json'
import fr from '~/i18n/locales/fr.json'

import App from './App.vue'
import { useSettings } from './composables/useSettings.ts'
import { router } from './plugins/router.ts'

const pinia = createPinia()
const app = createApp(App)
const { loadSetting } = useSettings()

const savedLanguage = await loadSetting('language')

const i18n = createI18n({
  legacy: false,
  locale: 'en',
  fallbackLocale: 'en',
  messages: {
    en,
    fr,
  },
})

app.use(pinia)
app.use(PhosphorIcons)
app.use(router)
app.use(i18n)

if (
  savedLanguage &&
  i18n.global.availableLocales.includes(
    savedLanguage as (typeof i18n.global.availableLocales)[number],
  )
)
  i18n.global.locale.value = savedLanguage as (typeof i18n.global.availableLocales)[number]

app.mount('#app')

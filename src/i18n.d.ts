import en from '../i18n/locales/en.json'

type EnglishLocale = typeof en

declare module 'vue-i18n' {
  export interface DefineLocaleMessage extends EnglishLocale {}
}
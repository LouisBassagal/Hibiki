import { clearToken, loadToken, saveToken } from '@/utils/token'
import { getCurrent, onOpenUrl } from '@tauri-apps/plugin-deep-link'
import { openUrl } from '@tauri-apps/plugin-opener'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

export const useAnilistUser = defineStore('user', () => {
  const token = ref<string | undefined>(undefined)
  const isListening = ref(false)
  const isLoggedIn = computed(() => !!token.value)

  const handleRedirect = async (urls: string[] | null) => {
    if (!urls) return

    const params = new URLSearchParams(new URL(urls[0]).hash.slice(1))
    const t = params.get('access_token')

    if (!t) return
    token.value = t
    await saveToken(t)
  }

  const login = async () => {
    await openUrl(
      'https://anilist.co/api/v2/oauth/authorize?client_id=54579&response_type=token'
    )
  }

  const init = async () => {
    token.value = await loadToken()

    console.log('HIIHII')
    console.log(token.value)
    console.log(isLoggedIn.value)
    if (!isListening.value) {
      isListening.value = true
      await onOpenUrl(handleRedirect)
    }
    await handleRedirect(await getCurrent())
  }

  const logout = async () => {
    token.value = undefined
    await clearToken()
  }

  return { login, logout, token, isLoggedIn, init }
})

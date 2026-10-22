import db from '@/plugins/db'

const KEY = 'anilist_key'

export const saveToken = async (token: string) => {
  await db.settings.put({ key: KEY, value: token })
}

export const loadToken = async () => (await db.settings.get(KEY))?.value

export const clearToken = async () => await db.settings.delete(KEY)

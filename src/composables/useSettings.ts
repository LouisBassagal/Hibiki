import db from '@/plugins/db'

export const useSettings = () => {
  const saveSetting = async (key: string, value: string) =>
    await db.settings.put({ key, value })

  const loadSetting = async (key: string) => (await db.settings.get(key))?.value

  const clearSetting = async (key: string) => await db.settings.delete(key)

  const getAllSettings = async () => await db.settings.toArray()

  return { saveSetting, loadSetting, clearSetting, getAllSettings }
}

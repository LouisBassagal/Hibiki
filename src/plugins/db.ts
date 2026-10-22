import { Playlist } from '@/types/playlist'
import { Settings } from '@/types/settings'
import { Dexie, type EntityTable } from 'dexie'

const db = new Dexie('hibiki') as Dexie & {
  playlists: EntityTable<Playlist, 'id'>
  settings: EntityTable<Settings, 'key'>
}

db.version(1).stores({
  playlists: '++id, name, items, cover',
})

db.version(2).stores({
  playlists: '++id, name, items, cover',
  settings: 'key',
})

export default db

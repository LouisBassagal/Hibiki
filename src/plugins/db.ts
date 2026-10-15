import { Playlist } from '@/types/playlist'
import { Dexie, type EntityTable } from 'dexie'

const db = new Dexie('hibiki') as Dexie & {
  playlists: EntityTable<Playlist, 'id'>
}

db.version(1).stores({
  playlists: '++id, name, items',
})

export default db

import AnimeView from '@/views/AnimeView.vue'
import HomeView from '@/views/HomeView.vue'
import PlaylistListView from '@/views/PlaylistListView.vue'
import PlaylistView from '@/views/PlaylistView.vue'
import SearchView from '@/views/SearchView.vue'
import { createWebHistory, createRouter } from 'vue-router'

const routes = [
  {
    path: '/',
    component: HomeView,
    name: 'home',
  },
  {
    path: '/search',
    component: SearchView,
    name: 'search',
  },
  {
    path: '/anime/:id',
    component: AnimeView,
    name: 'anime',
  },
  {
    path: '/playlists/',
    component: PlaylistListView,
    name: 'playlists',
  },
  {
    path: '/playlists/:id',
    component: PlaylistView,
    name: 'playlist',
  },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

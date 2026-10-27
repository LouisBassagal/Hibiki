import AnimeView from '@/views/AnimeView.vue'
import HomeView from '@/views/HomeView.vue'
import PlaylistListView from '@/views/PlaylistListView.vue'
import PlaylistView from '@/views/PlaylistView.vue'
import ProfileView from '@/views/ProfileView.vue'
import SearchView from '@/views/SearchView.vue'
import SettingsView from '@/views/SettingsView.vue'
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
  {
    path: '/profile',
    component: ProfileView,
    name: 'profile',
  },
  {
    path: '/settings',
    component: SettingsView,
    name: 'settings',
  },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

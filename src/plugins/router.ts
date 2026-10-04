import AnimeView from '@/views/AnimeView.vue'
import HomeView from '@/views/HomeView.vue'
import PlayerView from '@/views/PlayerView.vue'
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
  { path: '/player', component: PlayerView, name: 'player' },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

import AnimeView from '@/views/AnimeView.vue'
import HomeView from '@/views/HomeView.vue'
import SearchView from '@/views/SearchView.vue'
import { createWebHistory, createRouter } from 'vue-router'

const routes = [
  { path: '/', component: HomeView, name: 'home' },
  { path: '/search', component: SearchView, name: 'search' },
  { path: '/anime', component: AnimeView, name: 'anime' },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

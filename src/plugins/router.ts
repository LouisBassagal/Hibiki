import HomeView from '@/views/HomeView.vue'
import SearchView from '@/views/SearchView.vue'
import { createWebHistory, createRouter } from 'vue-router'

const routes = [
  { path: '/', component: HomeView, name: 'home' },
  { path: '/search', component: SearchView, name: 'search' },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

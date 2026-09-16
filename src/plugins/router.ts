import HomeView from '@/views/HomeView.vue'
import SearchView from '@/views/SearchView.vue'
import { createWebHistory, createRouter } from 'vue-router'

const routes = [
  { path: '/', component: HomeView },
  { path: '/search', component: SearchView },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

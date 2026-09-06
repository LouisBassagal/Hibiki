import HomeView from '@/views/HomeView.vue'
import { createWebHistory, createRouter } from 'vue-router'

const routes = [{ path: '/', component: HomeView }]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

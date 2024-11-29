import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import LoginView from '@/views/LoginView.vue'
import SignupView from '@/views/SignupView.vue'
import { routes } from './routes'
import UserChatsView from '@/views/UserChatsView.vue'
import { AuthGuard } from './guards/auth.guard'
import ChatRoomView from '@/views/ChatRoomView.vue'
import chatAccessGuard from './guards/chat-access.guard'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: routes.HOME,
      name: 'home',
      component: HomeView,
    },
    {
      path: routes.SIGNUP,
      name: 'signup',
      component: SignupView,
    },
    {
      path: routes.LOGIN,
      name: 'login',
      component: LoginView,
    },
    {
      path: routes.CHATS,
      name: 'chats',
      component: UserChatsView,
      beforeEnter: AuthGuard,
    },
    {
      path: `${routes.CHATS}/:chatId`,
      name: 'chat-room',
      component: ChatRoomView,
      beforeEnter: [AuthGuard, chatAccessGuard],
      props: true,
    },

    // {
    //   path: '/about',
    //   name: 'about',
    //   // route level code-splitting
    //   // this generates a separate chunk (About.[hash].js) for this route
    //   // which is lazy-loaded when the route is visited.
    //   component: () => import('../views/AboutView.vue'),
    // },
  ],
})

export default router

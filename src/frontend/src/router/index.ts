import { createRouter, createWebHistory } from 'vue-router';
import HomeView from '../views/HomeView.vue';
import LoginView from '../views/LoginView.vue';
import { useAuthStore } from '@/stores/auth';
import { type UserData } from '@/models';
import EventDetails from '@/components/EventDetails.vue';
import NoEventDetails from '@/components/NoEventDetails.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
      props: { showPast: false },
      children: [
        {
          path: '',
          component: NoEventDetails,
        },
        {
          path: ':id',
          component: EventDetails,
          props: true,
        }
      ]
    },
    {
      path: '/login',
      name: 'login',
      component: LoginView
    },
    {
      path: '/history',
      name: 'history',
      component: HomeView,
      props: { showPast: true }
    }
  ]
});

router.beforeEach(async (to, _from, next) => {
  const authStore = useAuthStore(); // Access the auth store

  try {
    await fetch('/api/v1/auth/')
      .then(async (response) => {
        if (response.status != 200) {
          throw Error('Bad Return Code');
        }
        const jsonData: UserData = await response.json();
        return jsonData;
      })
      .then((jsonData) => {
        authStore.setUser(jsonData);
        next();
      });
  } catch (error) {
    console.error('Error fetching data:', error);
    if (to.matched.some((record) => record.path == '/login')) {
      next();
    } else {
      next('/login');
    }
  }
});

export default router;

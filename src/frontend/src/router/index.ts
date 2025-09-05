import {
  createRouter,
  createWebHistory,
  type RouteLocationNormalizedGeneric,
  type RouteLocationNormalizedLoadedGeneric
} from 'vue-router';
import HomeView from '../views/HomeView.vue';
import LoginView from '../views/LoginView.vue';
import { useAuthStore } from '@/stores/auth';
import { type UserData } from '@/models';
import EventDetails from '@/components/EventDetails.vue';
import NoEventDetails from '@/components/NoEventDetails.vue';

import { type Event } from '@/models';

async function handleEventIdRedirects(
  to: RouteLocationNormalizedGeneric,
  _from: RouteLocationNormalizedLoadedGeneric
) {
  const response = await fetch(`/api/v1/event/${to.params.id}`);

  if (response.status == 404) {
    return {
      path: '/'
    };
  }

  if (response.status != 200) {
    throw Error('Bad Return Code');
  }

  const jsonData: Event = await response.json();

  const isInPast = new Date(jsonData.startTime).getTime() < Date.now();

  const isGoingToPast = to.path.includes('history');

  if ((isGoingToPast && isInPast) || (!isGoingToPast && !isInPast)) {
    return true;
  }

  if (isInPast) {
    return { path: `/history/${to.params.id}` };
  } else {
    return { path: `/${to.params.id}` };
  }
}

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: HomeView,
      props: { showPast: false },
      children: [
        {
          path: '',
          name: 'home',
          component: NoEventDetails
        },
        {
          path: ':id',
          component: EventDetails,
          beforeEnter: handleEventIdRedirects,
          props: (route) => ({ id: Number(route.params.id) })
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
      component: HomeView,
      props: { showPast: true },
      children: [
        {
          path: '',
          name: 'history',
          component: NoEventDetails
        },
        {
          path: ':id',
          component: EventDetails,
          beforeEnter: handleEventIdRedirects,
          props: (route) => ({ id: Number(route.params.id) })
        }
      ]
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

<script lang="ts" setup>
import BellAlert from './icons/BellAlert.vue';
import BellSlash from './icons/BellSlash.vue';
</script>

<template>
  <button type="button" class="btn btn-danger mb-2" @click="sendDataRemove" v-if="userInNeedsRide">
    <BellSlash style="vertical-align: text-top" /> No Longer Need Ride ({{
      eventStore.selectedEvent?.needsRide.length
    }})
  </button>
  <button
    type="button"
    class="btn btn-primary mb-2"
    @click="sendDataAdd"
    :disabled="userInCar"
    v-else
  >
    <BellAlert style="vertical-align: text-top" /> Need Ride ({{
      eventStore.selectedEvent?.needsRide.length
    }})
  </button>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useEventStore } from '@/stores/events';
import { useAuthStore } from '@/stores/auth';
import { PopupType } from '@/models';
import { usePopupStore } from '@/stores/popup';

export default defineComponent({
  data() {
    return {
      authStore: useAuthStore(),
      eventStore: useEventStore()
    };
  },
  computed: {
    userInCar() {
      let allCars = this.eventStore.selectedEvent?.cars;
      let userId = this.authStore.user?.id;
      return allCars!.some(
        (car) => car.riders.some((rider) => rider.id === userId) || car.driver.id === userId
      );
    },
    userInNeedsRide() {
      return this.eventStore.selectedEvent?.needsRide.some(
        (user) => user.id === this.authStore.user?.id
      );
    }
  },
  methods: {
    async sendDataAdd() {
      const popupStore = usePopupStore();
      const eventStore = useEventStore();

      try {
        const response = await fetch(`/api/v1/event/${eventStore.selectedEvent?.id}/needride`, {
          method: 'POST'
        });

        if (response.ok) {
          popupStore.addPopup(PopupType.Success, 'Added to Need Ride List!');
          this.eventStore.selectedEvent?.needsRide.push({
            id: this.authStore.user!.id,
            realm: this.authStore.user!.type,
            name: this.authStore.user!.given_name + ' ' + this.authStore.user!.family_name,
            email: this.authStore.user!.email!
          });
        } else {
          popupStore.addPopup(
            PopupType.Danger,
            `Failed to Add to Need Ride List (${response.status})`
          );
        }
      } catch (error) {
        console.error(error);
        popupStore.addPopup(
          PopupType.Danger,
          'Failed to Add to Need Ride List. An unknown error occured.'
        );
      }
    },
    async sendDataRemove() {
      const popupStore = usePopupStore();
      const eventStore = useEventStore();

      try {
        const response = await fetch(`/api/v1/event/${eventStore.selectedEvent?.id}/needride`, {
          method: 'DELETE'
        });

        if (response.ok) {
          popupStore.addPopup(PopupType.Success, 'Removed from Need Ride List!');
          this.eventStore.selectedEvent?.needsRide.splice(
            this.eventStore.selectedEvent?.needsRide.findIndex(
              (user) => user.id === this.authStore.user?.id
            ),
            1
          );
        } else {
          popupStore.addPopup(
            PopupType.Danger,
            `Failed to Removed from Need Ride List (${response.status})`
          );
        }
      } catch (error) {
        console.error(error);
        popupStore.addPopup(
          PopupType.Danger,
          'Failed to Removed from Need Ride List. An unknown error occured.'
        );
      }
    }
  }
});
</script>

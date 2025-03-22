<template>
  <button type="button" class="btn btn-primary mt-3" @click="joinCar">Join Car</button>
</template>

<script lang="ts">
import { PopupType } from '@/models';
import { useAuthStore } from '@/stores/auth';
import { useEventStore } from '@/stores/events';
import { usePopupStore } from '@/stores/popup';
import { defineComponent } from 'vue';

export default defineComponent({
  props: {
    carId: Number
  },
  methods: {
    async joinCar() {
      const popupStore = usePopupStore();
      try {
        const authStore = useAuthStore();
        const eventStore = useEventStore();

        const response = await fetch(
          `/api/v1/event/${eventStore.selectedEvent?.id}/car/${this.carId}/rider/`,
          {
            method: 'POST'
          }
        );

        if (!response.ok) {
          popupStore.addPopup(PopupType.Danger, `Failed to Join Car (${response.status})`);
          return;
        }
        eventStore.selectedEvent?.cars
          ?.filter((car) => car.id === this.carId)
          .pop()
          ?.riders.push({
            id: authStore.user!.id,
            realm: authStore.user!.type,
            name: authStore.user!.given_name + ' ' + authStore.user!.family_name,
            email: authStore.user!.email!
          });

        let needRideIndex = eventStore.selectedEvent?.needsRide.findIndex((user) => user.id === authStore.user?.id);
        if (needRideIndex !== undefined && needRideIndex !== -1) {
          eventStore.selectedEvent?.needsRide.splice(needRideIndex, 1);
        }
        popupStore.addPopup(PopupType.Success, 'You have been added!');
      } catch (error) {
        console.error(error);
        popupStore.addPopup(PopupType.Danger, 'Failed to Join Car. An unknown error occured.');
      }
    }
  }
});
</script>

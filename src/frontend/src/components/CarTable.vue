<script lang="ts" setup>
import CarRowGroup from './CarRowGroup.vue';
import LeaveCarModal from './LeaveCarModal.vue';
import Loading from './LoadingWheel.vue';

const eventStore = useEventStore();
</script>

<template>
  <Loading v-if="loading" />
  <div v-else>
    <table class="table">
      <thead>
        <tr>
          <th scope="col">Driver</th>
          <th scope="col">Capacity</th>
          <th scope="col">Departure</th>
          <th scope="col">Return</th>
          <th scope="col"></th>
        </tr>
      </thead>
      <tbody>
        <CarRowGroup
          v-for="car in eventStore.selectedEventCars"
          :car="car"
          :eventId="eventId"
          :key="car.id"
        />
      </tbody>
    </table>
    <LeaveCarModal v-for="car in eventStore.selectedEventCars" :carId="car!.id" :key="car!.id" />
  </div>
</template>

<script lang="ts">
import { PopupType, type Car } from '@/models';
import { defineComponent } from 'vue';
import { useEventStore } from '@/stores/events';
import { usePopupStore } from '@/stores/popup';

export default defineComponent({
  props: {
    eventId: Number
  },
  data() {
    return {
      loading: true
    };
  },
  methods: {
    async fetchCarData() {
      try {
        const response = await fetch(`/api/v1/event/${this.eventId}/car/`);
        if (!response.ok) {
          const popupStore = usePopupStore();
          popupStore.addPopup(PopupType.Danger, `Failed to Get Cars (${response.status})`);
          return;
        }
        const data: Car[] = await response.json();
        const eventStore = useEventStore();
        if (eventStore.selectedEvent) {
          eventStore.selectedEvent.cars = data;
        }
        this.loading = false;
      } catch (error) {
        console.error(error);
        const popupStore = usePopupStore();
        popupStore.addPopup(PopupType.Danger, 'Failed to Get Cars. An unknown error occured.');
      }
    }
  },
  created() {
    this.fetchCarData(); // Fetch card data when the component is created
  }
});
</script>

<style>
.table {
  white-space: normal;
}
.collapse-enter-active,
.collapse-leave-active {
  transition: all 0.35s ease;
}

.collapse-enter-from,
.collapse-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.collapse-enter-active *,
.collapse-leave-active * {
  transition: all 0.35s ease;
}

.collapse-enter-from *,
.collapse-leave-to * {
  opacity: 0;
  transform: translateY(-30px);
}
</style>

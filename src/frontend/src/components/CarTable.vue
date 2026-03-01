<script lang="ts" setup>
import CarRowGroup from './CarRowGroup.vue';
import LeaveCarModal from './LeaveCarModal.vue';
import Loading from './LoadingWheel.vue';
import CaretUp from './icons/CaretUp.vue';
import CaretDown from './icons/CaretDown.vue';
import BlankIcon from './icons/BlankIcon.vue';
</script>

<template>
  <Loading v-if="loading" />
  <div v-else>
    <table class="table">
      <thead>
        <tr>
          <th scope="col" @click="changeSort('driver')">
            Driver <CaretUp v-if="sortingOrder.fieldName == 'driver' && sortingOrder.asc" />
            <CaretDown v-if="sortingOrder.fieldName == 'driver' && !sortingOrder.asc" />
            <BlankIcon v-if="sortingOrder.fieldName != 'driver'" />
          </th>
          <th scope="col" @click="changeSort('capacity')">
            Capacity <CaretUp v-if="sortingOrder.fieldName == 'capacity' && sortingOrder.asc" />
            <CaretDown v-if="sortingOrder.fieldName == 'capacity' && !sortingOrder.asc" />
            <BlankIcon v-if="sortingOrder.fieldName != 'capacity'" />
          </th>
          <th scope="col" @click="changeSort('departure')">
            Departure <CaretUp v-if="sortingOrder.fieldName == 'departure' && sortingOrder.asc" />
            <CaretDown v-if="sortingOrder.fieldName == 'departure' && !sortingOrder.asc" />
            <BlankIcon v-if="sortingOrder.fieldName != 'departure'" />
          </th>
          <th scope="col" @click="changeSort('return')">
            Return <CaretUp v-if="sortingOrder.fieldName == 'return' && sortingOrder.asc" />
            <CaretDown v-if="sortingOrder.fieldName == 'return' && !sortingOrder.asc" />
            <BlankIcon v-if="sortingOrder.fieldName != 'return'" />
          </th>
          <th scope="col"></th>
        </tr>
      </thead>
      <tbody>
        <CarRowGroup v-for="car in sortedCars" :car="car" :eventId="eventId" :key="car.id" />
      </tbody>
    </table>
    <LeaveCarModal v-for="car in sortedCars" :carId="car!.id" :key="car!.id" />
  </div>
</template>

<script lang="ts">
import { PopupType, type Car } from '@/models';
import { defineComponent } from 'vue';
import { useEventStore } from '@/stores/events';
import { usePopupStore } from '@/stores/popup';

interface SortingOrder {
  fieldName: string;
  asc: boolean;
}

export default defineComponent({
  props: {
    eventId: Number
  },
  data() {
    return {
      loading: true,
      sortingOrder: {
        fieldName: 'driver',
        asc: true
      } as SortingOrder
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
    },
    changeSort(field: string) {
      if (this.sortingOrder.fieldName === field) {
        this.sortingOrder.asc = !this.sortingOrder.asc;
      } else {
        this.sortingOrder.fieldName = field;
        this.sortingOrder.asc = true;
      }
    }
  },
  created() {
    this.fetchCarData(); // Fetch card data when the component is created
  },
  computed: {
    sortedCars() {
      const eventStore = useEventStore();
      const { fieldName, asc } = this.sortingOrder;
      return [...(eventStore.selectedEventCars || [])].sort((a, b) => {
        const compare = (valA: string | number, valB: string | number) => {
          if (valA < valB) return asc ? -1 : 1;
          if (valA > valB) return asc ? 1 : -1;
          return 0;
        };

        switch (fieldName) {
          case 'driver':
            return compare(a.driver.name, b.driver.name);
          case 'capacity':
            return compare(a.riders.length, b.riders.length);
          case 'departure':
            return compare(
              new Date(a.departureTime).getTime(),
              new Date(b.departureTime).getTime()
            );
          case 'return':
            return compare(new Date(a.returnTime).getTime(), new Date(b.returnTime).getTime());
          default:
            return 0;
        }
      });
    }
  }
});
</script>

<style>
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

th {
  cursor: pointer;
}
</style>

import { defineStore } from 'pinia';
import { type Car, type Event } from '@/models';

function sortByStartDate(a: Event, b: Event) {
  return new Date(a.startTime).getTime() - new Date(b.startTime).getTime();
}

export const useEventStore = defineStore('events', {
  state: () => ({
    events: [] as Event[],
    selectedEvent: null as Event | null
  }),
  getters: {
    selectedEventCars: (state) => state.selectedEvent?.cars
  },
  actions: {
    addEvent(event: Event) {
      this.events.push(event);
    },
    setEvents(events: Event[]) {
      this.events = events;
    },
    sortEvents(past: Boolean) {
      this.events.sort(sortByStartDate);

      if (past) {
          this.events.reverse();
      }
    },
    removeEvent(event: Event | null) {
      if (event == null) {
        return;
      }
      const index = this.events.indexOf(event);
      if (index > -1) {
        this.events.splice(index, 1);
      }
    },
    selectEvent(event: Event) {
      if (this.selectedEvent == event) {
        return;
      }
      this.selectedEvent = event;
      this.selectedEvent.cars = [];
    },
    addCar(car: Car) {
      this.selectedEvent?.cars?.push(car);
    },
    removeCar(car: Car) {
      const index = this.selectedEvent?.cars?.indexOf(car);
      if (index != null && index > -1) {
        this.selectedEvent?.cars?.splice(index, 1);
      }
    }
  }
});

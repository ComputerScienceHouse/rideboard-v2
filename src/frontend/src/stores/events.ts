import { defineStore } from 'pinia';
import { type Car, type Event } from '@/models';

function sortByStartDate(a: Event, b: Event) {
  return new Date(a.startTime).getTime() - new Date(b.startTime).getTime();
}

export const useEventStore = defineStore('events', {
  state: () => ({
    events: [] as Event[],
    id: null as number | null
  }),
  getters: {
    selectedEvent(state) {
      return state.events.find((event) => {
        return event.id == state.id;
      });
    },
    selectedEventCars(): Car[] | undefined {
      return this.selectedEvent?.cars;
    }
  },
  actions: {
    addEvent(event: Event) {
      this.events.push(event);
    },
    setEvents(events: Event[]) {
      this.events = events;
    },
    setEventId(id: number) {
      this.id = id;
    },
    sortEvents(past: Boolean) {
      this.events.sort(sortByStartDate);

      if (past) {
        this.events.reverse();
      }
    },
    removeEvent(id: number | null) {
      if (id == null) {
        return;
      }
      const index = this.events.findIndex((event) => event.id == id);
      if (index > -1) {
        this.events.splice(index, 1);
      }
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

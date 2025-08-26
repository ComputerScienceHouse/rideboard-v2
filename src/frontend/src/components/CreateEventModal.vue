<script setup lang="ts">
import LocationSearch from './LocationSearch.vue';
</script>

<template>
  <div
    class="modal fade"
    id="createEventModal"
    tabindex="-1"
    role="dialog"
    aria-labelledby="createEventModalLabel"
    aria-hidden="true"
  >
    <div class="modal-dialog" role="document">
      <div class="modal-content">
        <div class="modal-header">
          <h4 class="modal-title" id="createEventModalLabel">Create Event</h4>
          <button type="button" class="close" data-bs-dismiss="modal" aria-label="Close">
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label for="CreateEventTitle">Event Title</label>
            <input
              v-model="eventTitle"
              class="form-control"
              id="CreateEventTitle"
              placeholder="My Super Cool Event"
            />
          </div>
          <div class="form-group">
            <label for="CreateEventLocation">Location</label>
            <LocationSearch v-model="eventLocation" />
          </div>
          <div class="form-group">
            <label for="CreateEventStart">Start Time</label>
            <input
              v-model="eventStart"
              type="datetime-local"
              class="form-control"
              id="CreateEventStart"
            />
          </div>
          <div class="form-group">
            <label for="CreateEventEnd">End Time</label>
            <input
              v-model="eventEnd"
              :min="eventStart"
              type="datetime-local"
              class="form-control"
              id="CreateEventEnd"
            />
          </div>
        </div>
        <div class="modal-footer">
          <button
            type="button"
            id="createEventClose"
            class="btn btn-secondary"
            data-bs-dismiss="modal"
          >
            Close
          </button>
          <button type="button" class="btn btn-primary" @click="createEvent">Create</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useEventStore } from '@/stores/events';
import { useAuthStore } from '@/stores/auth';
import { usePopupStore } from '@/stores/popup';
import { PopupType } from '@/models';
import { validateEvent } from '@/validators';

export default defineComponent({
  data() {
    return {
      eventTitle: '',
      eventLocation: '',
      eventStart: '',
      eventEnd: ''
    };
  },
  methods: {
    async createEvent() {
      const popupStore = usePopupStore();
      try {
        let validate = validateEvent(
          this.eventTitle,
          this.eventLocation,
          this.eventStart,
          this.eventEnd
        );
        if (validate.length != 0) {
          validate.forEach((issue) => popupStore.addPopup(PopupType.Danger, issue));
          return;
        }
        const data = {
          name: this.eventTitle,
          location: this.eventLocation,
          startTime: new Date(this.eventStart).toISOString(),
          endTime: new Date(this.eventEnd).toISOString()
        };
        const response = await fetch('/api/v1/event/', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(data)
        });

        if (!response.ok) {
          popupStore.addPopup(PopupType.Danger, `Failed to Create Event (${response.status})`);
          return;
        }
        const result = await response.json();
        const eventStore = useEventStore();
        const authStore = useAuthStore();
        const newEvent = {
          id: result,
          name: data.name,
          location: data.location,
          startTime: new Date(data.startTime),
          endTime: new Date(data.endTime),
          creator: {
            id: authStore.user!.id,
            realm: authStore.user!.type,
            name: authStore.user!.given_name + ' ' + authStore.user!.family_name,
            email: authStore.user!.email!
          },
          needsRide: []
        };
        eventStore.addEvent(newEvent);
        eventStore.sortEvents(false);
        eventStore.selectEvent(newEvent);

        popupStore.addPopup(PopupType.Success, 'Event Created!');
        this.closeModal();
      } catch (error) {
        console.error(error);
        popupStore.addPopup(PopupType.Danger, 'Failed to Create Event. An unknown error occured.');
      }
    },
    closeModal() {
      const closeButton = document.getElementById('createEventClose');
      closeButton?.click();
    }
  }
});
</script>

<style scoped>
svg {
  height: 1.5em;
}
</style>

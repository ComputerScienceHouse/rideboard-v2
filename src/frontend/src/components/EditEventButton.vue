<script setup lang="ts">
import DeleteEventButton from './DeleteEventButton.vue';
import DeleteEventModal from './DeleteEventModal.vue';
</script>

<template>
  <button
    type="button"
    class="d-flex justify-content-center align-items-center btn btn-warning w-100"
    data-bs-toggle="modal"
    data-bs-target="#editEventModal"
  >
    Edit Event
  </button>
  <div
    class="modal fade"
    id="editEventModal"
    tabindex="-1"
    role="dialog"
    aria-labelledby="editEventModalLabel"
    aria-hidden="true"
  >
    <div class="modal-dialog" role="document">
      <div class="modal-content">
        <div class="modal-header">
          <h4 class="modal-title" id="editEventModalLabel">Edit Event</h4>
          <button type="button" class="close" data-bs-dismiss="modal" aria-label="Close">
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label for="editEventTitle">Event Title</label>
            <input
              v-model="eventTitle"
              class="form-control"
              id="editEventTitle"
              placeholder="My Super Cool Event"
            />
          </div>
          <div class="form-group">
            <label for="editEventLocation">Location</label>
            <input
              v-model="eventLocation"
              class="form-control"
              id="editEventLocation"
              placeholder="DSP 3"
            />
          </div>
          <div class="form-group">
            <label for="editEventStart">Start Time</label>
            <input
              v-model="eventStart"
              type="datetime-local"
              class="form-control"
              id="editEventStart"
            />
          </div>
          <div class="form-group">
            <label for="editEventEnd">End Time</label>
            <input
              v-model="eventEnd"
              :min="eventStart"
              type="datetime-local"
              class="form-control"
              id="editEventEnd"
            />
          </div>
        </div>
        <DeleteEventButton />
        <div class="modal-footer">
          <button
            type="button"
            id="editEventClose"
            class="btn btn-secondary"
            data-bs-dismiss="modal"
          >
            Close
          </button>
          <button type="button" class="btn btn-primary" @click="editEvent">Edit</button>
        </div>
      </div>
    </div>
  </div>
  <DeleteEventModal />
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useEventStore } from '@/stores/events';
import { format } from 'date-fns';
import { usePopupStore } from '@/stores/popup';
import { PopupType } from '@/models';
import { validateEvent } from '@/validators';

export default defineComponent({
  data() {
    const eventStore = useEventStore();
    return {
      eventTitle: eventStore.selectedEvent!.name,
      eventLocation: eventStore.selectedEvent!.location,
      eventStart: format(
        new Date(eventStore.selectedEvent!.startTime.toLocaleString()),
        "yyyy-MM-dd'T'HH:mm:ss"
      ),
      eventEnd: format(
        new Date(eventStore.selectedEvent!.endTime.toLocaleString()),
        "yyyy-MM-dd'T'HH:mm:ss"
      )
    };
  },
  methods: {
    async editEvent() {
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
        const eventStore = useEventStore();
        const response = await fetch(`/api/v1/event/${eventStore.selectedEvent!.id}`, {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(data)
        });

        if (!response.ok) {
          popupStore.addPopup(PopupType.Danger, `Failed to Edit Event (${response.status})`);
          return;
        }
        eventStore.selectedEvent!.name = data.name;
        eventStore.selectedEvent!.location = data.location;
        eventStore.selectedEvent!.startTime = new Date(data.startTime);
        eventStore.selectedEvent!.endTime = new Date(data.endTime);
        popupStore.addPopup(PopupType.Success, 'Event Updated!');
        this.closeModal();
      } catch (error) {
        console.error(error);
        popupStore.addPopup(PopupType.Danger, 'Failed to Edit Event. An unknown error occured.');
      }
    },
    closeModal() {
      const closeButton = document.getElementById('editEventClose');
      closeButton?.click();
    }
  }
});
</script>

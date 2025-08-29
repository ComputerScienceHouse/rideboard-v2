<template>
  <div
    class="modal fade"
    id="deleteEventModal"
    tabindex="-1"
    role="dialog"
    aria-labelledby="deleteEventModalLabel"
    aria-hidden="true"
  >
    <div class="modal-dialog" role="document">
      <div class="modal-content">
        <div class="modal-header">
          <h4 class="modal-title" id="deleteEventModalLabel">Delete Event</h4>
          <button type="button" class="close" data-bs-dismiss="modal" aria-label="Close">
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <h5>Are you sure you want to delete this event?</h5>
        </div>
        <div class="modal-footer">
          <button
            type="button"
            id="deleteEventClose"
            class="btn btn-secondary"
            data-bs-dismiss="modal"
          >
            Cancel
          </button>
          <button type="button" class="btn btn-primary" @click="removeEvent">Delete</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useEventStore } from '@/stores/events';
import { usePopupStore } from '@/stores/popup';
import { PopupType } from '@/models';

export default defineComponent({
  methods: {
    async removeEvent() {
      const popupStore = usePopupStore();
      try {
        const eventStore = useEventStore();
        const response = await fetch(`/api/v1/event/${eventStore.id}`, {
          method: 'DELETE'
        });

        if (!response.ok) {
          popupStore.addPopup(PopupType.Danger, `Failed to Delete Event (${response.status})`);
          return;
        }
        eventStore.removeEvent(eventStore.id);
        eventStore.id = -1;
        popupStore.addPopup(PopupType.Success, 'Event Deleted!');
        this.closeModal();
      } catch (error) {
        console.error(error);
        popupStore.addPopup(PopupType.Danger, 'Failed to Delete Event. An unknown error occured.');
      }
    },
    closeModal() {
      const closeButton = document.getElementById('deleteEventClose');
      closeButton?.click();
    }
  }
});
</script>

<template>
  <button type="button" class="btn btn-primary" @click="copyLink">Copy Link to Event</button>
</template>

<script lang="ts">
import { PopupType } from '@/models';
import { usePopupStore } from '@/stores/popup';
import { defineComponent } from 'vue';

export default defineComponent({
  props: {
    eventId: Number
  },
  methods: {
    copyLink() {
      const popupStore = usePopupStore();

      const urlToCopy = window.location.href;

      try {
        navigator.clipboard.writeText(urlToCopy);

        popupStore.addPopup(PopupType.Success, 'Copied url for event to clipboard!');
      } catch (_err) {
        popupStore.addPopup(PopupType.Danger, `Failed to copy url ${urlToCopy} to clipboard`);
      }
    }
  }
});
</script>

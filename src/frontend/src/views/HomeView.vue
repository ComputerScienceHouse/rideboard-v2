<script lang="ts" setup>
import EventCard from '@/components/EventCard.vue';
import EventDetails from '@/components/EventDetails.vue';
import CreateEventButton from '@/components/CreateEventButton.vue';
import { useEventStore } from '@/stores/events';
import Loading from '@/components/LoadingWheel.vue';

const eventStore = useEventStore();
</script>

<template>
  <div class="container">
    <Loading v-if="loading" />
    <div v-else>
      <button
        v-if="screenStore.mobile"
        class="btn btn-primary mb-2"
        type="button"
        @click="returnHome()"
      >
        All Events
      </button>
      <div class="row">
        <!-- Left column: List of cards -->
        <Transition @after-leave="showDetail = true" name="mobile">
          <div v-if="!screenStore.mobile || showList" class="noOverflow col-md-4 pb-1">
            <EventCard
              v-for="(event, index) in eventStore.events"
              :event="event"
              :key="index"
              :is-in-past="showPast"
            />
            <CreateEventButton v-if="!showPast" />
          </div>
        </Transition>
        <!-- Right column: Display selected card details -->
        <Transition @after-leave="showList = true" name="mobile">
          <div class="noOverflow col-md-8 pb-1" v-if="!screenStore.mobile || showDetail">
            <!-- 
              <EventDetails
              v-if="$props.id"
              :event="eventStore.getEvent($props.id)"
              :key="$props.id"
            />

            <div v-else>
              <p>Select an Event to see details</p>
            </div>
            -->
            <RouterView />
          </div>
        </Transition>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { PopupType, type Event } from '@/models';
import { defineComponent } from 'vue';
import { usePopupStore } from '@/stores/popup';
import { useScreenStore } from '@/stores/screen';

export default defineComponent({
  props: {
    showPast: Boolean,
    id: Number,
  },
  data() {
    let screenStore = useScreenStore();
    return {
      showList: true,
      showDetail: false,
      screenStore,
      loading: true
    };
  },
  methods: {
    async fetchCardData() {
      const popupStore = usePopupStore();
      try {
        const response = await fetch(
          '/api/v1/event/?' +
            new URLSearchParams({
              past: this.showPast.toString()
            }).toString()
        );
        if (!response.ok) {
          popupStore.addPopup(PopupType.Danger, `Failed to Get Events (${response.status})`);
          return;
        }
        const data = await response.json();
        const eventStore = useEventStore();
        eventStore.setEvents(data);
        eventStore.sortEvents(this.showPast);
        this.loading = false;
      } catch (error) {
        console.error(error);
        popupStore.addPopup(PopupType.Danger, 'Failed to Get Events. An unknown error occured.');
      }
    },
    selectEvent(event: Event) {
      const eventStore = useEventStore();
      // eventStore.selectEvent(event);
      if (this.screenStore.width < 768) {
        this.showList = false;
      }
    },
    returnHome() {
      this.showDetail = false;
      if (this.screenStore.width < 768) {
        const eventStore = useEventStore();
        // eventStore.selectedEvent = null;
      }
    }
  },
  created() {
    this.fetchCardData(); // Fetch card data when the component is created
  },
  provide() {
    return {
      historyMode: this.showPast
    };
  }
});
</script>

<style>
.cardlist {
  height: 90vh;
  max-height: 90vh;
  overflow: scroll;
}

.noOverflow > * {
  overflow: auto;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.mobile-enter-active,
.mobile-leave-active {
  transition: all 0.35s ease;
}

.mobile-enter-from,
.mobile-leave-to {
  opacity: 0;
  width: 0;
}

.col-md-4 .col-md-8 {
  flex: none !important;
}

svg {
  width: 1.5em;
}
</style>

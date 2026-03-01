<script lang="ts" setup>
import EventCard from '@/components/EventCard.vue';
import CreateEventButton from '@/components/CreateEventButton.vue';
import { useEventStore } from '@/stores/events';
import Loading from '@/components/LoadingWheel.vue';

const eventStore = useEventStore();
</script>

<template>
  <div class="container">
    <Loading v-if="loadingEvents" />
    <div v-else>
      <RouterLink
        v-if="screenStore.mobile"
        class="btn btn-primary mb-2"
        type="button"
        :to="showPast ? '/history' : '/'"
      >
        All Events
      </RouterLink>
      <div class="row">
        <!-- Left column: List of cards -->
        <Transition @after-leave="showDetail = true" name="mobile" appear>
          <div v-if="!screenStore.mobile || showList" class="noOverflow col-md-4 pb-1">
            <EventCard
              v-for="(event, index) in eventStore.events"
              :event="event"
              :key="index"
              @click="selectEvent()"
            />
            <CreateEventButton v-if="!showPast" />
          </div>
        </Transition>
        <!-- Right column: Display selected card details -->
        <Transition @after-leave="showList = true" name="mobile" appear>
          <div class="noOverflow col-md-8 pb-1" v-if="!screenStore.mobile || showDetail">
            <RouterView />
          </div>
        </Transition>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useScreenStore } from '@/stores/screen';

export default defineComponent({
  props: {
    showPast: Boolean
  },
  data() {
    let screenStore = useScreenStore();
    return {
      showList: true,
      showDetail: false,
      screenStore,
      loadingEvents: false,
      firstLoad: true
    };
  },
  methods: {
    selectEvent() {
      if (this.screenStore.width < 768) {
        this.showList = false;
      }
    },
    returnHome() {
      this.showDetail = false;
    }
  },
  mounted() {
    if (this.$route.params.id != undefined && this.screenStore.width < 768) {
      this.showList = false;

      // this feels so cursed but i don't know how to make it work otherwise
      // it work being when you first visit a page the transition doesn't run
      // I thought this would help but it doesn't https://vuejs.org/guide/built-ins/transition#transition-on-appear
      if (this.firstLoad) {
        this.showDetail = true;
      }
    }

    this.firstLoad = false;
  },
  async created() {
    const eventStore = useEventStore();
    if (!eventStore.isLoaded) {
      this.loadingEvents = true;
    }

    if (await eventStore.loadEvents(this.showPast)) {
      this.loadingEvents = false;
    }
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

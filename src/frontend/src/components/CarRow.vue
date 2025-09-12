<script lang="ts" setup>
import CaretRight from './icons/CaretRight.vue';
</script>

<template>
  <tr>
    <td>{{ car!.driver.name }}</td>
    <td>{{ car!.riders.length }} / {{ car!.maxCapacity }}</td>
    <td>{{ departureTime }}</td>
    <td>{{ returnTime }}</td>
    <td>
      <CaretRight :class="{ rotated: rotateCaret }" class="caret" />
    </td>
  </tr>
</template>

<script lang="ts">
import { type Car } from '@/models';
import { defineComponent, type PropType } from 'vue';
import { format } from 'date-fns';

export default defineComponent({
  props: {
    car: Object as PropType<Car>,
    rotateCaret: Boolean
  },
  computed: {
    departureTime() {
      let data = this.car!.departureTime.toLocaleString();
      return format(data, 'MM/dd/yyyy hh:mm a');
    },
    returnTime() {
      let data = this.car!.returnTime.toLocaleString();
      return format(data, 'MM/dd/yyyy hh:mm a');
    }
  }
});
</script>

<style scoped>
tr:hover > * {
  background-color: rgb(201, 201, 201);
  transition: background-color 0.35s ease;
}

.caret {
  display: inline-block;
  transition: transform 0.3s ease;
}

.rotated {
  transform: rotate(90deg);
}

tr {
  cursor: pointer;
}
</style>

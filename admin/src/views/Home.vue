<template>
  <div>
    <h2 class="mb-5">Top</h2>

    <h3 class="mb-3">イベント新規登録</h3>
    <EventForm class="mb-5" />

    <div>
      <h3 class="mb-3">直近のイベント</h3>
      <div v-if="loading" class="d-flex align-center justify-center py-10">
        <v-progress-circular
          indeterminate
          color="blue"
        />
      </div>
      <div v-else v-for="event in events" :key="event.id" class="mb-2">
        <EventCard :event="event" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, type Ref } from 'vue'
import axios from 'axios'

import type { Event } from '@/types/event'
import { Calendar } from '@/types/calendar'

import EventForm from '@/components/EventForm.vue'
import EventCard from '@/components/EventCard.vue'

export default defineComponent({
  components: {
    EventForm,
    EventCard,
  },
  setup() {
    const loading = ref(true);
    const calendars = ref([]) as Ref<Array<Calendar>>;
    const events = ref([]) as Ref<Array<Event>>;

    onMounted(async () => {
      loading.value = true;
      try {
        const { data: eventsResponseData } = await axios.get('http://localhost:3000/admin/calendar/events');
        events.value = eventsResponseData;
        const { data: calendarsResponseData } = await axios.get('http://localhost:3000/admin/calendar/details');
        calendars.value = calendarsResponseData;
      } catch (err: any) {
        console.error(err);
      } finally {
        loading.value = false;
      }
    });

    return {
      loading,
      events
    }
  },
})
</script>

<template>
  <div class="event">

    <div class="mb-5">
      <v-btn variant="outlined" color="blue" prepend-icon="mdi-arrow-left" to="/">トップ</v-btn>
    </div>

    <v-card>
      <v-card-text>
        <h2 class="mb-3">{{ event.summary }}</h2>


        <div class="mb-3">
          <div class="mb-1">
            <v-icon icon="mdi-calendar"></v-icon>
            <span>{{ calendarName }}</span>
          </div>
          <div class="mb-1">
            <v-icon icon="mdi-clock"></v-icon>
            <PeriodTime :start-at="event.start_at" :end-at="event.end_at" :is-all-day="event.all_day" />
          </div>
          <div class="mb-1">
            <v-icon icon="mdi-map-marker"/>
            <span>{{ event.location || '(未記入)'}}</span>
          </div>
        </div>

        <div>
          <EventDetail :detail="event.description" />
        </div>
      </v-card-text>
    </v-card>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, computed } from 'vue'
import axios from 'axios'

import type { Event } from '@/types/event'
import type { Calendar } from '@/types/calendar'

import { useCalendarStore } from '@/stores/calendar'

import PeriodTime from '@/components/PeriodTime.vue'
import EventDetail from '@/components/EventDetail.vue'

export default defineComponent({
  components: {
    PeriodTime,
    EventDetail,
  },
  props: {
    id: {
      type: String,
      required: true,
    }
  },
  setup(props) {
    const event = ref<Event>({} as Event);

    const calendars = ref<Array<Calendar>>([]);
    const calendarStore = useCalendarStore();

    onMounted(async () => {
      const response = await axios.get(`/api/calendar/events/${props.id}`);
      event.value = response.data;
      calendars.value = calendarStore.calendars;
    })

    const calendarName = computed(() => {
      const calendar = calendars.value.find((calendar: Calendar) => {
        return calendar.id === event.value.calendar_detail_id
      });
      return calendar ? calendar.name : '-'
    })

    return {
      event,
      calendarName,
    }
  },
})
</script>

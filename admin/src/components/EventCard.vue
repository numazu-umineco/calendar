<template>
  <v-card :to="`/events/${event.id}`">
    <v-card-title class="d-flex align-center">
      <v-chip class="mr-2" size="small">{{ convertCalendarName }}</v-chip>
      <h4>{{ event.summary }}</h4>
    </v-card-title>
    <v-card-text>
      <div class="mb-1">
        <v-icon icon="mdi-calendar" />
        <PeriodTime :start-at="event.start_at" :end-at="event.end_at" :is-all-day="event.all_day"></PeriodTime>
      </div>
      <div class="mb-1">
        <v-icon icon="mdi-map-marker"/> {{ event.location || '(未記入)'}}
      </div>
      <EventDetail :detail="event.description" />
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { defineComponent, computed } from 'vue'

import { useCalendarStore } from '@/stores/calendar'

import type { Calendar } from '@/types/calendar'

import PeriodTime from '@/components/PeriodTime.vue'
import EventDetail from '@/components/EventDetail.vue'

export default defineComponent({
  components: {
    PeriodTime,
    EventDetail,
  },
  props: {
    event: {
      type: Object,
      required: true,
    },
  },
  setup(props) {
    const calendarStore = useCalendarStore();
    const calendars = calendarStore.calendars;

    const convertCalendarName = computed(() => {
      const calendar = calendars.find((calendar: Calendar) => {
        return calendar.id === props.event.calendar_detail_id
      });
      return calendar ? calendar.name : '-'
    })

    const formatLine = (str: string) => {
      return str.replace(/\n/g, '<br />')
    };

    return {
      convertCalendarName,
      formatLine
    }
  },
})
</script>

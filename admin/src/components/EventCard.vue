<template>
  <v-card :to="`/events/${event.id}`">
    <v-card-title class="d-flex align-center">
      <v-chip class="mr-2" size="small">{{ convertCalendarName }}</v-chip>
      <h4>{{ event.summary }}</h4>
    </v-card-title>
    <v-card-text>
      <div class="mb-1">
        <v-icon icon="mdi-calendar" /> {{ formatDateTime }}
      </div>
      <div class="mb-1">
        <v-icon icon="mdi-map-marker"/> {{ event.location || '(未記入)'}}
      </div>
      <div v-html="formatLine(event.description)" />
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { defineComponent, computed } from 'vue'
import dayjs from 'dayjs'
import utc from 'dayjs/plugin/utc'
import timezone from 'dayjs/plugin/timezone'

import { useCalendarStore } from '@/stores/calendar'

import type { Calendar } from '@/types/calendar'

export default defineComponent({
  props: {
    event: {
      type: Object,
      required: true,
    },
  },
  setup(props) {
    dayjs.extend(utc);
    dayjs.extend(timezone);


    const calendarStore = useCalendarStore();
    const calendars = calendarStore.calendars;

    const convertCalendarName = computed(() => {
      const calendar = calendars.find((calendar: Calendar) => {
        return calendar.id === props.event.calendar_detail_id
      });
      return calendar ? calendar.name : '-'
    })

    const formatDateTime = computed(() => {
      const startAtDate = dayjs(props.event.start_at).tz('Asia/Tokyo').format('YYYY-MM-DD')
      const startAtTime = dayjs(props.event.start_at).tz('Asia/Tokyo').format('HH:mm')
      const endAtDate = dayjs(props.event.end_at).tz('Asia/Tokyo').format('YYYY-MM-DD')
      const endAtTime = dayjs(props.event.end_at).tz('Asia/Tokyo').format('HH:mm')

      if (props.event.all_day && startAtDate === endAtDate) {
        return `${startAtDate}`
      }
      if (props.event.all_day) {

        return `${startAtDate} - ${endAtDate}`
      }
      // start_at と end_at が同じ日付の場合は日付の表示をまとめて時間の表示だけをする
      if (startAtDate === endAtDate && startAtTime !== endAtTime) {
        return `${startAtDate} ${startAtTime} - ${endAtTime}`
      }
      return `${startAtDate} ${startAtTime} - ${endAtDate} ${endAtTime}`
    });

    const formatLine = (str: string) => {
      return str.replace(/\n/g, '<br />')
    };

    return {
      formatDateTime,
      convertCalendarName,
      formatLine
    }
  },
})
</script>

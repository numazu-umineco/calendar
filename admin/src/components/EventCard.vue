<template>
  <v-card :to="`/events/${event.id}`">
    <v-card-title class="d-flex align-center">
      <v-chip class="mr-2" size="small">{{ event.calendar_detail_id }}</v-chip>
      <h4>{{ event.summary }}</h4>
    </v-card-title>
    <v-card-text>
      <div class="mb-1">
        <v-icon icon="mdi-calendar" /> {{ startAt }} - {{ endAt }}
      </div>
      <div class="mb-1">
        <v-icon icon="mdi-map-marker"/> {{ event.location || '(未記入)'}}
      </div>
      <div>{{ event.description }}</div>
      <div>{{ event.calendar_detail_id }} </div>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { defineComponent, computed } from 'vue'
import dayjs from 'dayjs'
import utc from 'dayjs/plugin/utc'
import timezone from 'dayjs/plugin/timezone'

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
    const startAt = computed(() => {
      return dayjs.tz(props.event.start_at, 'Asia/Tokyo').format('YYYY-MM-DD HH:mm')
    })
    const endAt = computed(() => {
      return dayjs.tz(props.event.end_at, 'Asia/Tokyo').format('YYYY-MM-DD HH:mm')
    })
    return {
      startAt,
      endAt,
    }
  },
})
</script>

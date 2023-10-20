<template>
  <div class="event">

    <div class="mb-5">
      <v-btn variant="outlined" color="blue" prepend-icon="mdi-arrow-left" to="/">トップ</v-btn>
    </div>

    <EventCard :event="event" class="mb-5"></EventCard>

    <DeleteConfirm :id="id" @deleted="moveTop">
      イベントを削除
    </DeleteConfirm>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, computed } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'

import type { Event } from '@/types/event'
import type { Calendar } from '@/types/calendar'

import { useCalendarStore } from '@/stores/calendar'

import PeriodTime from '@/components/PeriodTime.vue'
import EventDetail from '@/components/EventDetail.vue'
import DeleteConfirm from '@/components/DeleteConfirm.vue'
import EventCard from '@/components/EventCard.vue'

export default defineComponent({
  components: {
    PeriodTime,
    EventDetail,
    DeleteConfirm,
    EventCard,
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

    const router = useRouter();
    const moveTop = () => {
      router.push('/');
    }

    return {
      event,
      calendarName,
      moveTop,
    }
  },
})
</script>

<template>
  <div class="event">

    <div class="mb-5">
      <v-btn variant="outlined" color="blue" prepend-icon="mdi-arrow-left" to="/">トップ</v-btn>
    </div>

    <v-expand-transition>
      <div v-if="currentState === state.VIEWING">
        <EventCard :event="event" class="mb-5" />
      </div>
    </v-expand-transition>

    <v-expand-transition>
      <div v-if="currentState === state.EDITING">
        <EventForm :event="event" @event-submitted="eventUpdated" class="mb-5" />
      </div>
    </v-expand-transition>

    <div class="d-inline-flex">
      <v-btn variant="outlined" @click="currentState = state.EDITING" class="mr-2">編集</v-btn>
      <DeleteConfirm :id="id" @deleted="moveTop">イベントを削除</DeleteConfirm>
    </div>
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
import EventForm from '../components/EventForm.vue'

const state = {
  VIEWING: 'viewing',
  EDITING: 'editing',
}

export default defineComponent({
  components: {
    PeriodTime,
    EventDetail,
    DeleteConfirm,
    EventCard,
    EventForm,
  },
  props: {
    id: {
      type: String,
      required: true,
    }
  },
  setup(props) {
    const currentState = ref(state.VIEWING);
    const event = ref<Event>({} as Event);

    const calendars = ref<Array<Calendar>>([]);
    const calendarStore = useCalendarStore();

    const getEvent = async () => {
      const response = await axios.get(`/api/calendar/events/${props.id}`);
      event.value = response.data;
      calendars.value = calendarStore.calendars;
    }

    onMounted(getEvent);

    const calendarName = computed(() => {
      const calendar = calendars.value.find((calendar: Calendar) => {
        return calendar.id === event.value.calendar_detail_id
      });
      return calendar ? calendar.name : '-'
    });

    const eventUpdated = async () => {
      await getEvent();
      currentState.value = state.VIEWING;
    }

    const router = useRouter();
    const moveTop = () => {
      router.push('/');
    }

    return {
      state,
      currentState,
      event,
      calendarName,
      eventUpdated,
      moveTop,
    }
  },
})
</script>

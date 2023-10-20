import { defineStore } from 'pinia'
import { type Ref, ref } from 'vue'
import axios from 'axios'

import type { Calendar } from '@/types/calendar'

export const useCalendarStore = defineStore('calendar', () => {
  const calendars = ref<Array<Calendar>>([]);

  const fetchData = async () => {
    try {
      const { data } = await axios.get('/api/calendar/details');
      calendars.value = data;
    } catch (error) {
      console.error(error);
      throw error;
    }
  }

  const getData = () => {
    return calendars.value;
  }

  return {
    calendars,
    fetchData,
    getData,
  }
})

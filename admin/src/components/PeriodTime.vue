<template>
  <span>{{ periodTimeFormat }}</span>
</template>

<script lang="ts">
import { computed, defineComponent } from 'vue'
import dayjs from 'dayjs'
import utc from 'dayjs/plugin/utc'
import timezone from 'dayjs/plugin/timezone'

export default defineComponent({
  props: {
    startAt: {
      type: String,
    },
    endAt: {
      type: String,
    },
    isAllDay: {
      type: Boolean,
    }
  },
  setup(props) {
    dayjs.extend(utc);
    dayjs.extend(timezone);

    const periodTimeFormat = computed(() => {
      if (!props.startAt && !props.endAt) return '';
      const startAtDate = dayjs(props.startAt).tz('Asia/Tokyo').format('YYYY-MM-DD')
      const startAtTime = dayjs(props.startAt).tz('Asia/Tokyo').format('HH:mm')
      const endAtDate = dayjs(props.endAt).tz('Asia/Tokyo').format('YYYY-MM-DD')
      const endAtTime = dayjs(props.endAt).tz('Asia/Tokyo').format('HH:mm')

      // startAt と endAt が同じで終日イベントだったら片方の日付だけ表示
      if (props.isAllDay && startAtDate === endAtDate) {
        return `${startAtDate}`
      }
      // startAt と endAt が異なるが終日イベントだったらそれぞれの日付だけ表示
      if (props.isAllDay) {
        return `${startAtDate} - ${endAtDate}`
      }
      // startAt と endAt が同じ日の場合は日付の表示をまとめてそれぞれの時間を表示する
      if (startAtDate === endAtDate && startAtTime !== endAtTime) {
        return `${startAtDate} ${startAtTime} - ${endAtTime}`
      }
      // どちらも異なれば、それぞれの日付と時間を表示
      return `${startAtDate} ${startAtTime} - ${endAtDate} ${endAtTime}`
    })

    return { periodTimeFormat }
  },
})
</script>

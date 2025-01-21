<template>
  <div>
    <v-form @submit.prevent="submit" ref="form">
      <v-card>
        <v-card-text>
          <v-overlay
            v-model="loading"
            contained
            persistent
            class="align-center justify-center"
          >
            <div v-show="!completed">
              <div class="mb-5 text-white text-center">投稿中...</div>
              <div>
                <v-progress-circular
                  indeterminate
                  color="white"
                  size="64"
                ></v-progress-circular>
              </div>
            </div>
            <v-scale-transition>
              <div v-if="completed">
                <v-icon size="x-large" color="white" style="font-size: 120px;">
                  mdi-check-circle
                </v-icon>
              </div>
            </v-scale-transition>
          </v-overlay>
          <v-alert
            v-if="errorMessage"
            type="error"
            dismissible
            class="mb-5"
            >
            {{ errorMessage }}
          </v-alert>
          <v-text-field
            v-model="summary"
            label="イベントタイトル"
            :rules="requiredRules"
            variant="outlined"
            density="compact"
            class="mb-2"
          ></v-text-field>

          <v-select
            v-model="calendarId"
            :items="calendars"
            item-title="name"
            item-value="id"
            label="イベントの種類"
            :rules="requiredRules"
            variant="outlined"
            density="compact"
            class="mb-2"
          />

          <div class="text-medium-emphasis mb-3">
            基本的にイベント日程は「終日」を指定し、時間を記述する場合は「イベント詳細」欄に記述してください。<br />
            ただし、イベントの開催期間が「1時間以下」のものに関してのみ、時間の指定を行うことを検討できます。
          </div>

          <div class="d-flex flex-fill flex-column flex-md-row justify-center align-center mb-2">
            <div class="flex-md-shrink-0 flex-md-grow-0 mr-md-5">
              <v-checkbox
                v-model="isAllDay"
                label="終日"
                color="info"
              ></v-checkbox>
            </div>
            <div class="flex-md-grow-1 mr-md-5">
              <v-text-field
                v-model="startAt"
                :type="dateInputType"
                label="開始"
                variant="outlined"
                density="compact"
              ></v-text-field>
            </div>
            <div class="flex-md-grow-0 mr-md-5">
              <v-text-field
                v-model="startWeekday"
                readonly
                variant="plain"
                density="compact"
              ></v-text-field>
            </div>
            <div class="flex-md-grow-1 mr-md-5">
              <v-text-field
                v-model="endAt"
                :type="dateInputType"
                label="終了"
                variant="outlined"
                density="compact"
              ></v-text-field>
            </div>
            <div class="flex-md-grow-0">
              <v-text-field
                v-model="endWeekday"
                readonly
                variant="plain"
                density="compact"
              ></v-text-field>
            </div>
          </div>

          <v-textarea
            v-model="description"
            label="イベントの詳細"
            :rules="requiredRules"
            variant="outlined"
            density="compact"
            class="mb-2"
          ></v-textarea>

          <v-text-field
            v-model="url"
            label="情報元URL (公式サイト・Twitterなど)"
            variant="outlined"
            density="compact"
            class="mb-2"
          ></v-text-field>

          <v-text-field
            v-model="location"
            label="場所 (施設名・住所)"
            :rules="requiredRules"
            variant="outlined"
            density="compact"
            class="mb-2"
          ></v-text-field>

          <div class="d-flex">
            <div class="ml-auto">
              <v-btn color="info" variant="outlined" @click="clear">クリア</v-btn>
            </div>
            <div class="ml-2">
              <v-btn type="submit" color="info" variant="flat">カレンダー登録</v-btn>
            </div>
          </div>
        </v-card-text>
      </v-card>
    </v-form>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed, onMounted, watch, PropType, onUnmounted } from 'vue'
import { useRouter } from 'vue-router';
import dayjs from 'dayjs'
import utc from 'dayjs/plugin/utc'
import timezone from 'dayjs/plugin/timezone'
import axios from 'axios'

import type { Calendar } from '@/types/calendar'
import type { Event } from '@/types/event'
import type { VForm } from '@/types/vuetify'

export default defineComponent({
  props: {
    event: {
      type: Object as PropType<Event>,
      default: {} as Event,
    }
  },
  setup(props, { emit }) {
    dayjs.extend(utc);
    dayjs.extend(timezone);
    dayjs.tz.setDefault('Asia/Tokyo');

    const router = useRouter();

    const loading = ref(false);
    const completed = ref(false);
    const errorMessage = ref('');
    const eventId = ref<number|null>(null);
    const summary = ref('');
    const description = ref('');
    const url = ref('');
    const location = ref('');
    const isAllDay = ref(true);
    const startAtRaw = ref<dayjs.Dayjs>(dayjs().startOf('day'));
    const endAtRaw = ref<dayjs.Dayjs>(dayjs().startOf('day'));
    const calendars = ref<Array<Calendar>>([]);
    const calendarId = ref<string|null>(null);

    const updateEvent = (event: Event) => {
      eventId.value = event.id;
      summary.value = event.summary;
      calendarId.value = event.calendar_detail_id;
      description.value = event.description;
      location.value = event.location;
      isAllDay.value = event.all_day;
      startAtRaw.value = dayjs(event.start_at);
      endAtRaw.value = dayjs(event.end_at);
    }

    onMounted(async () => {
      if (props.event.id) {
        updateEvent(props.event);
      }
    });

    watch(() => props.event, (newEvent) => {
      if (newEvent.id) {
        updateEvent(newEvent);1
      }
    }, { immediate: true });

    const startAt = computed({
      get: () => {
        if (isAllDay.value) {
          return startAtRaw.value.hour(0).minute(0).format('YYYY-MM-DD');
        } else {
          return startAtRaw.value.format('YYYY-MM-DDTHH:mm');
        }
      },
      set: (value) => {
        if (isAllDay.value) {
          startAtRaw.value = dayjs(value).startOf('day');
        } else {
          startAtRaw.value = dayjs(value);
        }
      }
    });

    const endAt = computed({
      get: () => {
        if (isAllDay.value) {
          return endAtRaw.value.startOf('day').format('YYYY-MM-DD');
        } else {
          return endAtRaw.value.format('YYYY-MM-DDTHH:mm');
        }
      },
      set: (value) => {
        if (isAllDay.value) {
          endAtRaw.value = dayjs(value).startOf('day');
        } else {
          endAtRaw.value = dayjs(value);
        }
      }
    });

    watch(startAtRaw, () => {
      if (startAtRaw.value.isAfter(endAtRaw.value)) {
        endAtRaw.value = startAtRaw.value;
      }
    });

    const requiredRules = ref([
      (v: string) => !!v || '必須項目です',
    ])

    const weekday = ['日', '月', '火', '水', '木', '金', '土']

    const startWeekday = computed(() => `(${weekday[startAtRaw.value.day()]})`);
    const endWeekday = computed(() => `(${weekday[endAtRaw.value.day()]})`);

    const dateInputType = computed(() => isAllDay.value ? 'date' : 'datetime-local');

    onMounted(async () => {
      try {
        const response = await axios.get('/api/calendar/details');
        calendars.value = response.data as Array<Calendar>;
      } catch (err: any) {
        console.error(err);
      }
    });

    const clear = () => {
      loading.value = false;
      eventId.value = null;
      summary.value = '';
      calendarId.value = null;
      description.value = '';
      url.value = '';
      location.value = '';
      isAllDay.value = true;
      startAtRaw.value = dayjs().hour(0).minute(0);
      endAtRaw.value = dayjs().hour(0).minute(0);
    };

    onUnmounted(clear);

    const eventDescription = computed(() => {
      if (url.value === '') {
        return description.value;
      }
      return `${description.value}\n\n[Source] ${url.value}`;
    });

    const form = ref({} as VForm);

    const submit = async () => {
      if (!form.value) return;
      const { valid } = await form.value.validate();
      if (!valid) return;

      completed.value = false;
      loading.value = true;
      errorMessage.value = '';
      let requestPath = '/api/calendar/events';
      let createdEvent: Event;
      if (eventId.value) {
        requestPath = `/api/calendar/events/${eventId.value}`;
      }
      try {
        const response = await axios({
          url: requestPath,
          method: eventId.value ? 'put' : 'post',
          data: {
            calendar_event: {
              calendar_id: calendarId.value,
              summary: summary.value,
              description: eventDescription.value,
              location: location.value,
              all_day: isAllDay.value,
              start_at: startAtRaw.value.format(),
              end_at: endAtRaw.value.format(),
            }
          },
          headers: { 'x-user-name': 'admin' }
        });
        createdEvent = response.data as Event;
      } catch (err: any) {
        console.error(err);
        await setTimeout(() => {
          loading.value = false;
          errorMessage.value = err.message;
        }, 1000);
        return;
      }

      await new Promise<void>((resolve) => {
        setTimeout(() => {
          completed.value = true;
          resolve();
        }, 500);
      });
      await new Promise<void> ((resolve) => {
        setTimeout(() => {
          loading.value = false;
          router.push(`/events/${createdEvent.id}`);
          emit('eventSubmitted', createdEvent);
          resolve();
        }, 500);
      });
    };

    return {
      loading,
      completed,
      errorMessage,
      calendars,
      calendarId,
      summary,
      description,
      url,
      location,
      isAllDay,
      startAt,
      endAt,
      form,
      startAtRaw,
      endAtRaw,
      startWeekday,
      endWeekday,
      dateInputType,
      requiredRules,
      submit,
      clear,
    }
  },
})
</script>

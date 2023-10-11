<template>
  <div>
    <form @submit.prevent="submit">
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
          <h3 class="mb-5">イベント新規登録</h3>
          <v-text-field
            v-model="summary"
            label="イベントタイトル"
            variant="outlined"
            density="compact"
            class="mb-2"
          ></v-text-field>

          <v-textarea
            v-model="description"
            label="イベントの詳細"
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
            variant="outlined"
            density="compact"
            class="mb-2"
          ></v-text-field>

          <div class="text-medium-emphasis mb-3">
            基本的にイベント日程は「終日」を指定し、時間を記述する場合は「イベント詳細」欄に記述してください。<br />
            ただし、イベントの開催期間が「1時間以下」のものに関しては、時間の指定を行うことを検討できます。
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
            <div class="flex-md-grow-1">
              <v-text-field
                v-model="endAt"
                :type="dateInputType"
                label="終了"
                variant="outlined"
                density="compact"
              ></v-text-field>
            </div>
          </div>

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
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed } from 'vue'
import dayjs from 'dayjs'

export default defineComponent({
  setup() {
    const loading = ref(false);
    const completed = ref(false);
    const summary = ref('');
    const description = ref('');
    const url = ref('');
    const location = ref('');
    const isAllDay = ref(true);
    const startAtRaw = ref(dayjs().hour(0).minute(0));
    const endAtRaw = ref(dayjs().hour(0).minute(0));

    const startAt = computed({
      get: () => {
        if (isAllDay.value) {
          return startAtRaw.value.hour(0).minute(0).format('YYYY-MM-DD');
        } else {
          return startAtRaw.value.format('YYYY-MM-DDTHH:mm');
        }
      },
      set: (value) => {
        startAtRaw.value = dayjs(value);
        console.log(startAtRaw.value.format('YYYY-MM-DDTHH:mm'))
      }
    });

    const endAt = computed({
      get: () => {
        if (isAllDay.value) {
          return endAtRaw.value.hour(0).minute(0).format('YYYY-MM-DD');
        } else {
          return endAtRaw.value.format('YYYY-MM-DDTHH:mm');
        }
      },
      set: (value) => {
        endAtRaw.value = dayjs(value);
      }
    });

    const dateInputType = computed(() => isAllDay.value ? 'date' : 'datetime-local');

    const clear = () => {
      loading.value = false;
      summary.value = '';
      description.value = '';
      url.value = '';
      location.value = '';
      isAllDay.value = true;
      startAtRaw.value = dayjs().hour(0).minute(0);
      endAtRaw.value = dayjs().hour(0).minute(0);
    };

    const submit = () => {
      completed.value = false;
      console.log(summary.value);
      console.log(description.value);
      console.log(url.value);
      console.log(location.value);
      console.log(isAllDay.value);
      console.log(startAtRaw.value);
      console.log(endAtRaw.value);
      loading.value = true;
      setTimeout(() => {
        completed.value = true;
      }, 1000);
      setTimeout(() => {
        loading.value = false;
        clear();
      }, 1500);
    };


    return {
      loading,
      completed,
      summary,
      description,
      url,
      location,
      isAllDay,
      startAt,
      endAt,
      dateInputType,
      submit,
      clear
    }
  },
})
</script>

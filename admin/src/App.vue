<template>
  <v-app>
    <v-app-bar color="blue">
      <v-container>
        <v-row>
          <v-col>
            <v-app-bar-title class="font-weight-bold">Umineco Calendar System</v-app-bar-title>
          </v-col>
        </v-row>
      </v-container>
    </v-app-bar>

    <v-container class="content">
      <v-row>
        <v-col>
          <router-view />
        </v-col>
      </v-row>
    </v-container>
  </v-app>
</template>

<script lang="ts">
import { defineComponent, onMounted } from 'vue'
import { useCalendarStore } from '@/stores/calendar'

export default defineComponent({
  setup() {
    const calendarStore = useCalendarStore();

    const fetchData = async () => {
      await calendarStore.fetchData();
    };

    onMounted(async () => {
      await fetchData();
    });
  },
})
</script>


<style scoped>
.content {
  margin-top: 64px;
}
</style>

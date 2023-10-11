<template>
  <div>
    <h1 class="mb-5">Home</h1>

    <EventForm class="mb-5" />

    <div class="mb-5">
      <p><router-link to="/events/1">Event detail (id: 1) page link sample</router-link></p>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import EventForm from '@/components/EventForm.vue'

export default defineComponent({
  components: {
    EventForm,
  },
  setup() {

  },
})
</script>

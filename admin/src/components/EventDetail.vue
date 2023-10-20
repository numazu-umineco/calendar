<template>
  <div>
    <div v-html="formatLine(detail)"></div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    detail: {
      type: String,
    },
  },
  setup() {
    const formatLine = (str: string | undefined) => {
      if (!str) return ''
      return str.replace(/\n/g, '<br />')
    }
    return { formatLine }
  },
})
</script>

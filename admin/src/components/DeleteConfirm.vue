<template>
  <div>
    <v-btn color="error" @click="openDialog">
      <slot />
    </v-btn>

    <v-dialog v-model="dialog" width="500">
      <v-card class="py-5">
        <div v-if="confirmMode">
          <v-card-text>
            <div v-if="errorMessage" class="mb-4">
              <v-alert type="error" :text="errorMessage" />
            </div>
            <div class="py-5 text-center">
              イベントを削除しますか？
            </div>
            <div class="d-flex justify-center mt-5">
              <v-btn variant="outlined" class="mr-2 px-7" @click="dialog = false">キャンセル</v-btn>
              <v-btn color="error" variant="flat" class="px-10" @click="submit">削除する</v-btn>
            </div>
          </v-card-text>
        </div>
        <div v-else>
          <v-card-text>
            <div class="d-flex justify-center">
              <v-progress-circular
                v-if="loading"
                color="blue"
                indeterminate
                size="40"
              />
              <v-scale-transition>
                <div v-if="!loading">
                  <v-icon size="x-large" color="green" style="font-size: 120px;">
                    mdi-check-circle
                  </v-icon>
                </div>
              </v-scale-transition>
            </div>
          </v-card-text>
        </div>
      </v-card>
    </v-dialog>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import axios from 'axios'

export default defineComponent({
  props: {
    id: {
      type: String,
      required: true,
    },
  },
  setup(props, { emit }) {
    const dialog = ref(false)

    const confirmMode = ref(true)
    const errorMessage = ref('')
    const loading = ref(false)

    const openDialog = () => {
      dialog.value = true
      confirmMode.value = true
      loading.value = false
      errorMessage.value = ''
    }

    const submit = async () => {
      confirmMode.value = false
      loading.value = true

      try {
        await axios.delete(`/api/calendar/events/${props.id}`)
      } catch (err: any) {
        console.error(err)
        await new Promise<void>((resolve) => {
          setTimeout(() => {
            errorMessage.value = err.message
            confirmMode.value = true
            loading.value = false
            resolve()
          }, 1000);
        });
        return;
      }

      await new Promise<void>((resolve) => {
        setTimeout(() => {
          loading.value = false
          resolve()
        }, 1000);
      });
      await new Promise<void>((resolve) => {
        setTimeout(() => {
          dialog.value = false
          emit('deleted')
          resolve()
        }, 1000);
      });
    }

    return {
      dialog,
      confirmMode,
      errorMessage,
      loading,
      openDialog,
      submit
    }
  },
})
</script>

import { createApp } from 'vue'

import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { mdi } from 'vuetify/iconsets/mdi'

import App from '@/App.vue'
import router from '@/router'
import '@mdi/font/css/materialdesignicons.css'
// import './style.css'

const app = createApp(App)

app.provide('$route', router.currentRoute)
app.provide('$router', router)

app.use(router)

// Vuetify
const vuetify = createVuetify({
  components,
  directives,
  icons: {
    defaultSet: 'mdi',
    sets: { mdi }
  }
})

app.use(vuetify)

app.mount('#app')

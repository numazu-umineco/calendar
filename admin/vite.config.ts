import path from 'path'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import Vuetify from '@vuetify/vite-plugin'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src')
    }
  }
})

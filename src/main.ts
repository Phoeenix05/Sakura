import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')

// there is a quick white flash without this
import { appWindow } from '@tauri-apps/api/window'
setTimeout(async () => await appWindow.show(), 50)

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  ssr: false,
  modules: [
    ['@pinia/nuxt', { autoImports: ['defineStore', 'acceptHMRUpdate'] }],
    '@nuxtjs/tailwindcss',
    '@nuxtjs/i18n',
  ],
  imports: {
    dirs: ['stores']
  },
  srcDir: './src'
})

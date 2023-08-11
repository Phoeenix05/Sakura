// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  ssr: false,
  // modules: ['@nuxthq/ui', '@pinia/nuxt'],
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

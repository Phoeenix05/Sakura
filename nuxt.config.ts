// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  ssr: false,
  // modules: ['@nuxthq/ui', '@pinia/nuxt'],
  modules: [
    [
      '@nuxtjs/i18n',
      { autoImports: ['defineStore', 'acceptHMRUpdate'] }
    ],
    '@nuxtjs/tailwindcss',
    '@pinia/nuxt'
  ],
  imports: {
    dirs: ['stores']
  },
  srcDir: './src'
})
